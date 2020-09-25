
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Trying to destructure an object that is not  Object::Cons.")]
    NotCons,
    #[error("Expecting and integer.")]
    NotInteger,
    #[error("Not a symbol.")]
    NotSymbol,
    #[error("Input error.")]
    InputError {
        #[from]
        source: rustf8::Utf8IteratorError,
    },
    #[error("Parsing error.")]
    ParseError {
        #[from]
        source: std::num::ParseIntError
    },
}