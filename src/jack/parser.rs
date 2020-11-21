use super::tokenizer::{Keyword, Token};
use anyhow::{anyhow, Error};
use std::convert::{TryFrom, TryInto};
use std::iter::Peekable;
use std::vec::IntoIter;

#[derive(Debug, Eq, PartialEq)]
pub struct Class {
    pub name: String,
    pub vars: Vec<ClassVars>,
    pub subs: Vec<Subroutine>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct ClassVars {
    pub kind: ClassVarKind,
    pub typ: VarType,
    pub names: Vec<String>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Subroutine {
    pub kind: SubroutineKind,
    pub typ: SubroutineType,
    pub name: String,
    pub params: Vec<Param>,
    pub body: SubroutineBody,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Param {
    pub typ: VarType,
    pub name: String,
}

#[derive(Debug, Eq, PartialEq)]
pub struct SubroutineBody {
    pub vars: Vec<LocalVars>,
    pub statements: Vec<Statement>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct LocalVars {
    pub typ: VarType,
    pub names: Vec<String>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct SubroutineCall {
    pub receiver: Option<String>,
    pub subroutine: String,
    pub args: Vec<Expr>,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ClassVarKind {
    Field,
    Static,
}

impl TryFrom<Keyword> for ClassVarKind {
    type Error = Error;

    fn try_from(keyword: Keyword) -> Result<Self, Self::Error> {
        match keyword {
            Keyword::Field => Ok(ClassVarKind::Field),
            Keyword::Static => Ok(ClassVarKind::Static),
            _ => Err(anyhow!(
                "expected either `field` or `static`, found `{}`",
                keyword
            )),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum VarType {
    Int,
    Char,
    Boolean,
    ClassName(String),
}

impl TryFrom<Token> for VarType {
    type Error = Error;

    fn try_from(token: Token) -> Result<Self, Self::Error> {
        match token {
            Token::Keyword(Keyword::Int) => Ok(VarType::Int),
            Token::Keyword(Keyword::Char) => Ok(VarType::Char),
            Token::Keyword(Keyword::Boolean) => Ok(VarType::Boolean),
            Token::Identifier(id) => Ok(VarType::ClassName(id)),
            _ => Err(anyhow!(
                "expected one of `int`, `char`, `boolean`, or a class name, found `{}`",
                token
            )),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SubroutineKind {
    Constructor,
    Function,
    Method,
}

impl TryFrom<Keyword> for SubroutineKind {
    type Error = Error;

    fn try_from(keyword: Keyword) -> Result<Self, Self::Error> {
        match keyword {
            Keyword::Constructor => Ok(SubroutineKind::Constructor),
            Keyword::Function => Ok(SubroutineKind::Function),
            Keyword::Method => Ok(SubroutineKind::Method),
            _ => Err(anyhow!(
                "expected one of `constructor`, `function`, `method`, found `{}`",
                keyword
            )),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum SubroutineType {
    Void,
    NonVoid(VarType),
}

impl TryFrom<Token> for SubroutineType {
    type Error = Error;

    fn try_from(token: Token) -> Result<Self, Self::Error> {
        match token {
            Token::Keyword(Keyword::Void) => Ok(SubroutineType::Void),
            _ => token.try_into().map(SubroutineType::NonVoid),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Statement {
    Let {
        lhs: String,
        index: Option<Expr>,
        rhs: Expr,
    },
    If {
        condition: Expr,
        if_body: Vec<Statement>,
        else_body: Option<Vec<Statement>>,
    },
    While {
        condition: Expr,
        body: Vec<Statement>,
    },
    Do(SubroutineCall),
    Return(Option<Expr>),
}

#[derive(Debug, Eq, PartialEq)]
pub enum Term {
    IntConst(u16),
    StrConst(String),
    KeywordConst(KeywordConst),
    Var(String),
    IndexedVar(String, Box<Expr>),
    SubroutineCall(SubroutineCall),
    Bracketed(Box<Expr>),
    Unary(UnaryOp, Box<Term>),
}

#[derive(Debug, Eq, PartialEq)]
pub enum Expr {
    Term(Term),
    Binary(BinaryOp, Term, Box<Expr>),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum KeywordConst {
    True,
    False,
    Null,
    This,
}

impl TryFrom<Keyword> for KeywordConst {
    type Error = Error;

    fn try_from(keyword: Keyword) -> Result<Self, Self::Error> {
        match keyword {
            Keyword::True => Ok(KeywordConst::True),
            Keyword::False => Ok(KeywordConst::False),
            Keyword::Null => Ok(KeywordConst::Null),
            Keyword::This => Ok(KeywordConst::This),
            _ => Err(anyhow!(
                "expected one of `true`, `false`, `null`, or `this`, found `{}`",
                keyword
            )),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum UnaryOp {
    Minus,
    Not,
}

impl TryFrom<Token> for UnaryOp {
    type Error = Error;

    fn try_from(token: Token) -> Result<Self, Self::Error> {
        match token {
            Token::Symbol('-') => Ok(UnaryOp::Minus),
            Token::Symbol('~') => Ok(UnaryOp::Not),
            _ => Err(anyhow!("`{}` is not a valid unary operator", token)),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    And,
    Or,
    LessThan,
    GreaterThan,
    Equal,
}

impl TryFrom<Token> for BinaryOp {
    type Error = Error;

    fn try_from(token: Token) -> Result<Self, Self::Error> {
        match token {
            Token::Symbol('+') => Ok(BinaryOp::Add),
            Token::Symbol('-') => Ok(BinaryOp::Subtract),
            Token::Symbol('*') => Ok(BinaryOp::Multiply),
            Token::Symbol('/') => Ok(BinaryOp::Divide),
            Token::Symbol('&') => Ok(BinaryOp::And),
            Token::Symbol('|') => Ok(BinaryOp::Or),
            Token::Symbol('<') => Ok(BinaryOp::LessThan),
            Token::Symbol('>') => Ok(BinaryOp::GreaterThan),
            Token::Symbol('=') => Ok(BinaryOp::Equal),
            _ => Err(anyhow!("`{}` is not a valid binary operator", token)),
        }
    }
}

pub struct Parser {
    tokens: Peekable<IntoIter<Token>>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        let tokens = tokens.into_iter().peekable();
        Parser { tokens }
    }

    pub fn parse(&mut self) -> Result<Class, Error> {
        self.expect(&Token::Keyword(Keyword::Class))?;
        let name = self.consume_identifier()?;

        self.expect_symbol('{')?;
        let vars = self.parse_class_vars()?;
        let subs = self.parse_subroutines()?;
        self.expect_symbol('}')?;

        Ok(Class { name, vars, subs })
    }

    fn parse_class_vars(&mut self) -> Result<Vec<ClassVars>, Error> {
        let mut vars = Vec::new();

        while let Token::Keyword(Keyword::Field) | Token::Keyword(Keyword::Static) = self.peek()? {
            let kind = self.parse_from_keyword()?;
            let typ = self.parse_from_token()?;
            let names = self.parse_identifiers_list()?;
            self.expect_symbol(';')?;

            vars.push(ClassVars { kind, typ, names });
        }

        Ok(vars)
    }

    fn parse_identifiers_list(&mut self) -> Result<Vec<String>, Error> {
        let mut names = vec![self.consume_identifier()?];

        while self.peek_symbol(';').is_none() {
            self.expect_symbol(',')?;
            names.push(self.consume_identifier()?);
        }

        Ok(names)
    }

    fn parse_subroutines(&mut self) -> Result<Vec<Subroutine>, Error> {
        let mut subs = Vec::new();

        while self.peek_symbol('}').is_none() {
            let kind = self.parse_from_keyword()?;
            let typ = self.parse_from_token()?;
            let name = self.consume_identifier()?;

            self.expect_symbol('(')?;
            let params = self.parse_params()?;
            self.expect_symbol(')')?;

            self.expect_symbol('{')?;
            let body = self.parse_subroutine_body()?;
            self.expect_symbol('}')?;

            subs.push(Subroutine {
                kind,
                typ,
                name,
                params,
                body,
            });
        }

        Ok(subs)
    }

    fn parse_params(&mut self) -> Result<Vec<Param>, Error> {
        let mut params = Vec::new();

        if self.peek_symbol(')').is_none() {
            loop {
                let typ = self.parse_from_token()?;
                let name = self.consume_identifier()?;
                params.push(Param { typ, name });

                match self.peek()? {
                    Token::Symbol(',') => self.consume()?,
                    Token::Symbol(')') => break,
                    token => return Err(anyhow!("expected either `,` or `)`, found `{}`", token)),
                };
            }
        }

        Ok(params)
    }

    fn parse_subroutine_body(&mut self) -> Result<SubroutineBody, Error> {
        let vars = self.parse_local_vars()?;
        let statements = self.parse_statements()?;
        Ok(SubroutineBody { vars, statements })
    }

    fn parse_local_vars(&mut self) -> Result<Vec<LocalVars>, Error> {
        let mut vars = Vec::new();

        while let Token::Keyword(Keyword::Var) = self.peek()? {
            self.consume()?;

            let typ = self.parse_from_token()?;
            let names = self.parse_identifiers_list()?;
            self.expect_symbol(';')?;

            vars.push(LocalVars { typ, names });
        }

        Ok(vars)
    }

    fn parse_block(&mut self) -> Result<Vec<Statement>, Error> {
        self.expect_symbol('{')?;
        let block = self.parse_statements()?;
        self.expect_symbol('}')?;

        Ok(block)
    }

    fn parse_statements(&mut self) -> Result<Vec<Statement>, Error> {
        let mut statements = Vec::new();

        while self.peek_symbol('}').is_none() {
            statements.push(self.parse_statement()?);
        }

        Ok(statements)
    }

    fn parse_statement(&mut self) -> Result<Statement, Error> {
        match self.consume_keyword()? {
            Keyword::Let => self.parse_let_statement(),
            Keyword::If => self.parse_if_statement(),
            Keyword::While => self.parse_while_statement(),
            Keyword::Do => self.parse_do_statement(),
            Keyword::Return => self.parse_return_statement(),
            keyword => Err(anyhow!(
                "expected one of `let`, `if`, `while`, `do`, or `return`, found `{}`",
                keyword
            )),
        }
    }

    fn parse_let_statement(&mut self) -> Result<Statement, Error> {
        let lhs = self.consume_identifier()?;

        let index = match self.peek()? {
            Token::Symbol('[') => Some(self.parse_index_expr()?),
            _ => None,
        };

        self.expect_symbol('=')?;
        let rhs = self.parse_expr()?;
        self.expect_symbol(';')?;

        Ok(Statement::Let { lhs, index, rhs })
    }

    fn parse_if_statement(&mut self) -> Result<Statement, Error> {
        self.expect_symbol('(')?;
        let condition = self.parse_expr()?;
        self.expect_symbol(')')?;

        let if_body = self.parse_block()?;

        let else_body = match self.peek()? {
            Token::Keyword(Keyword::Else) => {
                self.consume()?;
                Some(self.parse_block()?)
            }
            _ => None,
        };

        Ok(Statement::If {
            condition,
            if_body,
            else_body,
        })
    }

    fn parse_while_statement(&mut self) -> Result<Statement, Error> {
        self.expect_symbol('(')?;
        let condition = self.parse_expr()?;
        self.expect_symbol(')')?;
        let body = self.parse_block()?;

        Ok(Statement::While { condition, body })
    }

    fn parse_do_statement(&mut self) -> Result<Statement, Error> {
        let first = self.consume_identifier()?;
        let call = self.parse_subroutine_call(first)?;
        self.expect_symbol(';')?;

        Ok(Statement::Do(call))
    }

    fn parse_return_statement(&mut self) -> Result<Statement, Error> {
        let expr = match self.peek()? {
            Token::Symbol(';') => None,
            _ => Some(self.parse_expr()?),
        };
        self.expect_symbol(';')?;

        Ok(Statement::Return(expr))
    }

    fn parse_subroutine_call(&mut self, first: String) -> Result<SubroutineCall, Error> {
        let (receiver, subroutine) = match self.peek()? {
            Token::Symbol('.') => {
                self.consume()?;
                (Some(first), self.consume_identifier()?)
            }
            _ => (None, first),
        };

        self.expect_symbol('(')?;
        let args = self.parse_expr_list()?;
        self.expect_symbol(')')?;

        Ok(SubroutineCall {
            receiver,
            subroutine,
            args,
        })
    }

    fn parse_expr_list(&mut self) -> Result<Vec<Expr>, Error> {
        let mut exprs = Vec::new();

        if self.peek_symbol(')').is_none() {
            loop {
                exprs.push(self.parse_expr()?);

                match self.peek()? {
                    Token::Symbol(',') => self.consume()?,
                    Token::Symbol(')') => break,
                    token => return Err(anyhow!("expected either `,` or `)`, found `{}`", token)),
                };
            }
        }

        Ok(exprs)
    }

    fn parse_expr(&mut self) -> Result<Expr, Error> {
        let term = self.parse_term()?;

        let expr = match self.peek()? {
            Token::Symbol('+')
            | Token::Symbol('-')
            | Token::Symbol('*')
            | Token::Symbol('/')
            | Token::Symbol('&')
            | Token::Symbol('|')
            | Token::Symbol('<')
            | Token::Symbol('>')
            | Token::Symbol('=') => {
                let op = self.parse_from_token()?;
                let rhs = self.parse_expr()?;
                Expr::Binary(op, term, Box::new(rhs))
            }
            _ => Expr::Term(term),
        };

        Ok(expr)
    }

    fn parse_term(&mut self) -> Result<Term, Error> {
        match self.consume()? {
            Token::IntConst(n) => Ok(Term::IntConst(n)),
            Token::StrConst(s) => Ok(Term::StrConst(s)),
            Token::Keyword(kw) => kw.try_into().map(Term::KeywordConst),
            Token::Identifier(id) => match self.peek()? {
                Token::Symbol('.') | Token::Symbol('(') => {
                    self.parse_subroutine_call(id).map(Term::SubroutineCall)
                }
                Token::Symbol('[') => self
                    .parse_index_expr()
                    .map(|index| Term::IndexedVar(id, Box::new(index))),
                _ => Ok(Term::Var(id)),
            },
            Token::Symbol('(') => {
                let expr = self.parse_expr()?;
                self.expect_symbol(')')?;
                Ok(Term::Bracketed(Box::new(expr)))
            }
            token @ Token::Symbol(_) => {
                let op = token.try_into()?;
                let term = self.parse_term()?;
                Ok(Term::Unary(op, Box::new(term)))
            }
        }
    }

    fn parse_index_expr(&mut self) -> Result<Expr, Error> {
        self.expect_symbol('[')?;
        let index = self.parse_expr()?;
        self.expect_symbol(']')?;

        Ok(index)
    }

    fn parse_from_keyword<T>(&mut self) -> Result<T, Error>
    where
        T: TryFrom<Keyword, Error = Error>,
    {
        self.consume_keyword().and_then(|kw| kw.try_into())
    }

    fn parse_from_token<T>(&mut self) -> Result<T, Error>
    where
        T: TryFrom<Token, Error = Error>,
    {
        self.consume().and_then(|t| t.try_into())
    }

    fn expect_symbol(&mut self, want: char) -> Result<Token, Error> {
        self.expect(&Token::Symbol(want))
    }

    fn expect(&mut self, want: &Token) -> Result<Token, Error> {
        self.consume().and_then(|token| {
            if token == *want {
                Ok(token)
            } else {
                Err(anyhow!("expected `{}`, found `{}`", want, token))
            }
        })
    }

    fn consume_keyword(&mut self) -> Result<Keyword, Error> {
        self.consume().and_then(|token| match token {
            Token::Keyword(keyword) => Ok(keyword),
            _ => Err(anyhow!("expected a keyword, found `{}`", token)),
        })
    }

    fn consume_identifier(&mut self) -> Result<String, Error> {
        self.consume().and_then(|token| match token {
            Token::Identifier(id) => Ok(id),
            _ => Err(anyhow!("expected an identifier, found `{}`", token)),
        })
    }

    fn consume(&mut self) -> Result<Token, Error> {
        self.tokens
            .next()
            .ok_or_else(|| anyhow!("unexpected end of file"))
    }

    fn peek_symbol(&mut self, want: char) -> Option<&Token> {
        self.peek()
            .ok()
            .filter(|&token| token == &Token::Symbol(want))
    }

    fn peek(&mut self) -> Result<&Token, Error> {
        self.tokens
            .peek()
            .ok_or_else(|| anyhow!("unexpected end of file"))
    }
}
