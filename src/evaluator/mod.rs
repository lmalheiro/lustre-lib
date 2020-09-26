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

    pub fn eval(&mut self, obj: &RefObject) -> ResultRefObject {
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
                        self.lambda(cdr)
                    } else if s == "DEF" {
                        eprintln!(">>>DEF cdr>>>: {:?}", cdr);
                        let (name, cdr) = destructure_list(cdr)?;
                        let (lambda, _) = destructure_list(cdr)?;
                        eprintln!(">>>DEF name>>>: {:?}", name);
                        eprintln!(">>>DEF func>>>: {:?}", lambda);
                        let name = self.eval(name)?;
                        eprintln!(">>>DEF name>>>: {:?}", name);
                        let lambda = self.eval(lambda)?;
                        eprintln!(">>>DEF func>>>: {:?}", lambda);
                        Ok(self.environment.intern(symbol_value(&name)?, lambda))
                    } else {
                        let car_eval = self.eval(car)?;
                        let cdr_eval = self.eval_list(cdr)?;
                        // eprintln!(">>>apply op>>>: {:?}", car_eval);
                        // eprintln!(">>>apply param>>>: {:?}", cdr_eval);
                        self.apply(car_eval, cdr_eval)
                    }
                } else {
                    let car_eval = self.eval(car)?;
                    let cdr_eval = self.eval_list(cdr)?;
                    self.apply(car_eval, cdr_eval)
                }
            }
            Some(Object::Symbol(s)) => match self.environment.find_symbol(s) {
                Some(v) => Ok(Arc::clone(&v)),
                _ => panic!("Unbound!"),
            },
            _ => Ok(Arc::clone(obj)),
        }
    }

    fn eval_list(&mut self, obj: &RefObject) -> ResultRefObject {
        if not_nil(&obj) {
            let (car, cdr) = destructure_list(obj)?;
            Object::Cons(self.eval(car)?, self.eval_list(cdr)?).into()
        } else {
            result_nil()
        }
    }

    fn lambda(&self, obj: &RefObject) -> ResultRefObject {
        let (params, cdr) = destructure_list(obj)?;
        let (expression, _) = destructure_list(cdr)?;
        Object::Lambda(params.clone(), expression.clone()).into()
    }

    fn apply(&mut self, function: RefObject, cdr: RefObject) -> ResultRefObject {
        match function
            .as_ref()
            .as_ref()
            .expect("Expecting a value, instead got nil or other None value.")
        {
            Object::Lambda(parameters, expression) => {
                self.environment.new_layer();
                eprintln!(">>>VALUES>>> {:?}", cdr);
                eprintln!(">>>EXPR>>> {:?}", expression);
                let values = self.eval_list(&cdr)?;
                let mut next_value = &values;
                let mut next_param = parameters;
                while not_nil(next_value) && not_nil(next_param) {
                    let (value, cdr_value) = destructure_list(next_value)?;
                    let (param, cdr_param) = destructure_list(next_param)?;
                    eprintln!(">>>VALUE>>> {:?}", value);
                    eprintln!(">>>PARAM>>> {:?}", param);
                    self.environment.intern(symbol_value(param)?, value.clone());
                    next_value = cdr_value;
                    next_param = cdr_param;
                }
                let result = self.eval(expression);
                self.environment.drop_layer();
                Ok(result?)
            }
            Object::Operator(_, f) => f(cdr, self.environment),
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
            let value = reader::Reader::new(tokenizer).read().unwrap();
            eprintln!("reader: {:?}", value);
            if let Some(_) = value.as_ref() {
                let mut evaluator = Evaluator::new(&mut environment);
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
            "((lambda (x y) (+ x y)) 13 21)";
            with obj {
                use crate::object::Object::*;
                #[rustfmt::skip]
                assert_eq!(Integer(34), *obj);
            }
        }
    }

    #[test]
    fn eval_test_7() {
        let input = "(def 'add (lambda (x y) (+ x y))) (add 13 21)";
        let tokenizer = reader::tokenizer::Tokenizer::new(Cursor::new(input).bytes());
        let mut environment = environment::Environment::new();
        operators::initialize_operators(&mut environment);
        let mut reader = reader::Reader::new(tokenizer);
        let mut evaluator = Evaluator::new(&mut environment);

        let mut result: ResultRefObject = result_nil();
        loop {
            let ast = reader.read().unwrap();
            eprintln!("reader: {:?}", ast);
            if ast.as_ref().is_some() {
                result = evaluator.eval(&ast);
            } else {
                break;
            }
        }
        
        assert_eq!(Object::Integer(34), *result.unwrap().as_ref().as_ref().unwrap());

        
    }

    #[test]
    fn eval_test_8() {
        let input = "
        (def 'fact (lambda (n) 
                     (if (< n 1) 
                       1
                       (* n 
                          (fact (- n 1)))
                         ))) 
        (fact 7)";
        let tokenizer = reader::tokenizer::Tokenizer::new(Cursor::new(input).bytes());
        let mut environment = environment::Environment::new();
        operators::initialize_operators(&mut environment);
        let mut reader = reader::Reader::new(tokenizer);
        let mut evaluator = Evaluator::new(&mut environment);

        let mut result: ResultRefObject = result_nil();
        loop {
            let ast = reader.read().unwrap();
            eprintln!("reader: {:?}", ast);
            if ast.as_ref().is_some() {
                result = evaluator.eval(&ast);
            } else {
                break;
            }
        }
        eprintln!("result: {:?}", result);
        assert_eq!(Object::Integer(5040), *result.unwrap().as_ref().as_ref().unwrap());

        
    }
}
