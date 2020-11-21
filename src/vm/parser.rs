use super::*;
use anyhow::{anyhow, Error};
use std::str::FromStr;

pub fn parse(s: &str) -> Result<Vec<Command>, Error> {
    s.lines().map(|line| line.parse()).collect()
}

impl FromStr for Command {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "add" => Ok(Command::Add),
            "sub" => Ok(Command::Sub),
            "neg" => Ok(Command::Neg),
            "eq" => Ok(Command::Eq),
            "gt" => Ok(Command::Gt),
            "lt" => Ok(Command::Lt),
            "and" => Ok(Command::And),
            "or" => Ok(Command::Or),
            "not" => Ok(Command::Not),
            "return" => Ok(Command::Return),
            _ if s.starts_with("pop") => {
                match s.split_whitespace().skip(1).collect::<Vec<_>>().as_slice() {
                    [segment, index] => {
                        let segment = segment.parse()?;
                        let index = index.parse()?;
                        Ok(Command::Pop(segment, index))
                    }
                    got => Err(anyhow!(
                        "expected a segment followed by a number, found `{}`",
                        got.join(" ")
                    )),
                }
            }
            _ if s.starts_with("push") => {
                match s.split_whitespace().skip(1).collect::<Vec<_>>().as_slice() {
                    [segment, index] => {
                        let segment = segment.parse()?;
                        let index = index.parse()?;
                        Ok(Command::Push(segment, index))
                    }
                    got => Err(anyhow!(
                        "expected a segment followed by a number, found `{}`",
                        got.join(" ")
                    )),
                }
            }
            _ if s.starts_with("label") => {
                match s.split_whitespace().skip(1).collect::<Vec<_>>().as_slice() {
                    [label] => Ok(Command::Label(label.to_string())),
                    got => Err(anyhow!("expected a label, found `{}`", got.join(" "))),
                }
            }
            _ if s.starts_with("goto") => {
                match s.split_whitespace().skip(1).collect::<Vec<_>>().as_slice() {
                    [label] => Ok(Command::Goto(label.to_string())),
                    got => Err(anyhow!("expected a label, found `{}`", got.join(" "))),
                }
            }
            _ if s.starts_with("if-goto") => {
                match s.split_whitespace().skip(1).collect::<Vec<_>>().as_slice() {
                    [label] => Ok(Command::IfGoto(label.to_string())),
                    got => Err(anyhow!("expected a label, found `{}`", got.join(" "))),
                }
            }
            _ if s.starts_with("function") => {
                match s.split_whitespace().skip(1).collect::<Vec<_>>().as_slice() {
                    [function, locals] => {
                        let locals = locals.parse()?;
                        Ok(Command::Function(function.to_string(), locals))
                    }
                    got => Err(anyhow!(
                        "expected a function name followed by a number, found `{}`",
                        got.join(" ")
                    )),
                }
            }
            _ if s.starts_with("call") => {
                match s.split_whitespace().skip(1).collect::<Vec<_>>().as_slice() {
                    [function, args] => {
                        let args = args.parse()?;
                        Ok(Command::Call(function.to_string(), args))
                    }
                    got => Err(anyhow!(
                        "expected a function name followed by a number, found `{}`",
                        got.join(" ")
                    )),
                }
            }
            _ => Err(anyhow!("`{}` is not a valid VM command", s)),
        }
    }
}

impl FromStr for Segment {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "argument" => Ok(Segment::Argument),
            "local" => Ok(Segment::Local),
            "static" => Ok(Segment::Static),
            "constant" => Ok(Segment::Constant),
            "this" => Ok(Segment::This),
            "that" => Ok(Segment::That),
            "pointer" => Ok(Segment::Pointer),
            "temp" => Ok(Segment::Temp),
            _ => Err(anyhow!("`{}` is not a valid segment", s)),
        }
    }
}
