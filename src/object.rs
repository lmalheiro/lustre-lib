//use std::fmt::Debug;
use std::fmt::Debug;
use std::fmt::Display;
use std::sync::Arc;
use crate::errors;


pub type RefObject = Arc<Option<Object>>;
pub type ResultRefObject = errors::Result<RefObject>;
pub type DestrucuturedCons = (RefObject, RefObject);
pub type ResultDestrucuturedCons =  errors::Result<DestrucuturedCons>;

pub fn Nil() -> RefObject {
    Arc::new(None)
}

pub fn ResultNil() -> ResultRefObject {
    Ok(Arc::new(None))
}

pub trait Environment {
    //fn get_nil(&self) -> RefObject;
    fn find_symbol(&self, symbol: &String) -> Option<RefObject>;
    fn new_layer(&mut self);
    fn drop_layer(&mut self);
    fn intern(&mut self, symbol: String, value: RefObject) -> RefObject;
    fn unintern(&mut self, symbol: &String);
}

type Op = fn(RefObject, &dyn Environment) -> ResultRefObject;

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

pub fn destructure_list(list: RefObject) -> ResultDestrucuturedCons {
    if let Some(Object::Cons(car, cdr)) = list.as_ref() {
        Ok((car.clone(), cdr.clone()))
    } else {
        Err(errors::Error::NotCons)
    }
}

pub fn symbol_value(sym: RefObject) -> errors::Result<String> {
    if let Some(Object::Symbol(value)) = sym.as_ref() {
        Ok(value.to_string())
    } else {
        Err(errors::Error::NotSymbol)
    }
}

pub fn integer_value(int: RefObject) -> errors::Result<i32> {
    if let Some(Object::Integer(value)) = int.as_ref() {
        Ok(*value)
    } else {
        Err(errors::Error::NotInteger)
    }   
}
