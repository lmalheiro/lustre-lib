
use crate::object::*;
use std::rc::Rc;


pub fn sum(obj: RefObject, env: &dyn Environment) -> RefObject {
    let mut total = 0i32;
    let mut next = &obj;
    while not_nil(next) {
        if let Some(Object::Cons(car, cdr)) = next.as_ref() {
            if let Some(Object::Integer(value)) = car.as_ref() {
                total += value;
            } else {
                panic!("Should be an integer here...")
            }
            next = cdr
        } else {
            panic!("Should exist a list here...")
        }
    }
    Rc::new(Some(Object::Integer(total)))
}

pub fn sub(obj: RefObject, env: &dyn Environment) -> RefObject {
    let mut total = 0i32;
    let mut next = &obj;
    if not_nil(next) {
        if let Some(Object::Cons(car, cdr)) = next.as_ref() {
            if let Some(Object::Integer(value)) = car.as_ref() {
                total = *value;
            } else {
                panic!("Should be an integer here...")
            }
            next = cdr;
            while not_nil(next) {
                if let Some(Object::Cons(car, cdr)) = next.as_ref() {
                    if let Some(Object::Integer(value)) = car.as_ref() {
                        total -= *value;
                    } else {
                        panic!("Should be an integer here...")
                    }
                    next = cdr;
                } else {
                    panic!("Should exist a list here...")
                }
            }
        } else {
            panic!("Should exist a list here...")
        }
    }
    Rc::new(Some(Object::Integer(total)))
}

pub fn greater_than(obj: RefObject, env: &dyn Environment) -> RefObject {
    let (car1, cdr) = destructure_list(obj);
    let (car2, _) = destructure_list(cdr);
    if integer_value(car1) > integer_value(car2) {
        Rc::new(Some(Object::Integer(1)))
    } else {
        env.get_nil()
    }
}

pub fn less_than(obj: RefObject, env: &dyn Environment) -> RefObject {
    let (car1, cdr) = destructure_list(obj);
    let (car2, _) = destructure_list(cdr);
    if integer_value(car1) < integer_value(car2) {
        Rc::new(Some(Object::Integer(1)))
    } else {
        env.get_nil()
    }
}

pub fn equal_to(obj: RefObject, env: &dyn Environment) -> RefObject {
    let (car1, cdr) = destructure_list(obj);
    let (car2, _) = destructure_list(cdr);
    if integer_value(car1) == integer_value(car2) {
        Rc::new(Some(Object::Integer(1)))
    } else {
        env.get_nil()
    }
}

pub fn quote(obj: RefObject, _env: &dyn Environment) -> RefObject {
    let (car, _) = destructure_list(obj);
    car
}

pub fn apply(
    function: RefObject,
    obj: RefObject,
    env: &dyn Environment,
) -> RefObject {
    match function
        .as_ref()
        .as_ref()
        .expect("Expecting a value, instead got nil or other None value.")
    {
        Object::Function(_value) => unimplemented!(),
        Object::Operator(f) => f(obj, env),
        _ => panic!("Expected operator or function."),
    }
}


pub fn initialize_operators(environment: &mut dyn Environment) {
    environment.intern(String::from("QUOTE"), Rc::new(Some(Object::Operator(quote))));
    environment.intern(String::from("+"), Rc::new(Some(Object::Operator(sum))));
    environment.intern(String::from("-"), Rc::new(Some(Object::Operator(sub))));
    environment.intern(String::from("="), Rc::new(Some(Object::Operator(equal_to))));
    environment.intern(String::from("<"), Rc::new(Some(Object::Operator(less_than))));
    environment.intern(
        String::from(">"),
        Rc::new(Some(Object::Operator(greater_than))),
    );
}
