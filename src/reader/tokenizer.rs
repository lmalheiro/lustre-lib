//#![allow(unused)]
use anyhow::Result;
use rustf8::{Utf8Iterator, Utf8IteratorError::*};
use std::fmt::Debug;

pub enum Token {
    NoToken,
    Identifier(String),
    Integer(String),
    //Symbol(String),
    Text(String),
    Quote,
    Quasiquote,
    Unquote,
    OpenList,
    CloseList,
    Invalid(String),
}
enum State {
    Begin,
    DecodingIdentifier,
    DecodingInteger,
    DecodingText,
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
            (Quote, Quote) => true,
            (Quasiquote, Quasiquote) => true,
            (Unquote, Unquote) => true,
            (Text(a), Text(b)) => a == b,
            (Identifier(a), Identifier(b)) => a == b,
            (Integer(a), Integer(b)) => a == b,
            //(Symbol(a), Symbol(b)) => a == b,
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
            Quote => Quote,
            Quasiquote => Quasiquote,
            Unquote => Unquote,
            Text(a) => Text(a.to_string()),
            Identifier(a) => Identifier(a.to_string()),
            Integer(a) => Integer(a.to_string()),
            //Symbol(a) => Symbol(a.to_string()),
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
            Quote => f.debug_struct("Quote").finish(),
            Quasiquote => f.debug_struct("Quasiquote").finish(),
            Unquote => f.debug_struct("Unquote").finish(),
            Text(a) => f.debug_struct("Text").field("string", a).finish(),
            Identifier(a) => f.debug_struct("Identifier").field("string", a).finish(),
            Integer(a) => f.debug_struct("Integer").field("string", a).finish(),
            //Symbol(a) => f.debug_struct("Symbol").field("string", a).finish(),
            Invalid(a) => f.debug_struct("Invalid").field("string", a).finish(),
        }
    }
}

pub struct Tokenizer<T>
where
    T: Iterator,
{
    chiter: Utf8Iterator<T>,
    state: (State, Token),
    cache: Option<Token>,
}

impl<T> Tokenizer<T>
where
    T: Iterator<Item = Result<u8, std::io::Error>>,
{
    pub fn new(iter: T) -> Self {
        Tokenizer {
            chiter: Utf8Iterator::<T>::new(iter),
            state: (State::Begin, Token::NoToken),
            cache: None,
        }
    }
    pub fn putback(&mut self, tk: Token) {
        if self.cache.is_some() {
            panic!("Can't call 'putback()' twice before calling 'token()'.")
        }
        self.cache = Some(tk);
    }
    fn state_machine(&mut self, ch: Option<char>) {
        self.state = match ch {
            None => match &self.state {
                (State::Invalid, _) | (State::FinishedToken, _) | (State::Begin, _) => {
                    (State::FinishedToken, Token::NoToken)
                }
                (State::DecodingIdentifier, Token::Identifier(id)) => {
                    (State::FinishedToken, Token::Identifier(id.to_string()))
                }
                (State::DecodingInteger, Token::Integer(num)) => {
                    (State::FinishedToken, Token::Integer(num.to_string()))
                }
                (State::DecodingText, Token::Text(txt)) => {
                    (State::FinishedToken, Token::Text(txt.to_string()))
                }
                (State::DecodingIdentifier, _)
                | (State::DecodingInteger, _)
                | (State::DecodingText, _) => panic!("Inconsistent state!"),
            },
            Some(ch) => match &self.state {
                (State::Invalid, _) | (State::FinishedToken, _) | (State::Begin, _) => {
                    if ch == '(' {
                        (State::FinishedToken, Token::OpenList)
                    } else if ch == ')' {
                        (State::FinishedToken, Token::CloseList)
                    } else if ch == ',' {
                        (State::FinishedToken, Token::Unquote)
                    } else if ch == '`' {
                        (State::FinishedToken, Token::Quasiquote)
                    } else if ch == '\'' {
                        (State::FinishedToken, Token::Quote)
                    } else if ch == '"' {
                        (State::DecodingText, Token::Text(String::new()))
                    } else if ch.is_whitespace() {
                        (State::Begin, Token::NoToken)
                    } else if ch.is_alphabetic() || ch == '_' {
                        (State::DecodingIdentifier, Token::Identifier(ch.to_string()))
                    } else if ch.is_numeric() {
                        (State::DecodingInteger, Token::Integer(ch.to_string()))
                    } else if ch.is_ascii_punctuation() {
                        (State::FinishedToken, Token::Identifier(ch.to_string()))
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
                            State::DecodingInteger,
                            Token::Integer(num.to_string() + &ch.to_string()),
                        )
                    } else {
                        self.chiter.unget(ch);
                        (State::FinishedToken, Token::Integer(num.to_string()))
                    }
                }
                (State::DecodingText, Token::Text(txt)) => {
                    if ch == '"' {
                        (State::FinishedToken, Token::Text(txt.to_string()))
                    } else {
                        (
                            State::DecodingText,
                            Token::Text(txt.to_string() + &ch.to_string()),
                        )
                    }
                }
                (State::DecodingIdentifier, _)
                | (State::DecodingInteger, _)
                | (State::DecodingText, _) => panic!("Inconsistent state!"),
            },
        }
    }

    pub fn token(&mut self) -> Result<Option<Token>> {
        if let Some(tk) = self.cache.take() {
            return Ok(Some(tk));
        }

        loop {
            let ch = self.chiter.next().transpose()?;
            self.state_machine(ch);
            match &self.state {
                (State::FinishedToken, Token::NoToken) => return Ok(None),
                (State::FinishedToken, token) => return Ok(Some(token.clone())),
                (_,_) => continue,
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
    #[ignore]
    fn get_tokens() {
        macro_rules! s {
            ($value:expr) => {
                String::from($value)
            };
        }

        use Token::*;
        let input = "(defun κόσμε (x y) '(+ x y \"test strings!\"))";
        let mut tokens = Tokenizer::new(Cursor::new(input).bytes());

        let mut tokenized = Vec::<Token>::new();

        while let Some(token) = tokens.token().unwrap() {
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
            Quote,
            OpenList,
            Identifier(s!("+")),
            Identifier(s!("x")),
            Identifier(s!("y")),
            Text(s!("test strings!")),
            CloseList,
            CloseList,
        ];

        assert_eq!(cmp, tokenized);
    }
}
