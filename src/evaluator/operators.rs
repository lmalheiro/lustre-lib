use crate::object::Environment;
use crate::object::Object;
use std::rc::Rc;

macro_rules! not_nil {
    ($env:expr; $value:expr) => {
        !Rc::ptr_eq($value, &$env.get_nil())
    };
}
pub fn sum(obj: Rc<Option<Object>>, env: &dyn Environment) -> Rc<Option<Object>> {
    let mut total = 0i32;
    let mut next = &obj;
    while not_nil!(env; next) {
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

pub fn sub(obj: Rc<Option<Object>>, env: &dyn Environment) -> Rc<Option<Object>> {
    let mut total = 0i32;
    let mut next = &obj;
    if not_nil!(env; next) {
        if let Some(Object::Cons(car, cdr)) = next.as_ref() {
            if let Some(Object::Integer(value)) = car.as_ref() {
                total = *value;
            } else {
                panic!("Should be an integer here...")
            }
            next = cdr;
            while not_nil!(env; next) {
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

pub fn apply(function: Rc<Option<Object>>, obj: Rc<Option<Object>>, env: &dyn Environment) -> Rc<Option<Object>> {

    match function.as_ref().as_ref().expect("Expecting a value, instead got nil or other None value.") {
        Object::Function(_value) => unimplemented!(),
        Object::Operator(f) => { f(obj, env) },
        _ => { panic!("Expected operator or function.") }
    }
}

pub fn initialize_operators(environment: &mut dyn Environment) {
    environment.intern(
        String::from("+"),
        Rc::new(Some(Object::Operator(sum))),
    );
    environment.intern(
        String::from("-"),
        Rc::new(Some(Object::Operator(sub))),
    );
}