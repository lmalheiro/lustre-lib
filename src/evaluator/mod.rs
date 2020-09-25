mod operators;

use std::sync::Arc;

use crate::object::*;

struct Evaluator<'a> {
    environment: &'a mut dyn Environment,
}

impl<'a> Evaluator<'a> {
    pub fn new(environment: &'a mut dyn Environment) -> Self {
        Evaluator { environment }
    }

    pub fn eval(&self, obj: &RefObject) -> ResultRefObject {
        match obj.as_ref() {
            None => result_nil(),
            Some(Object::Cons(car, cdr)) => {
                if let Some(Object::Symbol(s)) = car.as_ref() {
                    if s == "IF" {
                        let (car1, cdr) = destructure_list(cdr)?;
                        let (car2, cdr) = destructure_list(&cdr)?;
                        let (car3, _) = destructure_list(&cdr)?;
                        let test = self.eval(car1)?;
                        if not_nil(&test) {
                            self.eval(car2)
                        } else {
                            self.eval(car3)
                        }
                    } else if s == "QUOTE" {
                        let (car, _) = destructure_list(cdr)?;
                        Ok(Arc::clone(car))
                    } else if s == "LAMBDA" {
                        let (_, cdr) = destructure_list(cdr)?;
                        self.mk_lambda(cdr.clone(), self.environment)
                    } else {
                        self.apply(self.eval(car)?, self.eval_list(cdr)?, self.environment)
                    }
                } else {
                    self.apply(self.eval(car)?, self.eval_list(cdr)?, self.environment)
                }
            }
            _ => Ok(Arc::clone(obj)),
        }
    }

    fn eval_list(&self, obj: &RefObject) -> ResultRefObject {
        if not_nil(&obj) {
            let (car, cdr) = destructure_list(obj)?;
            Object::Cons(self.eval(car)?, self.eval_list(cdr)?).into()
        } else {
            result_nil()
        }
    }

    fn mk_lambda(&self, obj: RefObject, _env: &dyn Environment) -> ResultRefObject {
        let (car1, cdr) = destructure_list(&obj)?;
        let (car2, _) = destructure_list(cdr)?;
        unimplemented!()
    }

    fn apply(&self, function: RefObject, obj: RefObject, env: &dyn Environment) -> ResultRefObject {
        match function
            .as_ref()
            .as_ref()
            .expect("Expecting a value, instead got nil or other None value.")
        {
            Object::Lambda(_parameters, _expression) => unimplemented!(),
            Object::Operator(f) => f(obj, env),
            _ => panic!("Expected operator or function."),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::environment;
    use crate::reader;
    use std::io::prelude::*;
    use std::io::Cursor;

    macro_rules! test_eval {
        ($code:expr; with $var:ident $test:block) => {
            let input = $code;
            let tokenizer = reader::tokenizer::Tokenizer::new(Cursor::new(input).bytes());
            let mut environment = environment::Environment::new();
            operators::initialize_operators(&mut environment);
            let value = reader::Reader::new(tokenizer, &mut environment)
                .read()
                .unwrap();
            eprintln!("reader: {:?}", value);
            if let Some(_) = value.as_ref() {
                let evaluator = Evaluator::new(&mut environment);
                let result = evaluator.eval(&value);
                eprintln!("result: {:?}", result);
                if let Some($var) = result.unwrap().as_ref() {
                    eprintln!("result: {}", $var);
                    $test;
                } else {
                    panic!("Ooops! Not an object...")
                }
            } else {
                panic!("Ooops! Not an object...")
            }
        };
    }

    #[test]
    fn eval_test_1() {
        test_eval! {
            "(+ 1000 1000 (+ 10 10) (- 0 100 ))";
            with obj {
                assert_eq!(Object::Integer(1920), *obj);
            }
        }
    }

    #[test]
    fn eval_test_2() {
        test_eval! {
            "(if (< 10 20) (if (> 10 20) \"TRUE-TRUE\" \"TRUE-FALSE\") \"FALSE\")";
            with obj {
                assert_eq!(Object::IString("TRUE-FALSE".to_string()), *obj);
            }
        }
    }

    #[test]
    fn eval_test_3() {
        test_eval! {
            "'(a b c)";
            with obj {
                use crate::object::Object::*;
                #[rustfmt::skip]
                assert_eq!(
                    Cons(
                        Symbol("A".to_string()).into(),
                        Cons(
                            Symbol("B".to_string()).into(),
                            Cons(Symbol("C".to_string()).into(),
                                 nil()).into()
                        ).into()
                    ),
                    *obj
                );
            }
        }
    }

    #[test]
    fn eval_test_4() {
        #[rustfmt::skip]
        test_eval! {
            "(if (and (< 10 20) 
                      (> 30 15)) 
                 (if (or (> 10 20) 
                         (> 20 (* 3 5)))
                     \"TRUE-TRUE\" 
                     \"TRUE-FALSE\") 
                \"FALSE\")";
            with obj {
                assert_eq!(Object::IString("TRUE-TRUE".to_string()), *obj); 
            }
        }
    }

    #[test]
    fn eval_test_5() {
        test_eval! {
            "(car (cdr '(X 100 b c)))";
            with obj {
                use crate::object::Object::*;
                #[rustfmt::skip]
                assert_eq!(Integer(100), *obj);
            }
        }
    }

    #[test]
    fn eval_test_6() {
        test_eval! {
            "(lambda (x y) (+ x y))";
            with obj {
                use crate::object::Object::*;
                #[rustfmt::skip]
                assert_eq!(Integer(100), *obj);
            }
        }
    }
}
