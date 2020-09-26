mod environment;
mod errors;
mod evaluator;
mod object;
mod reader;

use crate::environment::Environment;
use crate::evaluator::{operators::initialize_operators, Evaluator};
use crate::reader::tokenizer::Tokenizer;
use std::io::{self, Read, Write};

fn main() {
    let b = io::stdin().bytes();

    let tokenizer = Tokenizer::new(b);
    let mut environment = Environment::new();
    initialize_operators(&mut environment);
    let mut reader = reader::Reader::new(tokenizer);
    let mut evaluator = Evaluator::new(&mut environment);


    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let ast = reader.read().unwrap();
        if ast.as_ref().is_some() {
            let result = evaluator.eval(&ast).unwrap();
            if let Some(v) = result.as_ref() {
                println!("* {:?}", v);
            } else {
                println!("* None");
            }
            
        } else {
            break;
        }
    }

}
