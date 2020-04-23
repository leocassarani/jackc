use super::command::*;
use crate::asm;
use crate::hack::Instruction;

pub struct Translator<'a> {
    cmds: &'a [Command],
}

impl<'a> Translator<'a> {
    pub fn new(cmds: &'a [Command]) -> Self {
        Translator { cmds }
    }

    pub fn translate(&self) -> Vec<u16> {
        self.cmds
            .iter()
            .flat_map(|cmd| {
                self.translate_cmd(cmd)
                    .into_iter()
                    .map(|instr| instr.to_u16().unwrap())
            })
            .collect()
    }

    fn translate_cmd(&self, cmd: &Command) -> Vec<Instruction> {
        match cmd {
            Command::Add => self.translate_add(),
            Command::Sub => self.translate_sub(),
            Command::Pop(Segment::Pointer, 0) => self.translate_pop_pointer("THIS".to_owned()),
            Command::Pop(Segment::Pointer, 1) => self.translate_pop_pointer("THAT".to_owned()),
            Command::Pop(Segment::Temp, idx) => self.translate_pop_temp(*idx),
            Command::Pop(segment, idx) => self.translate_pop(*segment, *idx),
            Command::Push(Segment::Constant, n) => self.translate_push_const(*n),
            Command::Push(Segment::Pointer, 0) => self.translate_push_pointer("THIS".to_owned()),
            Command::Push(Segment::Pointer, 1) => self.translate_push_pointer("THAT".to_owned()),
            Command::Push(Segment::Temp, idx) => self.translate_push_temp(*idx),
            Command::Push(segment, idx) => self.translate_push(*segment, *idx),
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

    fn translate_pop_pointer(&self, segment: String) -> Vec<Instruction> {
        vec![
            asm!(@"SP"),
            asm!(AM = M - 1),
            asm!(D = M),
            asm!(@segment),
            asm!(M = D),
        ]
    }

    fn translate_pop_temp(&self, idx: u16) -> Vec<Instruction> {
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

        vec![
            asm!(@"SP"),
            asm!(AM = M - 1),
            asm!(D = M),
            temp,
            asm!(M = D),
        ]
    }

    fn translate_pop(&self, segment: Segment, idx: u16) -> Vec<Instruction> {
        let register = match segment {
            Segment::Argument => asm!(@"ARG"),
            Segment::Local => asm!(@"LCL"),
            Segment::This => asm!(@"THIS"),
            Segment::That => asm!(@"THAT"),
            _ => panic!("unexpected segment: {}", segment),
        };

        let mut instr = vec![asm!(@"SP"), asm!(AM = M - 1), asm!(D = M)];

        match idx {
            0 => {
                instr.extend(vec![register, asm!(A = M), asm!(M = D)]);
            }
            1..=6 => {
                instr.extend(vec![register, asm!(A = M + 1)]);

                for _ in 1..idx {
                    instr.push(asm!(A = A + 1));
                }

                instr.push(asm!(M = D));
            }
            _ => {
                instr.splice(
                    0..0,
                    vec![
                        register,
                        asm!(D = M),
                        asm!(@idx),
                        asm!(D = D + A),
                        asm!(@"R13"),
                        asm!(M = D),
                    ],
                );

                instr.extend(vec![asm!(@"R13"), asm!(A = M), asm!(M = D)]);
            }
        };

        instr
    }

    fn translate_push_const(&self, n: u16) -> Vec<Instruction> {
        vec![
            asm!(@n),
            asm!(D = A),
            asm!(@"SP"),
            asm!(AM = M + 1),
            asm!(A = A - 1),
            asm!(M = D),
        ]
    }

    fn translate_push_pointer(&self, segment: String) -> Vec<Instruction> {
        vec![
            asm!(@segment),
            asm!(D = M),
            asm!(@"SP"),
            asm!(AM = M + 1),
            asm!(A = A - 1),
            asm!(M = D),
        ]
    }

    fn translate_push_temp(&self, idx: u16) -> Vec<Instruction> {
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

        vec![
            temp,
            asm!(D = M),
            asm!(@"SP"),
            asm!(AM = M + 1),
            asm!(A = A - 1),
            asm!(M = D),
        ]
    }

    fn translate_push(&self, segment: Segment, idx: u16) -> Vec<Instruction> {
        let register = match segment {
            Segment::Argument => asm!(@"ARG"),
            Segment::Local => asm!(@"LCL"),
            Segment::This => asm!(@"THIS"),
            Segment::That => asm!(@"THAT"),
            _ => panic!("unexpected segment: {}", segment),
        };

        let mut instr = match idx {
            0 => vec![register, asm!(A = M), asm!(D = M)],
            1 => vec![register, asm!(A = M + 1), asm!(D = M)],
            2 => vec![register, asm!(A = M + 1), asm!(A = A + 1), asm!(D = M)],
            _ => vec![
                register,
                asm!(D = M),
                asm!(@idx),
                asm!(A = D + A),
                asm!(D = M),
            ],
        };

        instr.extend(vec![
            asm!(@"SP"),
            asm!(AM = M + 1),
            asm!(A = A - 1),
            asm!(M = D),
        ]);

        instr
    }
}
