use super::tokenizer::{Keyword, Token, Tokenizer};
use std::convert::{TryFrom, TryInto};
use std::iter::Peekable;

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
    type Error = ();

    fn try_from(keyword: Keyword) -> Result<Self, Self::Error> {
        match keyword {
            Keyword::Field => Ok(ClassVarKind::Field),
            Keyword::Static => Ok(ClassVarKind::Static),
            _ => Err(()),
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
    type Error = ();

    fn try_from(token: Token) -> Result<Self, Self::Error> {
        match token {
            Token::Keyword(Keyword::Int) => Ok(VarType::Int),
            Token::Keyword(Keyword::Char) => Ok(VarType::Char),
            Token::Keyword(Keyword::Boolean) => Ok(VarType::Boolean),
            Token::Identifier(id) => Ok(VarType::ClassName(id)),
            _ => Err(()),
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
    type Error = ();

    fn try_from(keyword: Keyword) -> Result<Self, Self::Error> {
        match keyword {
            Keyword::Constructor => Ok(SubroutineKind::Constructor),
            Keyword::Function => Ok(SubroutineKind::Function),
            Keyword::Method => Ok(SubroutineKind::Method),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum SubroutineType {
    Void,
    NonVoid(VarType),
}

impl TryFrom<Token> for SubroutineType {
    type Error = ();

    fn try_from(token: Token) -> Result<Self, Self::Error> {
        match token {
            Token::Keyword(Keyword::Void) => Ok(SubroutineType::Void),
            _ => token
                .try_into()
                .map(|var_type| SubroutineType::NonVoid(var_type)),
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
    type Error = ();

    fn try_from(keyword: Keyword) -> Result<Self, Self::Error> {
        match keyword {
            Keyword::True => Ok(KeywordConst::True),
            Keyword::False => Ok(KeywordConst::False),
            Keyword::Null => Ok(KeywordConst::Null),
            Keyword::This => Ok(KeywordConst::This),
            _ => Err(()),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum UnaryOp {
    Minus,
    Not,
}

impl TryFrom<Token> for UnaryOp {
    type Error = ();

    fn try_from(token: Token) -> Result<Self, Self::Error> {
        match token {
            Token::Symbol('-') => Ok(UnaryOp::Minus),
            Token::Symbol('~') => Ok(UnaryOp::Not),
            _ => Err(()),
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
    type Error = ();

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
            _ => Err(()),
        }
    }
}

pub struct Parser<'a> {
    tokens: Peekable<Tokenizer<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(tokenizer: Tokenizer<'a>) -> Self {
        let tokens = tokenizer.peekable();
        Parser { tokens }
    }

    pub fn parse(&mut self) -> Option<Class> {
        self.expect(&Token::Keyword(Keyword::Class))?;
        let name = self.consume_identifier()?;

        self.expect_symbol('{')?;
        let vars = self.parse_class_vars()?;
        let subs = self.parse_subroutines()?;
        self.expect_symbol('}')?;

        Some(Class { name, vars, subs })
    }

    fn parse_class_vars(&mut self) -> Option<Vec<ClassVars>> {
        let mut vars = Vec::new();

        loop {
            match self.peek()? {
                Token::Keyword(Keyword::Field) | Token::Keyword(Keyword::Static) => {
                    let kind = self.parse_from_keyword()?;
                    let typ = self.parse_from_token()?;
                    let names = self.parse_identifiers_list()?;
                    self.expect_symbol(';')?;

                    vars.push(ClassVars { kind, typ, names });
                }
                _ => break,
            }
        }

        Some(vars)
    }

    fn parse_identifiers_list(&mut self) -> Option<Vec<String>> {
        let mut names = vec![self.consume_identifier()?];

        while self.peek_symbol(';').is_none() {
            self.expect_symbol(',')?;
            names.push(self.consume_identifier()?);
        }

        Some(names)
    }

    fn parse_subroutines(&mut self) -> Option<Vec<Subroutine>> {
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

        Some(subs)
    }

    fn parse_params(&mut self) -> Option<Vec<Param>> {
        let mut params = Vec::new();

        if self.peek_symbol(')').is_none() {
            loop {
                let typ = self.parse_from_token()?;
                let name = self.consume_identifier()?;
                params.push(Param { typ, name });

                match self.peek()? {
                    Token::Symbol(',') => self.consume()?,
                    Token::Symbol(')') => break,
                    _ => return None,
                };
            }
        }

        Some(params)
    }

    fn parse_subroutine_body(&mut self) -> Option<SubroutineBody> {
        let vars = self.parse_local_vars()?;
        let statements = self.parse_statements()?;
        Some(SubroutineBody { vars, statements })
    }

    fn parse_local_vars(&mut self) -> Option<Vec<LocalVars>> {
        let mut vars = Vec::new();

        loop {
            match self.peek()? {
                Token::Keyword(Keyword::Var) => {
                    self.consume()?;

                    let typ = self.parse_from_token()?;
                    let names = self.parse_identifiers_list()?;
                    self.expect_symbol(';')?;

                    vars.push(LocalVars { typ, names });
                }
                _ => break,
            }
        }

        Some(vars)
    }

    fn parse_block(&mut self) -> Option<Vec<Statement>> {
        self.expect_symbol('{')?;
        let block = self.parse_statements()?;
        self.expect_symbol('}')?;

        Some(block)
    }

    fn parse_statements(&mut self) -> Option<Vec<Statement>> {
        let mut statements = Vec::new();

        while self.peek_symbol('}').is_none() {
            statements.push(self.parse_statement()?);
        }

        Some(statements)
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        match self.consume_keyword()? {
            Keyword::Let => self.parse_let_statement(),
            Keyword::If => self.parse_if_statement(),
            Keyword::While => self.parse_while_statement(),
            Keyword::Do => self.parse_do_statement(),
            Keyword::Return => self.parse_return_statement(),
            _ => None,
        }
    }

    fn parse_let_statement(&mut self) -> Option<Statement> {
        let lhs = self.consume_identifier()?;

        let index = match self.peek()? {
            Token::Symbol('[') => self.parse_index_expr(), // TODO: propagate failure
            _ => None,
        };

        self.expect_symbol('=')?;
        let rhs = self.parse_expr()?;
        self.expect_symbol(';')?;

        Some(Statement::Let { lhs, index, rhs })
    }

    fn parse_if_statement(&mut self) -> Option<Statement> {
        self.expect_symbol('(')?;
        let condition = self.parse_expr()?;
        self.expect_symbol(')')?;

        let if_body = self.parse_block()?;

        let else_body = match self.peek()? {
            Token::Keyword(Keyword::Else) => {
                self.consume()?;
                self.parse_block() // TODO: propagate failure
            }
            _ => None,
        };

        Some(Statement::If {
            condition,
            if_body,
            else_body,
        })
    }

    fn parse_while_statement(&mut self) -> Option<Statement> {
        self.expect_symbol('(')?;
        let condition = self.parse_expr()?;
        self.expect_symbol(')')?;
        let body = self.parse_block()?;

        Some(Statement::While { condition, body })
    }

    fn parse_do_statement(&mut self) -> Option<Statement> {
        let first = self.consume_identifier()?;
        let call = self.parse_subroutine_call(first)?;
        self.expect_symbol(';')?;

        Some(Statement::Do(call))
    }

    fn parse_return_statement(&mut self) -> Option<Statement> {
        let expr = match self.peek()? {
            Token::Symbol(';') => None,
            _ => self.parse_expr(),
        };
        self.expect_symbol(';')?;

        Some(Statement::Return(expr))
    }

    fn parse_subroutine_call(&mut self, first: String) -> Option<SubroutineCall> {
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

        Some(SubroutineCall {
            receiver,
            subroutine,
            args,
        })
    }

    fn parse_expr_list(&mut self) -> Option<Vec<Expr>> {
        let mut exprs = Vec::new();

        if self.peek_symbol(')').is_none() {
            loop {
                exprs.push(self.parse_expr()?);

                match self.peek()? {
                    Token::Symbol(',') => self.consume(),
                    Token::Symbol(')') => break,
                    _ => return None,
                };
            }
        }

        Some(exprs)
    }

    fn parse_expr(&mut self) -> Option<Expr> {
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

        Some(expr)
    }

    fn parse_term(&mut self) -> Option<Term> {
        match self.consume()? {
            Token::IntConst(n) => Some(Term::IntConst(n)),
            Token::StrConst(s) => Some(Term::StrConst(s)),
            Token::Keyword(kw) => kw.try_into().ok().map(Term::KeywordConst),
            Token::Identifier(id) => match self.peek()? {
                Token::Symbol('.') | Token::Symbol('(') => {
                    self.parse_subroutine_call(id).map(Term::SubroutineCall)
                }
                Token::Symbol('[') => self
                    .parse_index_expr()
                    .map(|index| Term::IndexedVar(id, Box::new(index))),
                _ => Some(Term::Var(id)),
            },
            Token::Symbol('(') => {
                let expr = self.parse_expr()?;
                self.expect_symbol(')')?;
                Some(Term::Bracketed(Box::new(expr)))
            }
            token @ Token::Symbol(_) => {
                let op = token.try_into().ok()?;
                let term = self.parse_term()?;
                Some(Term::Unary(op, Box::new(term)))
            }
        }
    }

    fn parse_index_expr(&mut self) -> Option<Expr> {
        self.expect_symbol('[')?;
        let index = self.parse_expr()?;
        self.expect_symbol(']')?;

        Some(index)
    }

    fn parse_from_keyword<T>(&mut self) -> Option<T>
    where
        T: TryFrom<Keyword>,
    {
        self.consume_keyword().and_then(|kw| kw.try_into().ok())
    }

    fn parse_from_token<T>(&mut self) -> Option<T>
    where
        T: TryFrom<Token>,
    {
        self.consume().and_then(|t| t.try_into().ok())
    }

    fn expect_symbol(&mut self, want: char) -> Option<Token> {
        self.expect(&Token::Symbol(want))
    }

    fn expect(&mut self, want: &Token) -> Option<Token> {
        self.consume().filter(|token| token == want)
    }

    fn consume_keyword(&mut self) -> Option<Keyword> {
        self.consume().and_then(|token| match token {
            Token::Keyword(keyword) => Some(keyword),
            _ => None,
        })
    }

    fn consume_identifier(&mut self) -> Option<String> {
        self.consume().and_then(|token| match token {
            Token::Identifier(id) => Some(id),
            _ => None,
        })
    }

    fn consume(&mut self) -> Option<Token> {
        self.tokens.next()
    }

    fn peek_symbol(&mut self, want: char) -> Option<&Token> {
        self.peek().filter(|&token| token == &Token::Symbol(want))
    }

    fn peek(&mut self) -> Option<&Token> {
        self.tokens.peek()
    }
}
