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
            Command::Pop(Segment::Argument, n) => vec![
                "@ARG".parse(),
                "D=M".parse(),
                Ok(hack::Instruction::A(*n)),
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
            Command::Pop(Segment::Local, 0) => vec![
                "@LCL".parse(),
                "D=M".parse(),
                "@R13".parse(),
                "M=D".parse(),
                "@SP".parse(),
                "AM=M-1".parse(),
                "D=M".parse(),
                "@R13".parse(),
                "A=M".parse(),
                "M=D".parse(),
            ],
            Command::Pop(Segment::This, n) => vec![
                "@THIS".parse(),
                "D=M".parse(),
                Ok(hack::Instruction::A(*n)),
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
            Command::Pop(Segment::That, n) => vec![
                "@THAT".parse(),
                "D=M".parse(),
                Ok(hack::Instruction::A(*n)),
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
            Command::Pop(Segment::Temp, 6) => vec![
                "@SP".parse(),
                "AM=M-1".parse(),
                "D=M".parse(),
                "@11".parse(),
                "M=D".parse(),
            ],
            Command::Push(Segment::Constant, n) => vec![
                // TODO: optimise this for known constants like 0
                Ok(hack::Instruction::A(*n)),
                "D=A".parse(),
                "@SP".parse(),
                "AM=M+1".parse(),
                "A=A-1".parse(),
                "M=D".parse(),
            ],
            Command::Push(Segment::Argument, n) => vec![
                "@ARG".parse(),
                "D=M".parse(),
                Ok(hack::Instruction::A(*n)),
                "A=D+A".parse(),
                "D=M".parse(),
                "@SP".parse(),
                "AM=M+1".parse(),
                "A=A-1".parse(),
                "M=D".parse(),
            ],
            Command::Push(Segment::Local, 0) => vec![
                "@LCL".parse(),
                "A=M".parse(),
                "D=M".parse(),
                "@SP".parse(),
                "AM=M+1".parse(),
                "A=A-1".parse(),
                "M=D".parse(),
            ],
            Command::Push(Segment::This, n) => vec![
                "@THIS".parse(),
                "D=M".parse(),
                Ok(hack::Instruction::A(*n)),
                "A=D+A".parse(),
                "D=M".parse(),
                "@SP".parse(),
                "AM=M+1".parse(),
                "A=A-1".parse(),
                "M=D".parse(),
            ],
            Command::Push(Segment::That, n) => vec![
                "@THAT".parse(),
                "D=M".parse(),
                Ok(hack::Instruction::A(*n)),
                "A=D+A".parse(),
                "D=M".parse(),
                "@SP".parse(),
                "AM=M+1".parse(),
                "A=A-1".parse(),
                "M=D".parse(),
            ],
            Command::Push(Segment::Temp, 6) => vec![
                "@11".parse(),
                "D=M".parse(),
                "@SP".parse(),
                "AM=M+1".parse(),
                "A=A-1".parse(),
                "M=D".parse(),
            ],
            _ => unimplemented!(),
        };

        instructions.into_iter().collect::<Result<_, _>>().unwrap()
    }
}
