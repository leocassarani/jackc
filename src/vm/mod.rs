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

#[derive(Debug, Eq, PartialEq)]
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
