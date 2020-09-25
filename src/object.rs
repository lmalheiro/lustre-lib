//use std::fmt::Debug;
use std::fmt::Debug;
use std::fmt::Display;
use std::rc::Rc;

pub type RefObject = Rc<Option<Object>>;

pub trait Environment {
    fn get_nil(&self) -> RefObject;
    fn find_symbol(&self, symbol: &String) -> Option<RefObject>;
    fn new_layer(&mut self);
    fn drop_layer(&mut self);
    fn intern(&mut self, symbol: String, value: RefObject) -> RefObject;
    fn unintern(&mut self, symbol: &String);
}

type Op = fn(RefObject, &dyn Environment) -> RefObject;

pub enum Object {
    Integer(i32),
    IString(String),
    Cons(RefObject, RefObject),
    Function(RefObject),
    Operator(Op),
    Symbol(String),
}

impl Debug for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Object::Cons(_, _) => {
                write!(f, "(");
                self.helper_fmt(f)?;
                write!(f, " )")
            }
            _ => self.helper_fmt(f),
        }
    }
}

impl Object {
    fn helper_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Object::Integer(v) => write!(f, "{}", v),
            Object::IString(v) => write!(f, "{}", v),
            Object::Symbol(v) => write!(f, "{}", v),
            Object::Cons(car, cdr) => {
                write!(
                    f,
                    " {}",
                    car.as_ref()
                        .as_ref()
                        .or(Some(&Object::IString(String::from(""))))
                        .unwrap()
                );
                cdr.as_ref()
                    .as_ref()
                    .or(Some(&Object::IString(String::from(""))))
                    .unwrap()
                    .helper_fmt(f)
            }
            Object::Function(_) => write!(f, "FUNCTION"),
            Object::Operator(_) => write!(f, "OPERATOR"),
        }
    }
    
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Object::Cons(_, _) => {
                write!(f, "(");
                self.helper_fmt(f)?;
                write!(f, " )")
            }
            _ => self.helper_fmt(f),
        }
    }
}


pub fn is_nil(obj: &RefObject) -> bool {
    match obj.as_ref() {
        Some(_) => false,
        None => true
    }
}

pub fn not_nil(obj: &RefObject) -> bool {
    match obj.as_ref() {
        Some(_) => true,
        None => false
    }
}

pub fn destructure_list(list: RefObject) -> (RefObject, RefObject) {
    if let Some(Object::Cons(car, cdr)) = list.as_ref() {
        (car.clone(), cdr.clone())
    } else {
        panic!("Not a list!");
    }
}

pub fn symbol_value(sym: RefObject) -> String {
    if let Some(Object::Symbol(value)) = sym.as_ref() {
        value.to_string()
    } else {
        panic!("Not a symbol!");
    }
}

pub fn integer_value(int: RefObject) -> i32 {
    if let Some(Object::Integer(value)) = int.as_ref() {
        *value
    } else {
        panic!("Not an integer!");
    }   
}
