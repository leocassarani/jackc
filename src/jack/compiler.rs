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
        let name = format!("{}.{}", self.class.name, sub.name);
        let locals = sub.body.vars.len() as u16;

        let func = vm::Command::Function(name, locals); // TODO: number of locals

        let body = sub.body.statements.iter().flat_map(|st| match st {
            Statement::Let { lhs, index, rhs } => self.compile_let(lhs, index.as_ref(), rhs),
            Statement::Do(call) => self.compile_do(call),
            Statement::Return(value) => self.compile_return(value.as_ref()),
            _ => unimplemented!(),
        });

        iter::once(func).chain(body).collect()
    }

    fn compile_let(&self, _lhs: &str, _index: Option<&Expr>, rhs: &Expr) -> Vec<vm::Command> {
        let mut cmds = self.compile_expr(rhs);
        cmds.push(vm::Command::Pop(vm::Segment::Local, 0));
        cmds
    }

    fn compile_do(&self, call: &SubroutineCall) -> Vec<vm::Command> {
        let mut cmds = self.compile_subroutine_call(call);
        cmds.push(vm::Command::Pop(vm::Segment::Temp, 0));
        cmds
    }

    fn compile_return(&self, value: Option<&Expr>) -> Vec<vm::Command> {
        let push = match value {
            Some(_) => unimplemented!(),
            None => vm::Command::Push(vm::Segment::Constant, 0),
        };

        vec![push, vm::Command::Return]
    }

    fn compile_subroutine_call(&self, call: &SubroutineCall) -> Vec<vm::Command> {
        let args = call.args.iter().flat_map(|arg| self.compile_expr(arg));

        let mut name = match call.receiver.as_ref() {
            Some(class) => format!("{}.", class),
            _ => String::new(),
        };
        name.push_str(&call.subroutine);

        let jump = iter::once(vm::Command::Call(name, call.args.len() as u16));
        args.chain(jump).collect()
    }

    fn compile_expr(&self, expr: &Expr) -> Vec<vm::Command> {
        match expr {
            Expr::Term(term) => self.compile_term(term),
            Expr::Binary(op, left, right) => {
                let mut cmds = self.compile_term(left);
                cmds.extend(self.compile_expr(right).into_iter());
                cmds.push(self.compile_binary_op(*op));
                cmds
            }
        }
    }

    fn compile_term(&self, term: &Term) -> Vec<vm::Command> {
        match term {
            Term::IntConst(n) => vec![vm::Command::Push(vm::Segment::Constant, *n as u16)],
            Term::Var(_name) => vec![vm::Command::Push(vm::Segment::Local, 0)],
            Term::SubroutineCall(call) => self.compile_subroutine_call(call),
            Term::Bracketed(expr) => self.compile_expr(expr),
            Term::Unary(op, subterm) => {
                let mut cmds = self.compile_term(subterm);
                cmds.push(self.compile_unary_op(*op));
                cmds
            }
            _ => unimplemented!(),
        }
    }

    fn compile_binary_op(&self, op: BinaryOp) -> vm::Command {
        match op {
            BinaryOp::Add => vm::Command::Add,
            BinaryOp::Multiply => vm::Command::Call("Math.multiply".to_string(), 2),
            _ => unimplemented!(),
        }
    }

    fn compile_unary_op(&self, op: UnaryOp) -> vm::Command {
        match op {
            UnaryOp::Minus => vm::Command::Neg,
            UnaryOp::Not => vm::Command::Not,
        }
    }
}
