mod operators;

use std::rc::Rc;

use crate::object::*;

struct Evaluator<'a> {
    environment: &'a mut dyn Environment,
}

macro_rules! not_nil {
    ($env:expr; $value:expr) => {
        !Rc::ptr_eq($value, &$env.get_nil())
    };
}

impl<'a> Evaluator<'a> {
    pub fn new(environment: &'a mut dyn Environment) -> Self {
        Evaluator { environment }
    }

    pub fn eval(&self, obj: Rc<Option<Object>>) -> Rc<Option<Object>> {
        match obj.as_ref() {
            None => self.environment.get_nil(),
            Some(Object::Cons(car, cdr)) => {
                if let Some(Object::Symbol(s)) = car.as_ref() {
                    if s == "if" {
                        let (car1, cdr) = operators::destructure_list(cdr.clone());
                        let (car2, cdr) = operators::destructure_list(cdr);
                        let (car3, _) = operators::destructure_list(cdr);
                        let test = self.eval(car1);
                        if not_nil!(self.environment; &test) {
                            self.eval(car2)
                        } else {
                            self.eval(car3)
                        }
                    } else if s == "QUOTE" {
                        let (car, _) = operators::destructure_list(cdr.clone());
                        car
                    } else {
                        operators::apply(
                            self.eval(car.clone()),
                            self.eval_list(cdr.clone()),
                            self.environment,
                        )
                    }
                } else {
                    operators::apply(
                        self.eval(car.clone()),
                        self.eval_list(cdr.clone()),
                        self.environment,
                    )
                }
            }
            _ => obj,
        }
        // if Rc::ptr_eq(&obj, &self.environment.get_nil()) {
        //     return self.environment.get_nil();
        // } else {
        //     if let Object::Cons(car, cdr) = obj
        //         .as_ref()
        //         .as_ref()
        //         .expect("Invalid 'None' object. It should have matched the 'nil'.")
        //     {
        //         operators::apply(
        //             self.eval(car.clone()),
        //             self.eval_list(cdr.clone()),
        //             self.environment,
        //         )
        //     } else {
        //         obj
        //     }
        // }
    }

    fn eval_list(&self, obj: Rc<Option<Object>>) -> Rc<Option<Object>> {
        if not_nil!(self.environment; &obj) {
            if let Some(Object::Cons(car, cdr)) = obj.as_ref() {
                Rc::new(Some(Object::Cons(
                    self.eval(car.clone()),
                    self.eval_list(cdr.clone()),
                )))
            } else {
                panic!("Should exist a list here...")
            }
        } else {
            self.environment.get_nil()
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

    #[test]
    #[ignore]
    fn eval_test() {
        let input = "1000";
        let tokenizer = reader::tokenizer::Tokenizer::new(Cursor::new(input).bytes());
        let mut environment = environment::Environment::new();
        operators::initialize_operators(&mut environment);
        let value = reader::Reader::new(tokenizer, &mut environment)
            .read()
            .unwrap();
        eprintln!(">>>>>>{:?}", value);
        if let Some(_) = value.as_ref() {
            let evaluator = Evaluator::new(&mut environment);
            let result = evaluator.eval(value);
            if let Some(obj) = result.as_ref() {
                eprintln!("result: {}", obj);
            }
        } else {
            panic!("Ooops! Not an object...")
        }
    }

    #[test]
    fn eval_test_2() {
        //let input = "(+ 1000 1000 (+ 10 10) (- 0 100 ))";
        //let input = "(if (< 20 20) (if (> 30 20) \"TRUE-TRUE\" \"TRUE-FALSE\") \"FALSE\")";
        let input = "'(a b c)";
        let tokenizer = reader::tokenizer::Tokenizer::new(Cursor::new(input).bytes());
        let mut environment = environment::Environment::new();
        operators::initialize_operators(&mut environment);
        let value = reader::Reader::new(tokenizer, &mut environment)
            .read()
            .unwrap();
        eprintln!(">>>>>>{:?}", value);
        if let Some(_) = value.as_ref() {
            let evaluator = Evaluator::new(&mut environment);
            let result = evaluator.eval(value);
            eprintln!("result: {:?}", result);
            if let Some(obj) = result.as_ref() {
                eprintln!("obj: {}", obj);
            }
        } else {
            panic!("Ooops! Not an object...")
        }
    }
}
