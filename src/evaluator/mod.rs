pub mod operators;

use crate::environment::RefEnvironment;
use std::sync::Arc;
use crate::object::{destructure_list, nil, not_nil, result_nil, symbol_value, Object, RefObject, ResultRefObject};
use tokio::runtime::Runtime;
use async_recursion::async_recursion;

pub fn eval(obj: &RefObject, environment: RefEnvironment, rt: &mut Runtime) -> ResultRefObject {
    rt.block_on(async move {
        parallel_eval(obj, environment).await
    })
}

#[async_recursion]
async fn parallel_eval(obj: &RefObject, environment: RefEnvironment ) -> ResultRefObject {
    match obj.as_ref() {
        None => result_nil(),
        Some(Object::Cons(car, cdr)) => {
            if let Some(Object::Symbol(s)) = car.as_ref() {
                if s == "IF" {
                    let (car1, cdr) = destructure_list(cdr)?;
                    let (car2, cdr) = destructure_list(&cdr)?;
                    let (car3, _) = destructure_list(&cdr)?;
                    let test = parallel_eval(car1, environment.clone());
                    let true_statement = parallel_eval(car2, environment.clone());
                    let false_statement = parallel_eval(car3, environment.clone());
                    if not_nil(&test.await?) {
                        true_statement.await
                    } else {
                        false_statement.await
                    }
                } else if s == "QUOTE" {
                    let (car, _) = destructure_list(cdr)?;
                    Ok(Arc::clone(car))
                } else if s == "LAMBDA" {
                    lambda(cdr)
                } else if s == "DEF" {
                    let (name, cdr) = destructure_list(cdr)?;
                    let (value, _) = destructure_list(cdr)?;
                    let name = parallel_eval(name, environment.clone()).await?;
                    let search_result = {
                        let env = environment.0.read().unwrap();
                        env.find_symbol(&symbol_value(&name)?)
                    };
                    if let None = search_result {
                        let value = parallel_eval(value, environment.clone()).await?;
                        Ok(environment.0.write().unwrap().intern(symbol_value(&name)?, value))
                    } else {
                        panic!("Not allowed to redefine symbol.")
                    }
                } else {
                    let car_eval = parallel_eval(car, environment.clone());
                    let cdr_eval = parallel_eval_list(cdr, environment.clone());
                    apply(car_eval.await?, cdr_eval.await?, environment).await
                }
            } else {
                let car_eval = parallel_eval(car, environment.clone());
                let cdr_eval = parallel_eval_list(cdr, environment.clone());
                apply(car_eval.await?, cdr_eval.await?, environment).await
            }
        }
        Some(Object::Symbol(s)) => match environment.0.read().unwrap().find_symbol(s) {
            Some(v) => Ok(Arc::clone(&v)),
            _ => panic!("Unbound!"),
        },
        _ => Ok(Arc::clone(obj)),
    }
}

// fn _recursive_eval_list<'a>(obj: &RefObject, environment: &RefEnvironment) -> ResultRefObject {
//     if not_nil(&obj) {
//         let (car, cdr) = destructure_list(obj)?;
//         Object::Cons(eval(car, environment)?, eval_list(cdr, environment)?).into()
//     } else {
//         result_nil()
//     }
// }

// fn eval_list(obj: &RefObject, environment: &RefEnvironment) -> ResultRefObject {
//     let mut next = obj;
//     let mut result:RefObject = nil();
//     let mut partials: Vec<_> = Vec::new();
//     while not_nil(next) {
//         let (car, cdr) = destructure_list(next)?;
//         partials.push(eval(car, environment)?);
//         next = cdr;
//     }
    
//     while let Some(value) = partials.pop() {
//         result = Arc::new(Some(Object::Cons(value, result)));
//     }
    
//     Ok(result)
// }

#[async_recursion]
async fn parallel_eval_list(obj: &RefObject, environment: RefEnvironment) -> ResultRefObject {
    let mut next = obj.clone();
    let mut result:RefObject = nil();
    let mut handles: Vec<_> = Vec::new();
    while not_nil(&next) {
        let (car, cdr) = destructure_list(&next)?;
        let car = car.clone();
        let env = environment.clone();
        handles.push(async move { parallel_eval(&car, env).await });
        next = cdr.clone();
    }

    let handles = handles.into_iter().map(tokio::spawn).collect::<Vec<_>>();

    let mut partials = futures::future::join_all(handles).await;

    while let Some(value) = partials.pop() {
        result = Arc::new(Some(Object::Cons(value.unwrap()?, result)));
    }
    
    Ok(result)
}

fn lambda(obj: &RefObject) -> ResultRefObject {
    let (params, cdr) = destructure_list(obj)?;
    let (expression, _) = destructure_list(cdr)?;
    Object::Lambda(params.clone(), expression.clone()).into()
}

#[async_recursion]
async fn apply(function: RefObject, cdr: RefObject, environment: RefEnvironment) -> ResultRefObject {
    match function
        .as_ref()
        .as_ref()
        .expect("Expecting a value, instead got nil or other None value.")
    {
        Object::Lambda(parameters, expression) => {
            let values = parallel_eval_list(&cdr, environment.clone()).await?;
            let mut next_value = &values;
            let mut next_param = parameters;
            let scope = RefEnvironment::from(&environment);
            while not_nil(next_value) && not_nil(next_param) {
                let (value, cdr_value) = destructure_list(next_value)?;
                let (param, cdr_param) = destructure_list(next_param)?;
                scope.0.write().unwrap().intern(symbol_value(param)?, value.clone());
                next_value = cdr_value;
                next_param = cdr_param;
            }
            let result = parallel_eval(expression, scope).await;
            Ok(result?)
        }
        Object::Operator(_, f) => f(cdr),
        _ => panic!("Expected operator or function."),
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
            let mut rt = Runtime::new().unwrap();
            let input = $code;
            let tokenizer = reader::tokenizer::Tokenizer::new(Cursor::new(input).bytes());
            let mut environment = RefEnvironment::new();
            operators::initialize_operators(&mut environment);
            let value = reader::Reader::new(tokenizer).read().unwrap();
            eprintln!("reader: {:?}", value);
            if let Some(_) = value.as_ref() {
                let result = eval(&value, environment, &mut rt);
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
        let mut rt = Runtime::new().unwrap();
        let tokenizer = reader::tokenizer::Tokenizer::new(Cursor::new(input).bytes());
        let mut environment = RefEnvironment::new();
        operators::initialize_operators(&mut environment);
        let mut reader = reader::Reader::new(tokenizer);
        let mut result: ResultRefObject = result_nil();
        loop {
            let ast = reader.read().unwrap();
            eprintln!("reader: {:?}", ast);
            if ast.as_ref().is_some() {
                result = eval(&ast, environment.clone(), &mut rt);
            } else {
                break;
            }
        }

        assert_eq!(
            Object::Integer(34),
            *result.unwrap().as_ref().as_ref().unwrap()
        );
    }

    #[test]
    fn eval_test_8() {
        let mut rt = Runtime::new().unwrap();
        let input = "
        (def 'fact (lambda (n) 
                     (if (< n 1) 
                       1
                       (* n 
                          (fact (- n 1)))
                         ))) 
        (fact 7)";
        let tokenizer = reader::tokenizer::Tokenizer::new(Cursor::new(input).bytes());
        let environment = environment::RefEnvironment::new();
        operators::initialize_operators(&environment);
        let mut reader = reader::Reader::new(tokenizer);
        let mut result: ResultRefObject = result_nil();
        loop {
            let ast = reader.read().unwrap();
            eprintln!("reader: {:?}", ast);
            if ast.as_ref().is_some() {
                result = eval(&ast, environment.clone(), &mut rt);
            } else {
                break;
            }
        }
        eprintln!("result: {:?}", result);
        assert_eq!(
            Object::Integer(5040),
            *result.unwrap().as_ref().as_ref().unwrap()
        );
    }
}
