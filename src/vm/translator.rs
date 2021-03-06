use super::*;
use crate::asm;
use crate::asm::Instruction;
use crate::labels::Labeller;
use anyhow::{anyhow, Error};

const DEFAULT_INIT: &str = "Sys.init";

type Result<T> = std::result::Result<T, Error>;

pub struct Translator<'a> {
    modules: &'a [Module],
    init: Option<String>,
    labeller: Labeller,
    function: Option<&'a str>,
}

impl<'a> Translator<'a> {
    pub fn new(modules: &'a [Module]) -> Self {
        Translator {
            modules,
            init: Some(DEFAULT_INIT.to_owned()),
            labeller: Labeller::new(),
            function: None,
        }
    }

    pub fn init(&mut self, init: Option<String>) {
        self.init = init;
    }

    pub fn translate(&mut self) -> Result<Vec<Instruction>> {
        let mut prog = vec![
            asm!(@256),
            asm!(D = A),
            asm!(@"SP"),
            asm!(M = D),
            asm!(@"START"),
            asm!(0;JMP),
        ];

        prog.extend(eq());
        prog.extend(lt());
        prog.extend(gt());
        prog.extend(ret());
        prog.extend(call());
        prog.push(asm!(("START")));

        if let Some(func) = &self.init {
            let func = func.clone();
            prog.extend(self.translate_call(func, 0));
        }

        for module in self.modules {
            for cmd in &module.cmds {
                prog.extend(self.translate_cmd(module, cmd)?);
            }
        }

        Ok(prog)
    }

    fn translate_cmd(&mut self, module: &Module, cmd: &'a Command) -> Result<Vec<Instruction>> {
        let instr = match cmd {
            Command::Add => self.translate_binary_op(asm!(M = D + M)),
            Command::Sub => self.translate_binary_op(asm!(M = M - D)),
            Command::Neg => self.translate_unary_op(asm!(M = -M)),
            Command::And => self.translate_binary_op(asm!(M = D & M)),
            Command::Or => self.translate_binary_op(asm!(M = D | M)),
            Command::Not => self.translate_unary_op(asm!(M = !M)),
            Command::Eq | Command::Gt | Command::Lt => self.translate_comparison(cmd)?,
            Command::Pop(Segment::Pointer, 0) => self.translate_pop(&[asm!(@"THIS")]),
            Command::Pop(Segment::Pointer, 1) => self.translate_pop(&[asm!(@"THAT")]),
            Command::Pop(Segment::Temp, idx) => self.translate_pop(&[temp_register(*idx)?]),
            Command::Pop(Segment::Static, idx) => self.translate_pop(&[static_addr(module, *idx)]),
            Command::Pop(segment, idx) => {
                let register = segment_register(*segment)?;

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
            Command::Push(Segment::Constant, n) => self.translate_push_const(*n),
            Command::Push(Segment::Pointer, 0) => {
                self.translate_push(&[asm!(@"THIS"), asm!(D = M)])
            }
            Command::Push(Segment::Pointer, 1) => {
                self.translate_push(&[asm!(@"THAT"), asm!(D = M)])
            }
            Command::Push(Segment::Temp, idx) => {
                self.translate_push(&[temp_register(*idx)?, asm!(D = M)])
            }
            Command::Push(Segment::Static, idx) => {
                self.translate_push(&[static_addr(module, *idx), asm!(D = M)])
            }
            Command::Push(segment, idx) => {
                let register = segment_register(*segment)?;

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
            Command::Label(label) => self.translate_label(label),
            Command::Goto(label) => self.translate_goto(label),
            Command::IfGoto(label) => self.translate_if_goto(label),
            Command::Function(func, locals) => self.translate_function(func, *locals),
            Command::Call(func, args) => self.translate_call(func.to_owned(), *args),
            Command::Return => vec![asm!(@"RETURN"), asm!(0;JMP)],
        };

        Ok(instr)
    }

    fn translate_unary_op(&self, op: Instruction) -> Vec<Instruction> {
        vec![asm!(@"SP"), asm!(A = M - 1), op]
    }

    fn translate_binary_op(&self, op: Instruction) -> Vec<Instruction> {
        vec![
            asm!(@"SP"),
            asm!(AM = M - 1),
            asm!(D = M),
            asm!(A = A - 1),
            op,
        ]
    }

    fn translate_comparison(&mut self, cmd: &Command) -> Result<Vec<Instruction>> {
        let (label, op) = match cmd {
            Command::Eq => Ok((self.labeller.generate("RET_ADDRESS_EQ"), asm!(@"EQ"))),
            Command::Gt => Ok((self.labeller.generate("RET_ADDRESS_GT"), asm!(@"GT"))),
            Command::Lt => Ok((self.labeller.generate("RET_ADDRESS_LT"), asm!(@"LT"))),
            _ => Err(anyhow!("unexpected comparison command `{}`", cmd)),
        }?;

        Ok(vec![
            asm!(@label.clone()),
            asm!(D = A),
            op,
            asm!(0;JMP),
            asm!((label)),
        ])
    }

    fn translate_pop(&self, write: &[Instruction]) -> Vec<Instruction> {
        let mut instr = vec![asm!(@"SP"), asm!(AM = M - 1), asm!(D = M)];
        instr.extend_from_slice(write);
        instr.push(asm!(M = D));
        instr
    }

    fn translate_push_const(&self, n: u16) -> Vec<Instruction> {
        match n {
            0 => vec![asm!(@"SP"), asm!(AM = M + 1), asm!(A = A - 1), asm!(M = 0)],
            1 => vec![asm!(@"SP"), asm!(AM = M + 1), asm!(A = A - 1), asm!(M = 1)],
            2 => vec![
                asm!(@"SP"),
                asm!(AM = M + 1),
                asm!(A = A - 1),
                asm!(D = 1),
                asm!(M = D + 1),
            ],
            _ if n >> 15 == 0 => self.translate_push(&[asm!(@n), asm!(D = A)]),
            _ => {
                // If the MSB of the constant value is high, then it won't fit in an A-instruction.
                // Instead, we flip all the bits of the immediate value to guarantee that it will
                // have a low MSB, then flip them again to load it into D.
                self.translate_push(&[asm!(@!n), asm!(D = !A)])
            }
        }
    }

    fn translate_push(&self, load: &[Instruction]) -> Vec<Instruction> {
        let push = &[asm!(@"SP"), asm!(AM = M + 1), asm!(A = A - 1), asm!(M = D)];
        [load, push].concat()
    }

    fn translate_label(&self, label: &str) -> Vec<Instruction> {
        let full_label = func_label_name(self.function, label);
        vec![asm!((full_label))]
    }

    fn translate_goto(&self, label: &str) -> Vec<Instruction> {
        let full_label = func_label_name(self.function, label);
        vec![asm!(@full_label), asm!(0;JMP)]
    }

    fn translate_if_goto(&self, label: &str) -> Vec<Instruction> {
        let full_label = func_label_name(self.function, label);
        vec![
            asm!(@"SP"),
            asm!(AM = M - 1),
            asm!(D = M),
            asm!(@full_label),
            asm!(D;JNE),
        ]
    }

    fn translate_function(&mut self, func: &'a str, locals: u16) -> Vec<Instruction> {
        self.function = Some(func);

        let init = match locals {
            0 => vec![],
            1 | 2 => std::iter::repeat(vec![
                asm!(@"SP"),
                asm!(AM = M + 1),
                asm!(A = A - 1),
                asm!(M = 0),
            ])
            .take(locals as usize)
            .flatten()
            .collect(),
            _ => {
                let label = format!("LOOP_{}", func);
                vec![
                    asm!(@locals),
                    asm!(D = A),
                    asm!((label.clone())),
                    asm!(D = D - 1),
                    asm!(@"SP"),
                    asm!(AM = M + 1),
                    asm!(A = A - 1),
                    asm!(M = 0),
                    asm!(@label),
                    asm!(D;JGT),
                ]
            }
        };

        let mut instr = vec![asm!((func))];
        instr.extend(init);
        instr
    }

    fn translate_call(&mut self, func: String, args: u16) -> Vec<Instruction> {
        let label = self.labeller.generate("RET_ADDRESS_CALL");

        let mut instr = match args {
            0 => vec![asm!(@"R13"), asm!(M = 0)],
            1 => vec![asm!(@"R13"), asm!(M = 1)],
            2 => vec![asm!(@"R13"), asm!(D = 1), asm!(M = D + 1)],
            _ => vec![asm!(@args), asm!(D = A), asm!(@"R13"), asm!(M = D)],
        };

        instr.extend_from_slice(&[
            asm!(@func),
            asm!(D = A),
            asm!(@"R14"),
            asm!(M = D),
            asm!(@label.clone()),
            asm!(D = A),
            asm!(@"CALL"),
            asm!(0;JMP),
            asm!((label)),
        ]);

        instr
    }
}

fn eq() -> Vec<Instruction> {
    vec![
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
    ]
}

fn lt() -> Vec<Instruction> {
    vec![
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
    ]
}

fn gt() -> Vec<Instruction> {
    vec![
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
    ]
}

fn ret() -> Vec<Instruction> {
    vec![
        asm!(("RETURN")),
        asm!(@5),
        asm!(D = A),
        asm!(@"LCL"),
        asm!(A = M - D),
        asm!(D = M),
        asm!(@"R13"),
        asm!(M = D),
        asm!(@"SP"),
        asm!(AM = M - 1),
        asm!(D = M),
        asm!(@"ARG"),
        asm!(A = M),
        asm!(M = D),
        asm!(D = A),
        asm!(@"SP"),
        asm!(M = D + 1),
        asm!(@"LCL"),
        asm!(D = M),
        asm!(@"R14"),
        asm!(AM = D - 1),
        asm!(D = M),
        asm!(@"THAT"),
        asm!(M = D),
        asm!(@"R14"),
        asm!(AM = M - 1),
        asm!(D = M),
        asm!(@"THIS"),
        asm!(M = D),
        asm!(@"R14"),
        asm!(AM = M - 1),
        asm!(D = M),
        asm!(@"ARG"),
        asm!(M = D),
        asm!(@"R14"),
        asm!(AM = M - 1),
        asm!(D = M),
        asm!(@"LCL"),
        asm!(M = D),
        asm!(@"R13"),
        asm!(A = M),
        asm!(0;JMP),
    ]
}

fn call() -> Vec<Instruction> {
    vec![
        asm!(("CALL")),
        asm!(@"SP"),
        asm!(A = M),
        asm!(M = D),
        asm!(@"LCL"),
        asm!(D = M),
        asm!(@"SP"),
        asm!(AM = M + 1),
        asm!(M = D),
        asm!(@"ARG"),
        asm!(D = M),
        asm!(@"SP"),
        asm!(AM = M + 1),
        asm!(M = D),
        asm!(@"THIS"),
        asm!(D = M),
        asm!(@"SP"),
        asm!(AM = M + 1),
        asm!(M = D),
        asm!(@"THAT"),
        asm!(D = M),
        asm!(@"SP"),
        asm!(AM = M + 1),
        asm!(M = D),
        asm!(@4),
        asm!(D = A),
        asm!(@"R13"),
        asm!(D = D + M),
        asm!(@"SP"),
        asm!(D = M - D),
        asm!(@"ARG"),
        asm!(M = D),
        asm!(@"SP"),
        asm!(MD = M + 1),
        asm!(@"LCL"),
        asm!(M = D),
        asm!(@"R14"),
        asm!(A = M),
        asm!(0;JMP),
    ]
}

fn temp_register(idx: u16) -> Result<Instruction> {
    match idx {
        0 => Ok(asm!(@"R5")),
        1 => Ok(asm!(@"R6")),
        2 => Ok(asm!(@"R7")),
        3 => Ok(asm!(@"R8")),
        4 => Ok(asm!(@"R9")),
        5 => Ok(asm!(@"R10")),
        6 => Ok(asm!(@"R11")),
        7 => Ok(asm!(@"R12")),
        _ => Err(anyhow!("`{}` is not a valid temp segment index", idx)),
    }
}

fn segment_register(seg: Segment) -> Result<Instruction> {
    match seg {
        Segment::Argument => Ok(asm!(@"ARG")),
        Segment::Local => Ok(asm!(@"LCL")),
        Segment::This => Ok(asm!(@"THIS")),
        Segment::That => Ok(asm!(@"THAT")),
        _ => Err(anyhow!("unexpected segment `{}`", seg)),
    }
}

fn static_addr(module: &Module, idx: u16) -> Instruction {
    let symbol = format!("{}.{}", module.name, idx);
    asm!(@symbol)
}

fn func_label_name(function: Option<&str>, label: &str) -> String {
    function.map_or_else(|| label.to_owned(), |func| format!("{}${}", func, label))
}
