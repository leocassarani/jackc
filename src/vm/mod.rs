mod command;
mod translator;

pub use command::*;
pub use translator::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Module {
    pub name: String,
    pub cmds: Vec<Command>,
}

impl Module {
    pub fn new(name: String, cmds: Vec<Command>) -> Self {
        Module { name, cmds }
    }
}
