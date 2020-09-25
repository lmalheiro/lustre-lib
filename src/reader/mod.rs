
pub mod tokenizer;

use crate::reader::tokenizer::*;
use crate::object::{Object, RefObject, Environment, nil, result_nil};

use crate::errors::Result;
use std::sync::Arc;

pub struct Reader<'a, T>
where
    T: Iterator,
{
    tokenizer: Tokenizer<T>,
    environment: &'a mut dyn Environment,
}

macro_rules! r#return {
    ($obj:ident; $value:expr) => {
        return Ok(Arc::new(Some(Object::$obj($value))));
    };
    ($value:expr) => {
        return Ok($value);
    };
}

impl<'a, T> Reader<'a, T>
where
    T: Iterator<Item = std::result::Result<u8, std::io::Error>>,
{
    pub fn new(tokenizer: Tokenizer<T>, environment: &'a mut dyn Environment) -> Self {
        Self {
            tokenizer,
            environment,
        }
    }
    pub fn read(&mut self) -> Result<RefObject> {
        if let Some(token) = self.tokenizer.token()? {
            match token {
                Token::Integer(s) => {
                    let value = s.parse::<i32>()?;
                    r#return!(Integer; value);
                }
                Token::Text(s) => r#return!(IString; s),
                Token::Identifier(s) => {
                    if let Some(value) = self.environment.find_symbol(&s) {
                        return Ok(value);
                    } else {
                        let value = self
                            .environment
                            .intern(s.to_uppercase(), Arc::new(Some(Object::Symbol(s.to_uppercase()))));
                        return Ok(value);
                    }
                }
                Token::OpenList => self.read_list(),
                Token::Quote => {
                    Ok(Arc::new(Some(Object::Cons(
                        Arc::new(Some(Object::Symbol(String::from("QUOTE")))),
                        Arc::new(Some(Object::Cons(self.read()?, nil()))),
                    ))))
                }

                Token::NoToken => panic!("Inconsistent state sice NoToken isn't a valid return value."),
                Token::Quasiquote => unimplemented!(),
                Token::Unquote => unimplemented!(),
                Token::CloseList => panic!("Unexpected end of a list."),
                Token::Invalid(s) => unimplemented!("{:?}", s),
            }
        } else {
            Ok(Arc::new(None))
        }
    }
    fn read_list(&mut self) -> Result<RefObject> {
        if let Some(token) = self.tokenizer.token()? {
            if let Token::CloseList = token {
                result_nil()
            } else {
                self.tokenizer.putback(token);
                Ok(Arc::new(Some(Object::Cons(
                    self.read()?,
                    self.read_list()?,
                ))))
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
    use crate::environment;

    #[test]
    #[ignore]
    fn reader_test() {
        let input = "(defun κόσμε (x y) (* (+ x y) 10))";
        let tokenizer = Tokenizer::new(Cursor::new(input).bytes());
        let mut environment = environment::Environment::new();
        let mut reader = Reader::new(tokenizer, &mut environment);
        let a = reader.read();
        let b = a.unwrap();
        if let Some(object) = b.as_ref() {
            eprintln!("result: {}", object);

            assert_eq!("( defun κόσμε ( x y ) ( * ( + x y ) 10 ) )", format!("{}", object));
        } else {
            panic!("Ooops! Not an object...")
        }

        
    }
}
