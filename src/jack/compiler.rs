use super::parser::*;
use crate::vm;
use std::iter;

pub struct Compiler {
    class: Class,
}

impl Compiler {
    pub fn new(class: Class) -> Self {
        Compiler { class }
    }

    pub fn compile(&self) -> Vec<vm::Command> {
        self.class
            .subs
            .iter()
            .flat_map(|sub| self.compile_subroutine(sub))
            .collect()
    }

    fn compile_subroutine(&self, sub: &Subroutine) -> Vec<vm::Command> {
        let name = format!("{}.{}", self.class.name, sub.name).to_ascii_lowercase();

        let func = vm::Command::Function(name, 0); // TODO: number of locals

        let body = sub.body.statements.iter().flat_map(|st| match st {
            Statement::Do(call) => self.compile_do(call),
            Statement::Return(value) => self.compile_return(value.as_ref()),
            _ => unimplemented!(),
        });

        iter::once(func).chain(body).collect()
    }

    fn compile_do(&self, call: &SubroutineCall) -> Vec<vm::Command> {
        let args = call.args.iter().flat_map(|arg| self.compile_expr(arg));

        let mut name = match call.receiver.as_ref() {
            Some(class) => format!("{}.", class.to_ascii_lowercase()),
            _ => String::new(),
        };

        name.push_str(&call.subroutine.to_ascii_lowercase());

        let jump = vec![
            vm::Command::Call(name, 1),
            vm::Command::Pop(vm::Segment::Temp, 0),
        ];

        args.chain(jump).collect()
    }

    fn compile_return(&self, value: Option<&Expr>) -> Vec<vm::Command> {
        let push = match value {
            Some(_) => unimplemented!(),
            None => vm::Command::Push(vm::Segment::Constant, 0),
        };

        vec![push, vm::Command::Return]
    }

    fn compile_expr(&self, expr: &Expr) -> Vec<vm::Command> {
        match expr {
            Expr::Term(term) => self.compile_term(term),
            Expr::Binary(op, left, right) => {
                let mut cmds = self.compile_term(left);
                cmds.extend(self.compile_expr(right).into_iter());
                cmds.push(self.compile_op(*op));
                cmds
            }
        }
    }

    fn compile_term(&self, term: &Term) -> Vec<vm::Command> {
        match term {
            Term::IntConst(n) => vec![vm::Command::Push(vm::Segment::Constant, *n as u16)],
            Term::Bracketed(expr) => self.compile_expr(expr),
            _ => unimplemented!(),
        }
    }

    fn compile_op(&self, op: BinaryOp) -> vm::Command {
        match op {
            BinaryOp::Add => vm::Command::Add,
            BinaryOp::Multiply => vm::Command::Call("math.multiply".to_string(), 2),
            _ => unimplemented!(),
        }
    }
}
