use crate::hack;
use std::fmt;

#[derive(Debug, Eq, PartialEq)]
pub enum Command {
    Add,
    Sub,
    Neg,
    Eq,
    Gt,
    Lt,
    And,
    Or,
    Not,
    Pop(Segment, u16),
    Push(Segment, u16),
    Label(String),
    Goto(String),
    IfGoto(String),
    Function(String, u16),
    Call(String, u16),
    Return,
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Command::Add => write!(f, "add"),
            Command::Sub => write!(f, "sub"),
            Command::Neg => write!(f, "neg"),
            Command::Eq => write!(f, "eq"),
            Command::Gt => write!(f, "gt"),
            Command::Lt => write!(f, "lt"),
            Command::And => write!(f, "and"),
            Command::Or => write!(f, "or"),
            Command::Not => write!(f, "not"),
            Command::Pop(segment, index) => write!(f, "pop {} {}", segment, index),
            Command::Push(segment, index) => write!(f, "push {} {}", segment, index),
            Command::Label(symbol) => write!(f, "label {}", symbol),
            Command::Goto(symbol) => write!(f, "goto {}", symbol),
            Command::IfGoto(symbol) => write!(f, "if-goto {}", symbol),
            Command::Function(name, locals) => write!(f, "function {} {}", name, locals),
            Command::Call(name, args) => write!(f, "call {} {}", name, args),
            Command::Return => write!(f, "return"),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Segment {
    Argument,
    Local,
    Static,
    Constant,
    This,
    That,
    Pointer,
    Temp,
}

impl fmt::Display for Segment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Segment::Argument => write!(f, "argument"),
            Segment::Local => write!(f, "local"),
            Segment::Static => write!(f, "static"),
            Segment::Constant => write!(f, "constant"),
            Segment::This => write!(f, "this"),
            Segment::That => write!(f, "that"),
            Segment::Pointer => write!(f, "pointer"),
            Segment::Temp => write!(f, "temp"),
        }
    }
}

pub struct Translator<'a> {
    cmds: &'a [Command],
}

impl<'a> Translator<'a> {
    pub fn new(cmds: &'a [Command]) -> Self {
        Translator { cmds }
    }

    pub fn translate(&self) -> Vec<hack::Instruction> {
        self.cmds
            .iter()
            .flat_map(|cmd| self.translate_cmd(cmd))
            .collect()
    }

    fn translate_cmd(&self, cmd: &Command) -> Vec<hack::Instruction> {
        let instructions = match cmd {
            Command::Add => vec![
                "@SP".parse(),
                "AM=M-1".parse(),
                "D=M".parse(),
                "A=A-1".parse(),
                "M=D+M".parse(),
            ],
            Command::Sub => vec![
                "@SP".parse(),
                "AM=M-1".parse(),
                "D=M".parse(),
                "A=A-1".parse(),
                "M=M-D".parse(),
            ],
            Command::Pop(Segment::Pointer, 0) => vec![
                "@SP".parse(),
                "AM=M-1".parse(),
                "D=M".parse(),
                "@THIS".parse(),
                "M=D".parse(),
            ],
            Command::Pop(Segment::Pointer, 1) => vec![
                "@SP".parse(),
                "AM=M-1".parse(),
                "D=M".parse(),
                "@THAT".parse(),
                "M=D".parse(),
            ],
            Command::Pop(Segment::Temp, n) => self.translate_pop_temp(*n),
            Command::Pop(segment, n) => self.translate_pop(segment, *n),
            Command::Push(Segment::Constant, n) => self.translate_push_const(*n),
            Command::Push(Segment::Pointer, 0) => vec![
                "@THIS".parse(),
                "D=M".parse(),
                "@SP".parse(),
                "AM=M+1".parse(),
                "A=A-1".parse(),
                "M=D".parse(),
            ],
            Command::Push(Segment::Pointer, 1) => vec![
                "@THAT".parse(),
                "D=M".parse(),
                "@SP".parse(),
                "AM=M+1".parse(),
                "A=A-1".parse(),
                "M=D".parse(),
            ],
            Command::Push(Segment::Temp, n) => self.translate_push_temp(*n),
            Command::Push(segment, n) => self.translate_push(segment, *n),
            _ => unimplemented!(),
        };

        instructions.into_iter().collect::<Result<_, _>>().unwrap()
    }

    fn translate_pop_temp(&self, n: u16) -> Vec<Result<hack::Instruction, ()>> {
        let addr = match n {
            0 => "@R5".parse(),
            1 => "@R6".parse(),
            2 => "@R7".parse(),
            3 => "@R8".parse(),
            4 => "@R9".parse(),
            5 => "@R10".parse(),
            6 => "@R11".parse(),
            7 => "@R12".parse(),
            _ => Err(()),
        };

        vec![
            "@SP".parse(),
            "AM=M-1".parse(),
            "D=M".parse(),
            addr,
            "M=D".parse(),
        ]
    }

    fn translate_pop(&self, segment: &Segment, n: u16) -> Vec<Result<hack::Instruction, ()>> {
        let base = match segment {
            Segment::Argument => "@ARG".parse(),
            Segment::Local => "@LCL".parse(),
            Segment::This => "@THIS".parse(),
            Segment::That => "@THAT".parse(),
            _ => Err(()),
        };

        match n {
            0 => vec![
                "@SP".parse(),
                "AM=M-1".parse(),
                "D=M".parse(),
                base,
                "A=M".parse(),
                "M=D".parse(),
            ],
            1..=6 => {
                let mut inst = vec![
                    "@SP".parse(),
                    "AM=M-1".parse(),
                    "D=M".parse(),
                    base,
                    "A=M+1".parse(),
                ];

                for _ in 1..n {
                    inst.push("A=A+1".parse());
                }

                inst.push("M=D".parse());
                inst
            }
            _ => vec![
                base,
                "D=M".parse(),
                Ok(hack::Instruction::A(n)),
                "D=D+A".parse(),
                "@R13".parse(),
                "M=D".parse(),
                "@SP".parse(),
                "AM=M-1".parse(),
                "D=M".parse(),
                "@R13".parse(),
                "A=M".parse(),
                "M=D".parse(),
            ],
        }
    }

    fn translate_push_const(&self, n: u16) -> Vec<Result<hack::Instruction, ()>> {
        let mut instr = vec!["@SP".parse(), "AM=M+1".parse(), "A=A-1".parse()];

        match n {
            0 => instr.push("M=0".parse()),
            1 => instr.push("M=1".parse()),
            2 => instr.extend(vec!["M=1".parse(), "M=M+1".parse()]),
            _ => {
                let load = if n >> 15 == 0 {
                    vec![Ok(hack::Instruction::A(n)), "D=A".parse()]
                } else {
                    vec![Ok(hack::Instruction::A(n & 0x7fff)), "D=-A".parse()]
                };
                instr.splice(0..0, load);
                instr.push("M=D".parse());
            }
        };

        instr
    }

    fn translate_push_temp(&self, n: u16) -> Vec<Result<hack::Instruction, ()>> {
        let addr = match n {
            0 => "@R5".parse(),
            1 => "@R6".parse(),
            2 => "@R7".parse(),
            3 => "@R8".parse(),
            4 => "@R9".parse(),
            5 => "@R10".parse(),
            6 => "@R11".parse(),
            7 => "@R12".parse(),
            _ => Err(()),
        };

        vec![
            addr,
            "D=M".parse(),
            "@SP".parse(),
            "AM=M+1".parse(),
            "A=A-1".parse(),
            "M=D".parse(),
        ]
    }

    fn translate_push(&self, segment: &Segment, n: u16) -> Vec<Result<hack::Instruction, ()>> {
        let base = match segment {
            Segment::Argument => "@ARG".parse(),
            Segment::Local => "@LCL".parse(),
            Segment::This => "@THIS".parse(),
            Segment::That => "@THAT".parse(),
            _ => Err(()),
        };

        match n {
            0 => vec![
                base,
                "A=M".parse(),
                "D=M".parse(),
                "@SP".parse(),
                "AM=M+1".parse(),
                "A=A-1".parse(),
                "M=D".parse(),
            ],
            1 | 2 => {
                let mut inst = vec![base, "A=M+1".parse()];
                for _ in 1..n {
                    inst.push("A=A+1".parse());
                }
                inst.extend(vec![
                    "D=M".parse(),
                    "@SP".parse(),
                    "AM=M+1".parse(),
                    "A=A-1".parse(),
                    "M=D".parse(),
                ]);
                inst
            }
            _ => vec![
                base,
                "D=M".parse(),
                Ok(hack::Instruction::A(n)),
                "A=D+A".parse(),
                "D=M".parse(),
                "@SP".parse(),
                "AM=M+1".parse(),
                "A=A-1".parse(),
                "M=D".parse(),
            ],
        }
    }
}
