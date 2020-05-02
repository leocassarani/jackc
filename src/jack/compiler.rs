use super::parser::*;
use super::symbol_table::{Kind, SymbolTable, Type};
use crate::vm;
use std::collections::HashMap;

pub struct Compiler<'a> {
    class: &'a Class,
    symbols: SymbolTable,
    labels: Labeller,
}

impl<'a> Compiler<'a> {
    pub fn new(class: &'a Class) -> Self {
        Compiler {
            class,
            symbols: SymbolTable::new(),
            labels: Labeller::new(),
        }
    }

    pub fn compile(&mut self) -> vm::Module {
        self.symbols.reset();

        for vars in &self.class.vars {
            for name in &vars.names {
                self.symbols
                    .define(name.clone(), Type::from(&vars.typ), Kind::from(&vars.kind));
            }
        }

        let cmds = self
            .class
            .subs
            .iter()
            .flat_map(|sub| self.compile_subroutine(sub))
            .collect();

        vm::Module::new(self.class.name.clone(), cmds)
    }

    fn compile_subroutine(&mut self, sub: &Subroutine) -> Vec<vm::Command> {
        self.symbols.start_subroutine();
        self.labels.reset();

        let name = format!("{}.{}", self.class.name, sub.name);

        let locals = sub
            .body
            .vars
            .iter()
            .map(|vars| vars.names.len() as u16)
            .sum();

        let mut cmds = vec![vm::Command::Function(name, locals)];

        match sub.kind {
            SubroutineKind::Constructor => {
                let fields = self
                    .class
                    .vars
                    .iter()
                    .filter(|vars| vars.kind == ClassVarKind::Field)
                    .map(|vars| vars.names.len() as u16)
                    .sum();

                cmds.extend(vec![
                    vm::Command::Push(vm::Segment::Constant, fields),
                    vm::Command::Call("Memory.alloc".to_owned(), 1),
                    vm::Command::Pop(vm::Segment::Pointer, 0),
                ]);
            }
            SubroutineKind::Method => {
                // If this is a method, the symbol table must be pre-filled with "this", which
                // would have been passed in as the first argument. As "this" is a keyword rather
                // than an identifier, it will never be looked up in the symbol table, but defining
                // it in the symbol table will have the desired side-effect of causing subsequent
                // method arguments to start from the index 1 rather than 0.
                self.symbols.define(
                    "this".to_owned(),
                    Type::ClassName(self.class.name.clone()),
                    Kind::Argument,
                );

                cmds.extend(vec![
                    vm::Command::Push(vm::Segment::Argument, 0),
                    vm::Command::Pop(vm::Segment::Pointer, 0),
                ]);
            }
            _ => {}
        }

        for param in &sub.params {
            self.symbols
                .define(param.name.clone(), Type::from(&param.typ), Kind::Argument);
        }

        for vars in &sub.body.vars {
            for name in &vars.names {
                self.symbols
                    .define(name.clone(), Type::from(&vars.typ), Kind::LocalVar);
            }
        }

        cmds.extend(self.compile_statements(&sub.body.statements));
        cmds
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

    fn compile_let(&self, lhs: &str, index: Option<&Expr>, rhs: &Expr) -> Vec<vm::Command> {
        let mut cmds = self.compile_expr(rhs);

        match index {
            Some(expr) => {
                cmds.push(self.compile_var(vm::Command::Push, lhs));
                cmds.extend(self.compile_expr(expr));
                cmds.extend(vec![
                    vm::Command::Add,
                    vm::Command::Pop(vm::Segment::Pointer, 1),
                    vm::Command::Pop(vm::Segment::That, 0),
                ]);
            }
            None => cmds.push(self.compile_var(vm::Command::Pop, lhs)),
        }

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
        let mut cmds = Vec::new();
        let mut args = call.args.len() as u16;
        let receiver: &str;

        match call.receiver.as_ref() {
            Some(recv) => match self.symbols.get(recv) {
                Some(sym) => {
                    if let Type::ClassName(class) = &sym.typ {
                        receiver = &class;
                    } else {
                        panic!(
                            "can't call method {} on primitive type receiver {}",
                            call.subroutine, recv
                        );
                    }

                    cmds.push(self.compile_var(vm::Command::Push, recv));
                    args += 1;
                }
                None => receiver = &recv,
            },
            None => {
                receiver = &self.class.name;
                cmds.push(vm::Command::Push(vm::Segment::Pointer, 0));
                args += 1;
            }
        }

        for arg in &call.args {
            cmds.extend(self.compile_expr(arg));
        }

        let name = format!("{}.{}", receiver, call.subroutine);
        cmds.push(vm::Command::Call(name, args));

        cmds
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
            Term::IntConst(n) => vec![self.compile_int_const(*n)],
            Term::StrConst(s) => self.compile_str_const(s),
            Term::KeywordConst(kw) => self.compile_keyword(kw),
            Term::Var(name) => vec![self.compile_var(vm::Command::Push, name)],
            Term::IndexedVar(name, expr) => self.compile_indexed_var(name, expr),
            Term::SubroutineCall(call) => self.compile_subroutine_call(call),
            Term::Bracketed(expr) => self.compile_expr(expr),
            Term::Unary(op, subterm) => self.compile_unary(*op, subterm),
        }
    }

    fn compile_int_const(&self, n: u16) -> vm::Command {
        vm::Command::Push(vm::Segment::Constant, n)
    }

    fn compile_str_const(&self, s: &str) -> Vec<vm::Command> {
        let len = s.encode_utf16().count() as u16;

        let mut cmds = vec![
            vm::Command::Push(vm::Segment::Constant, len),
            vm::Command::Call("String.new".to_owned(), 1),
        ];

        for ch in s.encode_utf16() {
            cmds.extend(vec![
                vm::Command::Push(vm::Segment::Constant, ch),
                vm::Command::Call("String.appendChar".to_owned(), 2),
            ]);
        }

        cmds
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
            KeywordConst::This => vec![vm::Command::Push(vm::Segment::Pointer, 0)],
        }
    }

    fn compile_var<F>(&self, f: F, name: &str) -> vm::Command
    where
        F: Fn(vm::Segment, u16) -> vm::Command,
    {
        let symbol = self.symbols.get(name).expect("undefined symbol");

        let segment = match symbol.kind {
            Kind::Argument => vm::Segment::Argument,
            Kind::LocalVar => vm::Segment::Local,
            Kind::Field => vm::Segment::This,
            Kind::Static => vm::Segment::Static,
        };

        f(segment, symbol.index)
    }

    fn compile_indexed_var(&self, name: &str, expr: &Expr) -> Vec<vm::Command> {
        let mut cmds = vec![self.compile_var(vm::Command::Push, name)];
        cmds.extend(self.compile_expr(expr));
        cmds.extend(vec![
            vm::Command::Add,
            vm::Command::Pop(vm::Segment::Pointer, 1),
            vm::Command::Push(vm::Segment::That, 0),
        ]);
        cmds
    }

    fn compile_unary(&self, op: UnaryOp, term: &Term) -> Vec<vm::Command> {
        let mut cmds = self.compile_term(term);
        cmds.push(self.compile_unary_op(op));
        cmds
    }

    fn compile_binary_op(&self, op: BinaryOp) -> vm::Command {
        match op {
            BinaryOp::Add => vm::Command::Add,
            BinaryOp::Subtract => vm::Command::Sub,
            BinaryOp::Multiply => vm::Command::Call("Math.multiply".to_owned(), 2),
            BinaryOp::Divide => vm::Command::Call("Math.divide".to_owned(), 2),
            BinaryOp::And => vm::Command::And,
            BinaryOp::Or => vm::Command::Or,
            BinaryOp::LessThan => vm::Command::Lt,
            BinaryOp::GreaterThan => vm::Command::Gt,
            BinaryOp::Equal => vm::Command::Eq,
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
