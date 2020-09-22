mod environment;
mod tokenizer;

use crate::reader::environment::*;
use crate::reader::tokenizer::*;
use anyhow::Result;

pub struct Reader<T>
where
    T: Iterator,
{
    tokenizer: Tokenizer<T>,
    environment: Environment,
}

macro_rules! r#return {
    ($obj:ident; $value:expr) => {
        return Ok(Some(Object::$obj($value)));
    };
    ($value:expr) => {
        return Ok(Some($value));
    };
}

impl<T> Reader<T>
where
    T: Iterator<Item = std::result::Result<u8, std::io::Error>>,
{
    pub fn new(tokenizer: Tokenizer<T>, environment: Environment) -> Self {
        Self {
            tokenizer,
            environment,
        }
    }
    pub fn read(&mut self) -> Result<Option<Object>> {
        if let Some(token) = self.tokenizer.token()? {
            match token {
                Token::Identifier(s) => r#return!(IString; s),
                Token::Integer(s) => {
                    let value = s.parse::<i32>()?;
                    r#return!(Integer; value);
                }
                Token::Text(s) => r#return!(IString; s),
                Token::Symbol(s) => {
                    self.environment
                        .intern(s.clone(), Object::Symbol(s.clone()));
                    r#return!(Object::Symbol(s));
                }
                Token::OpenList => self.read_list(),
                Token::Quote => self.read(),

                Token::NoToken => unimplemented!(),
                Token::Quasiquote => unimplemented!(),
                Token::Unquote => unimplemented!(),
                Token::CloseList => unimplemented!(),
                Token::Invalid(s) => unimplemented!("{:?}", s),
            }
        } else {
            Ok(None)
        }
    }
    fn read_list(&mut self) -> Result<Option<Object>> {
        if let Some(token) = self.tokenizer.token()? {
            if let Token::CloseList = token {
                Ok(None)
            } else {
                self.tokenizer.putback(token);
                Ok(Some(Object::Cons(
                    Box::new(self.read()?),
                    Box::new(self.read_list()?),
                )))
            }
        } else {
            panic!("Unexpected end of stream!");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::prelude::*;
    use std::io::Cursor;

    #[test]
    fn reader_test() {
        let input = "(defun κόσμε (x y) (+ x y))";
        let tokenizer = Tokenizer::new(Cursor::new(input).bytes());
        let environment = Environment::new();
        let mut reader = Reader::new(tokenizer, environment);
        let object = reader.read().unwrap().unwrap();

        eprintln!("result: {}", object);

        assert_eq!("( defun κόσμε ( x y ) ( + x y ) )", format!("{}", object));
    }
}
