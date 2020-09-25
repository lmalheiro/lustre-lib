use crate::object::*;
use std::sync::Arc;

pub fn sum(obj: RefObject, _env: &dyn Environment) -> ResultRefObject {
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

pub fn mult(obj: RefObject, _env: &dyn Environment) -> ResultRefObject {
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

pub fn div(obj: RefObject, _env: &dyn Environment) -> ResultRefObject {
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

pub fn sub(obj: RefObject, _env: &dyn Environment) -> ResultRefObject {
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

pub fn and(obj: RefObject, _env: &dyn Environment) -> ResultRefObject {
    let (car1, cdr) = destructure_list(&obj)?;
    let (car2, _) = destructure_list(&cdr)?;
    if not_nil(&car1) && not_nil(&car2) {
        Object::Integer(1).into()
    } else {
        result_nil()
    }
}

pub fn or(obj: RefObject, _env: &dyn Environment) -> ResultRefObject {
    let (car1, cdr) = destructure_list(&obj)?;
    let (car2, _) = destructure_list(&cdr)?;
    if not_nil(&car1) || not_nil(&car2) {
        Object::Integer(1).into()
    } else {
        result_nil()
    }
}

pub fn not(obj: RefObject, _env: &dyn Environment) -> ResultRefObject {
    let (car1, _) = destructure_list(&obj)?;
    if !not_nil(&car1) {
        Object::Integer(1).into()
    } else {
        result_nil()
    }
}

pub fn car(obj: RefObject, _env: &dyn Environment) -> ResultRefObject {
    if not_nil(&obj) {
        let (car, _) = destructure_list(&obj)?;
        let (car, _) = destructure_list(car)?;
        Ok(Arc::clone(car))
    } else {
        result_nil()
    }
}

pub fn cdr(obj: RefObject, _env: &dyn Environment) -> ResultRefObject {
    if not_nil(&obj) {
        let (car, _) = destructure_list(&obj)?;
        let (_, cdr) = destructure_list(car)?;
        Ok(Arc::clone(cdr))
    } else {
        result_nil()
    }
}



pub fn greater_than(obj: RefObject, _env: &dyn Environment) -> ResultRefObject {
    let (car1, cdr) = destructure_list(&obj)?;
    let (car2, _) = destructure_list(&cdr)?;
    if integer_value(&car1)? > integer_value(&car2)? {
        Object::Integer(1).into()
    } else {
        result_nil()
    }
}

pub fn less_than(obj: RefObject, _env: &dyn Environment) -> ResultRefObject {
    let (car1, cdr) = destructure_list(&obj)?;
    let (car2, _) = destructure_list(&cdr)?;
    if integer_value(&car1)? < integer_value(&car2)? {
        Object::Integer(1).into()
    } else {
        result_nil()
    }
}

pub fn equal_to(obj: RefObject, _env: &dyn Environment) -> ResultRefObject {
    let (car1, cdr) = destructure_list(&obj)?;
    let (car2, _) = destructure_list(&cdr)?;
    if integer_value(&car1)? == integer_value(&car2)? {
        Object::Integer(1).into()
    } else {
        result_nil()
    }
}

pub fn quote(obj: RefObject, _env: &dyn Environment) -> ResultRefObject {
    let (car, _) = destructure_list(&obj)?;
    Ok(Arc::clone(car))
}

pub fn initialize_operators(environment: &mut dyn Environment) {
    environment.intern(
        String::from("QUOTE"),
        Arc::new(Some(Object::Operator(quote))),
    );
    environment.intern(
        String::from("AND"),
        Arc::new(Some(Object::Operator(and))),
    );
    environment.intern(
        String::from("OR"),
        Arc::new(Some(Object::Operator(or))),
    );
    environment.intern(
        String::from("NOT"),
        Arc::new(Some(Object::Operator(not))),
    );
    environment.intern(
        String::from("CAR"),
        Arc::new(Some(Object::Operator(car))),
    );
    environment.intern(
        String::from("CDR"),
        Arc::new(Some(Object::Operator(cdr))),
    );
    environment.intern(String::from("+"), Arc::new(Some(Object::Operator(sum))));
    environment.intern(String::from("-"), Arc::new(Some(Object::Operator(sub))));
    environment.intern(String::from("*"), Arc::new(Some(Object::Operator(mult))));
    environment.intern(String::from("/"), Arc::new(Some(Object::Operator(div))));
    environment.intern(
        String::from("="),
        Arc::new(Some(Object::Operator(equal_to))),
    );
    environment.intern(
        String::from("<"),
        Arc::new(Some(Object::Operator(less_than))),
    );
    environment.intern(
        String::from(">"),
        Arc::new(Some(Object::Operator(greater_than))),
    );
}
