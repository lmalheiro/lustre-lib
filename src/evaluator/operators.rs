use crate::object::Object;
use std::rc::Rc;
use crate::object::Environment;


pub fn sum(obj: Rc<Option<Object>>, env: &dyn Environment) -> Rc<Option<Object>> {
    let mut total = 0i32;
    let mut next = &obj;
    while !Rc::ptr_eq(next, &env.get_nil()) {
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

pub fn sub(obj: Rc<Option<Object>>, env: &Environment) -> Rc<Option<Object>> {
    unimplemented!()
}
