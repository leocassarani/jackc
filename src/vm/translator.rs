use super::command::*;
use crate::asm;
use crate::asm::Instruction;

pub struct Translator<'a> {
    cmds: &'a [Command],
    count: usize,
}

impl<'a> Translator<'a> {
    pub fn new(cmds: &'a [Command]) -> Self {
        Translator { cmds, count: 0 }
    }

    pub fn translate(&mut self) -> Vec<Instruction> {
        let mut prog = vec![
            asm!(@"START"),
            asm!(0;JMP),
            asm!(("EQ")),
            asm!(@"R14"),
            asm!(M = D),
            asm!(@"SP"),
            asm!(AM = M - 1),
            asm!(D = M),
            asm!(A = A - 1),
            asm!(D = M - D),
            asm!(M = 0),
            asm!(@"END_EQ"),
            asm!(D;JNE),
            asm!(@"SP"),
            asm!(A = M - 1),
            asm!(M = -1),
            asm!(("END_EQ")),
            asm!(@"R14"),
            asm!(A = M),
            asm!(0;JMP),
            asm!(("LT")),
            asm!(@"R15"),
            asm!(M = D),
            asm!(@"SP"),
            asm!(AM = M - 1),
            asm!(D = M),
            asm!(A = A - 1),
            asm!(D = M - D),
            asm!(M = 0),
            asm!(@"END_LT"),
            asm!(D;JGE),
            asm!(@"SP"),
            asm!(A = M - 1),
            asm!(M = -1),
            asm!(("END_LT")),
            asm!(@"R15"),
            asm!(A = M),
            asm!(0;JMP),
            asm!(("GT")),
            asm!(@"R15"),
            asm!(M = D),
            asm!(@"SP"),
            asm!(AM = M - 1),
            asm!(D = M),
            asm!(A = A - 1),
            asm!(D = M - D),
            asm!(M = 0),
            asm!(@"END_GT"),
            asm!(D;JLE),
            asm!(@"SP"),
            asm!(A = M - 1),
            asm!(M = -1),
            asm!(("END_GT")),
            asm!(@"R15"),
            asm!(A = M),
            asm!(0;JMP),
            asm!(("START")),
        ];
        prog.extend(self.cmds.iter().flat_map(|cmd| self.translate_cmd(cmd)));
        prog
    }

    fn translate_cmd(&mut self, cmd: &Command) -> Vec<Instruction> {
        match cmd {
            Command::Add => self.translate_add(),
            Command::Sub => self.translate_sub(),
            Command::Neg => self.translate_neg(),
            Command::And => self.translate_and(),
            Command::Or => self.translate_or(),
            Command::Not => self.translate_not(),
            Command::Eq => self.translate_eq(),
            Command::Gt => self.translate_gt(),
            Command::Lt => self.translate_lt(),
            Command::Pop(Segment::Pointer, 0) => self.translate_pop(&[asm!(@"THIS")]),
            Command::Pop(Segment::Pointer, 1) => self.translate_pop(&[asm!(@"THAT")]),
            Command::Pop(Segment::Temp, idx) => {
                let temp = match idx {
                    0 => asm!(@"R5"),
                    1 => asm!(@"R6"),
                    2 => asm!(@"R7"),
                    3 => asm!(@"R8"),
                    4 => asm!(@"R9"),
                    5 => asm!(@"R10"),
                    6 => asm!(@"R11"),
                    7 => asm!(@"R12"),
                    _ => panic!("invalid temp segment offset: {}", idx),
                };
                self.translate_pop(&[temp])
            }
            Command::Pop(segment, idx) => {
                let register = match segment {
                    Segment::Argument => asm!(@"ARG"),
                    Segment::Local => asm!(@"LCL"),
                    Segment::This => asm!(@"THIS"),
                    Segment::That => asm!(@"THAT"),
                    _ => panic!("unexpected segment: {}", segment),
                };

                match idx {
                    0 => self.translate_pop(&[register, asm!(A = M)]),
                    1..=6 => {
                        let mut write = vec![register, asm!(A = M + 1)];
                        for _ in 1..*idx {
                            write.push(asm!(A = A + 1));
                        }
                        self.translate_pop(&write)
                    }
                    _ => {
                        let mut instr = vec![
                            register,
                            asm!(D = M),
                            asm!(@*idx),
                            asm!(D = D + A),
                            asm!(@"R13"),
                            asm!(M = D),
                        ];
                        instr.extend(self.translate_pop(&[asm!(@"R13"), asm!(A = M)]));
                        instr
                    }
                }
            }
            Command::Push(Segment::Constant, n) => self.translate_push(&[asm!(@*n), asm!(D = A)]),
            Command::Push(Segment::Pointer, 0) => {
                self.translate_push(&[asm!(@"THIS"), asm!(D = M)])
            }
            Command::Push(Segment::Pointer, 1) => {
                self.translate_push(&[asm!(@"THAT"), asm!(D = M)])
            }
            Command::Push(Segment::Temp, idx) => {
                let temp = match idx {
                    0 => asm!(@"R5"),
                    1 => asm!(@"R6"),
                    2 => asm!(@"R7"),
                    3 => asm!(@"R8"),
                    4 => asm!(@"R9"),
                    5 => asm!(@"R10"),
                    6 => asm!(@"R11"),
                    7 => asm!(@"R12"),
                    _ => panic!("invalid temp segment offset: {}", idx),
                };
                self.translate_push(&[temp, asm!(D = M)])
            }
            Command::Push(segment, idx) => {
                let register = match segment {
                    Segment::Argument => asm!(@"ARG"),
                    Segment::Local => asm!(@"LCL"),
                    Segment::This => asm!(@"THIS"),
                    Segment::That => asm!(@"THAT"),
                    _ => panic!("unexpected segment: {}", segment),
                };

                match idx {
                    0 => self.translate_push(&[register, asm!(A = M), asm!(D = M)]),
                    1 => self.translate_push(&[register, asm!(A = M + 1), asm!(D = M)]),
                    2 => self.translate_push(&[
                        register,
                        asm!(A = M + 1),
                        asm!(A = A + 1),
                        asm!(D = M),
                    ]),
                    _ => self.translate_push(&[
                        register,
                        asm!(D = M),
                        asm!(@*idx),
                        asm!(A = D + A),
                        asm!(D = M),
                    ]),
                }
            }

            _ => unimplemented!(),
        }
    }

    fn translate_add(&self) -> Vec<Instruction> {
        vec![
            asm!(@"SP"),
            asm!(AM = M - 1),
            asm!(D = M),
            asm!(A = A - 1),
            asm!(M = D + M),
        ]
    }

    fn translate_sub(&self) -> Vec<Instruction> {
        vec![
            asm!(@"SP"),
            asm!(AM = M - 1),
            asm!(D = M),
            asm!(A = A - 1),
            asm!(M = M - D),
        ]
    }

    fn translate_neg(&self) -> Vec<Instruction> {
        vec![asm!(@"SP"), asm!(A = M - 1), asm!(M = -M)]
    }

    fn translate_and(&self) -> Vec<Instruction> {
        vec![
            asm!(@"SP"),
            asm!(AM = M - 1),
            asm!(D = M),
            asm!(A = A - 1),
            asm!(M = D & M),
        ]
    }

    fn translate_or(&self) -> Vec<Instruction> {
        vec![
            asm!(@"SP"),
            asm!(AM = M - 1),
            asm!(D = M),
            asm!(A = A - 1),
            asm!(M = D | M),
        ]
    }

    fn translate_not(&self) -> Vec<Instruction> {
        vec![asm!(@"SP"), asm!(A = M - 1), asm!(M = !M)]
    }

    fn translate_eq(&mut self) -> Vec<Instruction> {
        let label = format!("RET_ADDRESS_{}", self.count);
        self.count += 1;

        vec![
            asm!(@label.clone()),
            asm!(D = A),
            asm!(@"EQ"),
            asm!(0;JMP),
            asm!((label)),
        ]
    }

    fn translate_gt(&mut self) -> Vec<Instruction> {
        let label = format!("RET_ADDRESS_{}", self.count);
        self.count += 1;

        vec![
            asm!(@label.clone()),
            asm!(D = A),
            asm!(@"GT"),
            asm!(0;JMP),
            asm!((label)),
        ]
    }

    fn translate_lt(&mut self) -> Vec<Instruction> {
        let label = format!("RET_ADDRESS_{}", self.count);
        self.count += 1;

        vec![
            asm!(@label.clone()),
            asm!(D = A),
            asm!(@"LT"),
            asm!(0;JMP),
            asm!((label)),
        ]
    }

    fn translate_pop(&self, write: &[Instruction]) -> Vec<Instruction> {
        let mut instr = vec![asm!(@"SP"), asm!(AM = M - 1), asm!(D = M)];
        instr.extend_from_slice(write);
        instr.push(asm!(M = D));
        instr
    }

    fn translate_push(&self, load: &[Instruction]) -> Vec<Instruction> {
        let push = &[asm!(@"SP"), asm!(AM = M + 1), asm!(A = A - 1), asm!(M = D)];
        [load, push].concat()
    }
}
