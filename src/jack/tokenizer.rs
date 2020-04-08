use std::iter::Peekable;
use std::str::{Chars, FromStr};

#[derive(Debug, Eq, PartialEq)]
pub enum Token {
    Keyword(Keyword),
    Identifier(String),
    IntConst(i16),
    StrConst(String),
    Symbol(char),
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

impl FromStr for Keyword {
    type Err = ();

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
            _ => Err(()),
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
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespace();

        self.read_char().and_then(|ch| match ch {
            '/' => {
                if self.skip_comment() {
                    // If we've skipped a comment, call next() recursively to move on to the
                    // next token and pretend the comment wasn't there.
                    self.next()
                } else {
                    Some(Token::Symbol(ch))
                }
            }
            '"' => self.read_str_const().map(Token::StrConst),
            _ if is_symbol(ch) => Some(Token::Symbol(ch)),
            _ if ch.is_ascii_digit() => self.read_int_const(ch).map(Token::IntConst),
            _ if is_identifier(ch) => self.read_word(ch).map(parse_keyword_or_identifier),
            _ => None,
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
        self.read_while(first, is_identifier)
    }

    fn read_int_const(&mut self, first: char) -> Option<i16> {
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
        self.read_char().expect("couldn't advance: EOF reached");
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
