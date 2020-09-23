
pub mod tokenizer;

use crate::object::Environment;
use crate::reader::tokenizer::*;
use crate::object::Object;

use anyhow::Result;
use std::rc::Rc;

pub struct Reader<'a, T>
where
    T: Iterator,
{
    tokenizer: Tokenizer<T>,
    environment: &'a mut dyn Environment,
}

macro_rules! r#return {
    ($obj:ident; $value:expr) => {
        return Ok(Rc::new(Some(Object::$obj($value))));
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
    pub fn read(&mut self) -> Result<Rc<Option<Object>>> {
        if let Some(token) = self.tokenizer.token()? {
            match token {
                Token::Identifier(s) => r#return!(IString; s),
                Token::Integer(s) => {
                    let value = s.parse::<i32>()?;
                    r#return!(Integer; value);
                }
                Token::Text(s) => r#return!(IString; s),
                Token::Symbol(s) => {
                    if let Some(value) = self.environment.find_symbol(&s) {
                        return Ok(value);
                    } else {
                        let value = self
                            .environment
                            .intern(s.clone(), Rc::new(Some(Object::Symbol(s.clone()))));
                        return Ok(value);
                    }
                }
                Token::OpenList => self.read_list(),
                Token::Quote => {
                    Ok(Rc::new(Some(Object::Cons(
                        Rc::new(Some(Object::Symbol(String::from("QUOTE")))),
                        self.read_list()?,
                    ))))
                }

                Token::NoToken => unimplemented!(),
                Token::Quasiquote => unimplemented!(),
                Token::Unquote => unimplemented!(),
                Token::CloseList => unimplemented!(),
                Token::Invalid(s) => unimplemented!("{:?}", s),
            }
        } else {
            Ok(Rc::new(None))
        }
    }
    fn read_list(&mut self) -> Result<Rc<Option<Object>>> {
        if let Some(token) = self.tokenizer.token()? {
            if let Token::CloseList = token {
                Ok(self.environment.get_nil())
            } else {
                self.tokenizer.putback(token);
                Ok(Rc::new(Some(Object::Cons(
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
