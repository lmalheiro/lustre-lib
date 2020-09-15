//#![allow(unused)]
use rustf8::{Utf8Iterator, Utf8IteratorError::*};
use std::fmt::Debug;
use std::io::Error;

pub enum Token {
    NoToken,
    Identifier(String),
    Integer(String),
    OpenList,
    CloseList,
    Symbol(String),
    Invalid(String),
}
enum State {
    Begin,
    DecodingIdentifier,
    DecodingInteger,
    FinishedToken,
    Invalid,
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        use Token::*;
        match (self, other) {
            (NoToken, NoToken) => true,
            (OpenList, OpenList) => true,
            (CloseList, CloseList) => true,
            (Identifier(a), Identifier(b)) => a == b,
            (Integer(a), Integer(b)) => a == b,
            (Symbol(a), Symbol(b)) => a == b,
            (Invalid(a), Invalid(b)) => a == b,
            (_, _) => false,
        }
    }
}
impl Clone for Token {
    fn clone(&self) -> Self {
        use Token::*;
        match self {
            NoToken => NoToken,
            OpenList => OpenList,
            CloseList => CloseList,
            Identifier(a) => Identifier(a.to_string()),
            Integer(a) => Integer(a.to_string()),
            Symbol(a) => Symbol(a.to_string()),
            Invalid(a) => Invalid(a.to_string()),
        }
    }
}
impl Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        use Token::*;
        match self {
            NoToken => f.debug_struct("None").finish(),
            OpenList => f.debug_struct("OpenList").finish(),
            CloseList => f.debug_struct("CloseList").finish(),
            Identifier(a) => f.debug_struct("Identifier").field("string", a).finish(),
            Integer(a) => f.debug_struct("Integer").field("string", a).finish(),
            Symbol(a) => f.debug_struct("Symbol").field("string", a).finish(),
            Invalid(a) => f.debug_struct("Invalid").field("string", a).finish(),
        }
    }
}

struct Tokenizer<T>
where
    T: Iterator,
{
    chiter: Utf8Iterator<T>,
    state: (State, Token),
}

impl<T> Tokenizer<T>
where
    T: Iterator<Item = Result<u8, Error>>,
{
    pub fn new(iter: T) -> Self {
        Tokenizer {
            chiter: Utf8Iterator::<T>::new(iter),
            state: (State::Begin, Token::NoToken),
        }
    }
    fn state_machine(&mut self, ch: char) {
        self.state = match &self.state {
            (State::Invalid, _) | (State::FinishedToken, _) | (State::Begin, _) => {
                if ch == '(' {
                    (State::FinishedToken, Token::OpenList)
                } else if ch == ')' {
                    (State::FinishedToken, Token::CloseList)
                } else if ch.is_whitespace() {
                    (State::Begin, Token::NoToken)
                } else if ch.is_alphabetic() || ch == '_' {
                    (State::DecodingIdentifier, Token::Identifier(ch.to_string()))
                } else if ch.is_numeric() {
                    (State::DecodingInteger, Token::Integer(ch.to_string()))
                } else if ch.is_ascii_punctuation() {
                    (State::FinishedToken, Token::Symbol(ch.to_string()))
                } else {
                    (State::Invalid, Token::Invalid(ch.to_string()))
                }
            }
            (State::DecodingIdentifier, Token::Identifier(id)) => {
                if ch.is_whitespace() {
                    (State::FinishedToken, Token::Identifier(id.to_string()))
                } else if ch.is_alphanumeric() || ch == '_' {
                    (
                        State::DecodingIdentifier,
                        Token::Identifier(id.to_string() + &ch.to_string()),
                    )
                } else {
                    self.chiter.unget(ch);
                    (State::FinishedToken, Token::Identifier(id.to_string()))
                }
            }
            (State::DecodingInteger, Token::Integer(num)) => {
                if ch.is_whitespace() {
                    (State::FinishedToken, Token::Integer(num.to_string()))
                } else if ch.is_digit(10) {
                    (
                        State::DecodingIdentifier,
                        Token::Integer(num.to_string() + &ch.to_string()),
                    )
                } else {
                    self.chiter.unget(ch);
                    (
                        State::FinishedToken,
                        Token::Integer(num.to_string() + &ch.to_string()),
                    )
                }
            }
            (_, _) => panic!("Inconsistent state!"),
        }
    }

    pub fn next(&mut self) -> Option<Token> {
        loop {
            let r = self.chiter.next();
            match r {
                Some(item) => match item {
                    Ok(ch) => {
                        self.state_machine(ch);
                        if let State::FinishedToken = self.state.0 {
                            return Some(self.state.1.clone());
                        }
                    }
                    Err(e) => match e {
                        InvalidSequenceError(bytes) => {
                            panic!("Detected an invalid UTF-8 sequence! {:?}", bytes)
                        }
                        LongSequenceError(bytes) => {
                            panic!("UTF-8 sequence with more tha 4 bytes! {:?}", bytes)
                        }
                        InvalidCharError(bytes) => panic!(
                            "UTF-8 sequence resulted in an invalid character! {:?}",
                            bytes
                        ),
                        IoError(ioe, bytes) => panic!(
                            "I/O error {:?} while decoding de sequence {:?} !",
                            ioe, bytes
                        ),
                    },
                },
                None => {
                    return None
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::prelude::*;
    use std::io::Cursor;

    #[test]
    fn get_tokens() {

        macro_rules! s {
            ($value:expr) => {
                String::from($value)
            };
        }

        use Token::*;
        let input = "(defun κόσμε (x y) (+ x y))";
        let mut tokens = Tokenizer::new(Cursor::new(input).bytes());

        let mut tokenized = Vec::<Token>::new();

        while let Some(token) = tokens.next() {
            eprintln!("{:?}", token);
            tokenized.push(token);
        }

        let cmp: Vec<Token> = vec![
            OpenList,
            Identifier(s!("defun")),
            Identifier(s!("κόσμε")),
            OpenList,
            Identifier(s!("x")),
            Identifier(s!("y")),
            CloseList,
            OpenList,
            Symbol(s!("+")),
            Identifier(s!("x")),
            Identifier(s!("y")),
            CloseList,
            CloseList
        ];

        assert_eq!(cmp, tokenized);
    }
}
