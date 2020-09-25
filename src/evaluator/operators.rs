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

pub fn greater_than(obj: Rc<Option<Object>>, env: &dyn Environment) -> Rc<Option<Object>> {
    let (car1, cdr) = destructure_list(obj);
    let (car2, _) = destructure_list(cdr);
    if integer_value(car1) > integer_value(car2) {
        Rc::new(Some(Object::Integer(1)))
    } else {
        env.get_nil()
    }
}

pub fn less_than(obj: Rc<Option<Object>>, env: &dyn Environment) -> Rc<Option<Object>> {
    let (car1, cdr) = destructure_list(obj);
    let (car2, _) = destructure_list(cdr);
    if integer_value(car1) < integer_value(car2) {
        Rc::new(Some(Object::Integer(1)))
    } else {
        env.get_nil()
    }
}

pub fn equal_to(obj: Rc<Option<Object>>, env: &dyn Environment) -> Rc<Option<Object>> {
    let (car1, cdr) = destructure_list(obj);
    let (car2, _) = destructure_list(cdr);
    if integer_value(car1) == integer_value(car2) {
        Rc::new(Some(Object::Integer(1)))
    } else {
        env.get_nil()
    }
}

pub fn quote(obj: Rc<Option<Object>>, _env: &dyn Environment) -> Rc<Option<Object>> {
    let (car, _) = destructure_list(obj);
    car
}

pub fn apply(
    function: Rc<Option<Object>>,
    obj: Rc<Option<Object>>,
    env: &dyn Environment,
) -> Rc<Option<Object>> {
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

pub fn destructure_list(list: Rc<Option<Object>>) -> (Rc<Option<Object>>, Rc<Option<Object>>) {
    if let Some(Object::Cons(car, cdr)) = list.as_ref() {
        (car.clone(), cdr.clone())
    } else {
        panic!("Not a list!");
    }
}

pub fn symbol_value(sym: Rc<Option<Object>>) -> String {
    if let Some(Object::Symbol(value)) = sym.as_ref() {
        value.to_string()
    } else {
        panic!("Not a symbol!");
    }
}

pub fn integer_value(int: Rc<Option<Object>>) -> i32 {
    if let Some(Object::Integer(value)) = int.as_ref() {
        *value
    } else {
        panic!("Not an integer!");
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
