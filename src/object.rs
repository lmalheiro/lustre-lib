use crate::errors::{self, Error};
use std::fmt::Debug;
use std::fmt::Display;
use std::sync::Arc;

pub type RefObject = Arc<Option<Object>>;
pub type ResultRefObject = errors::Result<RefObject>;
pub type DestrucuturedCons<'a> = (&'a RefObject, &'a RefObject);
pub type ResultDestrucuturedCons<'a> = errors::Result<DestrucuturedCons<'a>>;

pub fn nil() -> RefObject {
    Arc::new(None)
}

pub fn result_nil() -> ResultRefObject {
    Ok(Arc::new(None))
}

type Op = fn(RefObject) -> ResultRefObject;

pub enum Object {
    Integer(i32),
    IString(String),
    Cons(RefObject, RefObject),
    Lambda(RefObject, RefObject),
    Operator(String, Op),
    Symbol(String),
}

unsafe impl Send for Object {

}

impl PartialEq for Object {
    fn eq(&self, other: &Object) -> bool {
        use Object::*;
        match (self, other) {
            (Integer(v1), Integer(v2)) => v1 == v2,
            (IString(v1), IString(v2)) => v1 == v2,
            (Cons(v11, v12), Cons(v21, v22)) => v11.as_ref() == v21.as_ref() && v12 == v22,
            (Lambda(v11, v12), Lambda(v21, v22)) => v11.as_ref() == v21.as_ref() && v12 == v22,
            (Operator(n1, _), Operator(n2, _)) => n1 == n2,
            (Symbol(v1), Symbol(v2)) => v1 == v2,
            (_, _) => false,
        }
    }
}

impl Debug for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Object::Cons(_, _) => {
                write!(f, "(")?;
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
            Object::IString(v) => write!(f, "\"{}\"", v),
            Object::Symbol(v) => write!(f, "{}", v),
            Object::Cons(car, cdr) => {
                if let Some(v) = car.as_ref().as_ref() {
                    write!(f, " {}", v)?
                }
                if let Some(v) = cdr.as_ref().as_ref() {
                    v.helper_fmt(f)
                } else {
                    Ok(())
                }
            }
            Object::Lambda(params, expression) => {
                write!(f, "( LAMBDA ")?;
                if let Some(v) = params.as_ref().as_ref() {
                    write!(f, " {}", v)?
                } else {
                    write!(f, " ()")?
                }
                if let Some(v) = expression.as_ref().as_ref() {
                    write!(f, " {} )", v)
                } else {
                    write!(f, " )")
                } 
            }
            Object::Operator(n, _) => write!(f, "{}", n),
        }
    }
}

impl From<Object> for ResultRefObject {
    fn from(obj: Object) -> Self {
        Ok(Arc::new(Some(obj)))
    }
}

impl From<Object> for Box<ResultRefObject> {
    fn from(obj: Object) -> Self {
        Box::new(Ok(Arc::new(Some(obj))))
    }
}

impl From<Object> for RefObject {
    fn from(obj: Object) -> Self {
        Arc::new(Some(obj))
    }
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Object::Cons(_, _) => {
                write!(f, "(")?;
                self.helper_fmt(f)?;
                write!(f, " )")
            }
            _ => self.helper_fmt(f),
        }
    }
}

pub fn not_nil(obj: &RefObject) -> bool {
    match obj.as_ref() {
        Some(_) => true,
        None => false,
    }
}

pub fn destructure_list<'a>(list: &'a RefObject) -> ResultDestrucuturedCons<'a> {
    if let Some(Object::Cons(car, cdr)) = list.as_ref() {
        Ok((car, cdr))
    } else {
        Err(Error::NotCons)
    }
}

pub fn symbol_value(sym: &RefObject) -> errors::Result<String> {
    if let Some(Object::Symbol(value)) = sym.as_ref() {
        Ok(value.to_string())
    } else {
        Err(Error::NotSymbol)
    }
}

pub fn integer_value(int: &RefObject) -> errors::Result<i32> {
    if let Some(Object::Integer(value)) = int.as_ref() {
        Ok(*value)
    } else {
        Err(Error::NotInteger)
    }
}
