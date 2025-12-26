use anyhow::{anyhow, Error};
use std::fmt;
use std::iter::Peekable;
use std::str::{Chars, FromStr};

#[derive(Debug, Eq, PartialEq)]
pub enum Token {
    Keyword(Keyword),
    Identifier(String),
    IntConst(u16),
    StrConst(String),
    Symbol(char),
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Keyword(keyword) => write!(f, "{}", keyword),
            Token::Identifier(s) => write!(f, "{}", s),
            Token::IntConst(n) => write!(f, "{}", n),
            // This is safe because Jack string literals can't contain escaped double quotes.
            Token::StrConst(s) => write!(f, "\"{}\"", s),
            Token::Symbol(ch) => write!(f, "{}", ch),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Keyword {
    Boolean,
    Char,
    Class,
    Constructor,
    Do,
    Else,
    False,
    Field,
    Function,
    If,
    Int,
    Let,
    Method,
    Null,
    Return,
    Static,
    This,
    True,
    Var,
    Void,
    While,
}

impl fmt::Display for Keyword {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Keyword::Boolean => write!(f, "boolean"),
            Keyword::Char => write!(f, "char"),
            Keyword::Class => write!(f, "class"),
            Keyword::Constructor => write!(f, "constructor"),
            Keyword::Do => write!(f, "do"),
            Keyword::Else => write!(f, "else"),
            Keyword::False => write!(f, "false"),
            Keyword::Field => write!(f, "field"),
            Keyword::Function => write!(f, "function"),
            Keyword::If => write!(f, "if"),
            Keyword::Int => write!(f, "int"),
            Keyword::Let => write!(f, "let"),
            Keyword::Method => write!(f, "method"),
            Keyword::Null => write!(f, "null"),
            Keyword::Return => write!(f, "return"),
            Keyword::Static => write!(f, "static"),
            Keyword::This => write!(f, "this"),
            Keyword::True => write!(f, "true"),
            Keyword::Var => write!(f, "var"),
            Keyword::Void => write!(f, "void"),
            Keyword::While => write!(f, "while"),
        }
    }
}

impl FromStr for Keyword {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "boolean" => Ok(Keyword::Boolean),
            "char" => Ok(Keyword::Char),
            "class" => Ok(Keyword::Class),
            "constructor" => Ok(Keyword::Constructor),
            "do" => Ok(Keyword::Do),
            "else" => Ok(Keyword::Else),
            "false" => Ok(Keyword::False),
            "field" => Ok(Keyword::Field),
            "function" => Ok(Keyword::Function),
            "if" => Ok(Keyword::If),
            "int" => Ok(Keyword::Int),
            "let" => Ok(Keyword::Let),
            "method" => Ok(Keyword::Method),
            "null" => Ok(Keyword::Null),
            "return" => Ok(Keyword::Return),
            "static" => Ok(Keyword::Static),
            "this" => Ok(Keyword::This),
            "true" => Ok(Keyword::True),
            "var" => Ok(Keyword::Var),
            "void" => Ok(Keyword::Void),
            "while" => Ok(Keyword::While),
            _ => Err(anyhow!("`{}` is not a valid keyword", s)),
        }
    }
}

pub struct Tokenizer<'a> {
    chars: Peekable<Chars<'a>>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &'a str) -> Self {
        let chars = input.chars().peekable();
        Tokenizer { chars }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, Error> {
        self.collect()
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Result<Token, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespace();

        self.read_char().and_then(|ch| match ch {
            '/' => {
                if self.skip_comment() {
                    // If we've skipped a comment, call next() recursively to move on to the
                    // next token and pretend the comment wasn't there.
                    self.next()
                } else {
                    Some(Ok(Token::Symbol(ch)))
                }
            }
            '"' => self.read_str_const().map(Token::StrConst).map(Ok),
            _ if is_symbol(ch) => Some(Ok(Token::Symbol(ch))),
            _ if ch.is_ascii_digit() => self.read_int_const(ch).map(Token::IntConst).map(Ok),
            _ if is_identifier(ch) => self.read_word(ch).map(parse_keyword_or_identifier).map(Ok),
            _ => Some(Err(anyhow!("`{}` is not a valid token", ch))),
        })
    }
}

impl<'a> Tokenizer<'a> {
    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.peek_char() {
            if ch.is_whitespace() {
                self.advance();
            } else {
                return;
            }
        }
    }

    fn skip_comment(&mut self) -> bool {
        match self.peek_char() {
            Some('/') => {
                self.skip_line();
                true
            }
            Some('*') => {
                self.skip_block_comment();
                true
            }
            _ => false,
        }
    }

    fn skip_line(&mut self) {
        while let Some(ch) = self.read_char() {
            if ch == '\n' {
                return;
            }
        }
    }

    fn skip_block_comment(&mut self) {
        let mut star = false;

        while let Some(ch) = self.read_char() {
            match ch {
                '/' if star => return,
                '*' => star = true,
                _ => star = false,
            }
        }
    }

    fn read_str_const(&mut self) -> Option<String> {
        let mut string = String::new();

        while let Some(ch) = self.read_char() {
            match ch {
                '"' => return Some(string),
                _ => string.push(ch),
            }
        }

        None
    }

    fn read_word(&mut self, first: char) -> Option<String> {
        // Identifiers may contain digits, as long as they're not the first
        // character in the word.
        self.read_while(first, |ch| is_identifier(ch) || ch.is_ascii_digit())
    }

    fn read_int_const(&mut self, first: char) -> Option<u16> {
        self.read_while(first, |ch| ch.is_ascii_digit())
            .and_then(|num| num.parse().ok())
    }

    fn read_while<P>(&mut self, first: char, pred: P) -> Option<String>
    where
        P: Fn(char) -> bool,
    {
        let mut string = first.to_string();

        while let Some(ch) = self.peek_char() {
            if pred(ch) {
                string.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        Some(string)
    }

    fn peek_char(&mut self) -> Option<char> {
        self.chars.peek().copied()
    }

    fn advance(&mut self) {
        self.read_char().expect("unexpected end of file");
    }

    fn read_char(&mut self) -> Option<char> {
        self.chars.next()
    }
}

const ALL_SYMBOLS: &[char] = &[
    '{', '}', '(', ')', '[', ']', '.', ',', ';', '+', '-', '*', '/', '&', ',', '|', '<', '>', '=',
    '~',
];

fn is_symbol(ch: char) -> bool {
    ALL_SYMBOLS.contains(&ch)
}

fn is_identifier(ch: char) -> bool {
    ch.is_ascii_alphabetic() || ch == '_'
}

fn parse_keyword_or_identifier(word: String) -> Token {
    word.parse::<Keyword>()
        .map_or(Token::Identifier(word), Token::Keyword)
}
