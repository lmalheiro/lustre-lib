//use std::fmt::Debug;
use std::fmt::Debug;
use std::fmt::Display;
use std::rc::Rc;

pub trait Environment {
    fn get_nil(&self) -> Rc<Option<Object>>;
    fn find_symbol(&self, symbol: &String) -> Option<Rc<Option<Object>>>;
    fn new_layer(&mut self);
    fn drop_layer(&mut self);
    fn intern(&mut self, symbol: String, value: Rc<Option<Object>>) -> Rc<Option<Object>>;
    fn unintern(&mut self, symbol: &String);
}

type Op = fn(Rc<Option<Object>>, &dyn Environment) -> Rc<Option<Object>>;

pub enum Object {
    Integer(i32),
    IString(String),
    Cons(Rc<Option<Object>>, Rc<Option<Object>>),
    Function(Rc<Option<Object>>),
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
