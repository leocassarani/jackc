use super::parser::*;
use super::symbol_table::{Kind, SymbolTable, Type};
use crate::vm;
use std::collections::HashMap;
use std::iter;

pub struct Compiler {
    symbols: SymbolTable,
    labels: Labeller,
}

impl Compiler {
    pub fn new() -> Self {
        Compiler {
            symbols: SymbolTable::new(),
            labels: Labeller::new(),
        }
    }

    pub fn compile(&mut self, class: &Class) -> Vec<vm::Command> {
        class
            .subs
            .iter()
            .flat_map(|sub| self.compile_subroutine(class, sub))
            .collect()
    }

    fn compile_subroutine(&mut self, class: &Class, sub: &Subroutine) -> Vec<vm::Command> {
        self.symbols.start_subroutine();
        self.labels.reset();

        for param in &sub.params {
            self.symbols
                .define(param.name.clone(), Type::from(&param.typ), Kind::Argument);
        }

        let name = format!("{}.{}", class.name, sub.name);
        let mut locals: u16 = 0;

        for vars in &sub.body.vars {
            for name in &vars.names {
                locals += 1;
                self.symbols
                    .define(name.clone(), Type::from(&vars.typ), Kind::LocalVar);
            }
        }

        iter::once(vm::Command::Function(name, locals))
            .chain(self.compile_statements(&sub.body.statements))
            .collect()
    }

    fn compile_statements(&mut self, stmts: &[Statement]) -> Vec<vm::Command> {
        stmts
            .iter()
            .flat_map(|stmt| self.compile_statement(stmt))
            .collect()
    }

    fn compile_statement(&mut self, stmt: &Statement) -> Vec<vm::Command> {
        match stmt {
            Statement::Let { lhs, index, rhs } => self.compile_let(lhs, index.as_ref(), rhs),
            Statement::If {
                condition,
                if_body,
                else_body,
            } => self.compile_if(condition, if_body, else_body.as_ref()),
            Statement::While { condition, body } => self.compile_while(condition, body),
            Statement::Do(call) => self.compile_do(call),
            Statement::Return(value) => self.compile_return(value.as_ref()),
        }
    }

    fn compile_let(&self, lhs: &str, _index: Option<&Expr>, rhs: &Expr) -> Vec<vm::Command> {
        let mut cmds = self.compile_expr(rhs);
        cmds.extend(self.compile_var_write(lhs));
        cmds
    }

    fn compile_if(
        &mut self,
        condition: &Expr,
        if_body: &[Statement],
        else_body: Option<&Vec<Statement>>,
    ) -> Vec<vm::Command> {
        let true_label = self.labels.generate("IF_TRUE");
        let false_label = self.labels.generate("IF_FALSE");
        let end_label = self.labels.generate("IF_END");

        let mut cmds = self.compile_expr(condition);

        cmds.extend(vec![
            vm::Command::IfGoto(true_label.clone()),
            vm::Command::Goto(false_label.clone()),
            vm::Command::Label(true_label.clone()),
        ]);

        cmds.extend(self.compile_statements(if_body));

        match else_body {
            Some(body) => {
                cmds.extend(vec![
                    vm::Command::Goto(end_label.clone()),
                    vm::Command::Label(false_label.clone()),
                ]);
                cmds.extend(self.compile_statements(body));
                cmds.push(vm::Command::Label(end_label.clone()));
            }
            None => {
                cmds.push(vm::Command::Label(false_label.clone()));
            }
        }

        cmds
    }

    fn compile_while(&mut self, condition: &Expr, body: &[Statement]) -> Vec<vm::Command> {
        let exp_label = self.labels.generate("WHILE_EXP");
        let end_label = self.labels.generate("WHILE_END");

        let mut cmds = vec![vm::Command::Label(exp_label.clone())];

        cmds.extend(self.compile_expr(condition));
        cmds.extend(vec![
            vm::Command::Not,
            vm::Command::IfGoto(end_label.clone()),
        ]);

        cmds.extend(self.compile_statements(body));
        cmds.extend(vec![
            vm::Command::Goto(exp_label.clone()),
            vm::Command::Label(end_label.clone()),
        ]);

        cmds
    }

    fn compile_do(&self, call: &SubroutineCall) -> Vec<vm::Command> {
        let mut cmds = self.compile_subroutine_call(call);
        cmds.push(vm::Command::Pop(vm::Segment::Temp, 0));
        cmds
    }

    fn compile_return(&self, value: Option<&Expr>) -> Vec<vm::Command> {
        let mut cmds = match value {
            Some(expr) => self.compile_expr(expr),
            None => vec![vm::Command::Push(vm::Segment::Constant, 0)],
        };
        cmds.push(vm::Command::Return);
        cmds
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
            Term::IntConst(n) => self.compile_int_const(*n),
            Term::KeywordConst(kw) => self.compile_keyword(kw),
            Term::Var(name) => self.compile_var_read(name),
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

    fn compile_int_const(&self, n: i16) -> Vec<vm::Command> {
        vec![vm::Command::Push(vm::Segment::Constant, n as u16)]
    }

    fn compile_keyword(&self, kw: &KeywordConst) -> Vec<vm::Command> {
        match kw {
            KeywordConst::True => vec![
                vm::Command::Push(vm::Segment::Constant, 1),
                vm::Command::Neg,
            ],
            KeywordConst::False | KeywordConst::Null => {
                vec![vm::Command::Push(vm::Segment::Constant, 0)]
            }
            _ => unimplemented!(),
        }
    }

    fn compile_var_read(&self, name: &str) -> Vec<vm::Command> {
        let symbol = self.symbols.get(&name).expect("undefined symbol");

        match symbol.kind {
            Kind::Argument => vec![vm::Command::Push(vm::Segment::Argument, symbol.index)],
            Kind::LocalVar => vec![vm::Command::Push(vm::Segment::Local, symbol.index)],
            _ => unimplemented!(),
        }
    }

    fn compile_var_write(&self, name: &str) -> Vec<vm::Command> {
        let symbol = self.symbols.get(&name).expect("undefined symbol");

        match symbol.kind {
            Kind::Argument => vec![vm::Command::Pop(vm::Segment::Argument, symbol.index)],
            Kind::LocalVar => vec![vm::Command::Pop(vm::Segment::Local, symbol.index)],
            _ => unimplemented!(),
        }
    }

    fn compile_binary_op(&self, op: BinaryOp) -> vm::Command {
        match op {
            BinaryOp::Add => vm::Command::Add,
            BinaryOp::Subtract => vm::Command::Sub,
            BinaryOp::Multiply => vm::Command::Call("Math.multiply".to_string(), 2),
            BinaryOp::And => vm::Command::And,
            BinaryOp::GreaterThan => vm::Command::Gt,
            BinaryOp::Equal => vm::Command::Eq,
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

struct Labeller {
    labels: HashMap<&'static str, u16>,
}

impl Labeller {
    fn new() -> Self {
        Labeller {
            labels: HashMap::new(),
        }
    }

    fn generate(&mut self, prefix: &'static str) -> String {
        let count = self.labels.entry(prefix).or_insert(0);
        let label = format!("{}{}", prefix, *count);
        *count += 1;
        label
    }

    fn reset(&mut self) {
        self.labels.clear();
    }
}
