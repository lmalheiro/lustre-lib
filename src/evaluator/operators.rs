use crate::evaluator::RefEnvironment;
use crate::object::*;


use std::sync::Arc;

pub fn sum(obj: RefObject) -> ResultRefObject {
    if not_nil(&obj) {
        let mut total = 0i32;
        let mut next = &obj;
        while not_nil(next) {
            let (car, cdr) = destructure_list(next)?;
            total += integer_value(&car)?;
            next = &cdr;
        }
        Object::Integer(total).into()
    } else {
        result_nil()
    }
}

pub fn mult(obj: RefObject) -> ResultRefObject {
    if not_nil(&obj) {
        let mut total = 1i32;
        let mut next = &obj;
        while not_nil(next) {
            let (car, cdr) = destructure_list(next)?;
            total *= integer_value(&car)?;
            next = &cdr;
        }
        Object::Integer(total).into()
    } else {
        result_nil()
    }
}

pub fn div(obj: RefObject) -> ResultRefObject {
    if not_nil(&obj) {
        let (car, cdr) = destructure_list(&obj)?;
        let mut total = integer_value(&car)?;
        let mut next = cdr;
        while not_nil(next) {
            let (car, cdr) = destructure_list(next)?;
            total /= integer_value(&car)?;
            next = &cdr;
        }
        Object::Integer(total).into()
    } else {
        result_nil()
    }
}

pub fn sub(obj: RefObject) -> ResultRefObject {
    if not_nil(&obj) {
        let (car, cdr) = destructure_list(&obj)?;
        let mut total = integer_value(&car)?;
        let mut next = cdr;
        while not_nil(next) {
            let (car, cdr) = destructure_list(next)?;
            total -= integer_value(&car)?;
            next = &cdr;
        }
        Object::Integer(total).into()
    } else {
        result_nil()
    }
}

pub fn and(obj: RefObject) -> ResultRefObject {
    let (car1, cdr) = destructure_list(&obj)?;
    let (car2, _) = destructure_list(&cdr)?;
    if not_nil(&car1) && not_nil(&car2) {
        Object::Integer(1).into()
    } else {
        result_nil()
    }
}

pub fn or(obj: RefObject) -> ResultRefObject {
    let (car1, cdr) = destructure_list(&obj)?;
    let (car2, _) = destructure_list(&cdr)?;
    if not_nil(&car1) || not_nil(&car2) {
        Object::Integer(1).into()
    } else {
        result_nil()
    }
}

pub fn not(obj: RefObject) -> ResultRefObject {
    let (car1, _) = destructure_list(&obj)?;
    if !not_nil(&car1) {
        Object::Integer(1).into()
    } else {
        result_nil()
    }
}

pub fn car(obj: RefObject) -> ResultRefObject {
    if not_nil(&obj) {
        let (car, _) = destructure_list(&obj)?;
        let (car, _) = destructure_list(car)?;
        Ok(Arc::clone(car))
    } else {
        result_nil()
    }
}

pub fn cdr(obj: RefObject) -> ResultRefObject {
    if not_nil(&obj) {
        let (car, _) = destructure_list(&obj)?;
        let (_, cdr) = destructure_list(car)?;
        Ok(Arc::clone(cdr))
    } else {
        result_nil()
    }
}

pub fn greater_than(obj: RefObject) -> ResultRefObject {
    let (car1, cdr) = destructure_list(&obj)?;
    let (car2, _) = destructure_list(&cdr)?;
    if integer_value(&car1)? > integer_value(&car2)? {
        Object::Integer(1).into()
    } else {
        result_nil()
    }
}

pub fn less_than(obj: RefObject) -> ResultRefObject {
    let (car1, cdr) = destructure_list(&obj)?;
    let (car2, _) = destructure_list(&cdr)?;
    if integer_value(&car1)? < integer_value(&car2)? {
        Object::Integer(1).into()
    } else {
        result_nil()
    }
}

pub fn equal_to(obj: RefObject) -> ResultRefObject {
    let (car1, cdr) = destructure_list(&obj)?;
    let (car2, _) = destructure_list(&cdr)?;
    if integer_value(&car1)? == integer_value(&car2)? {
        Object::Integer(1).into()
    } else {
        result_nil()
    }
}

pub fn quote(obj: RefObject) -> ResultRefObject {
    let (car, _) = destructure_list(&obj)?;
    Ok(Arc::clone(car))
}

pub fn initialize_operators(environment: &RefEnvironment) {
    macro_rules! register {
        ($name:literal, $func:ident) => {
            environment.0.write().unwrap().intern(
                String::from($name),
                Arc::new(Some(Object::Operator(String::from($name), $func))),
            );
        };
    }
    register!("QUOTE", quote);
    register!("AND", and);
    register!("OR", or);
    register!("NOT", not);
    register!("CAR", car);
    register!("CDR", cdr);
    register!("+", sum);
    register!("-", sub);
    register!("*", mult);
    register!("/", div);
    register!("=", equal_to);
    register!("<", less_than);
    register!(">", greater_than);
}
