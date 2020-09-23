mod operators;

use std::rc::Rc;

use crate::object::*;

struct Evaluator<'a> {
    environment: &'a mut dyn Environment,
}

impl <'a> Evaluator<'a> {
    pub fn new(environment: &'a mut dyn Environment) -> Self {
        environment.intern(
            String::from("+"),
            Rc::new(Some(Object::Operator(operators::sum))),
        );
        environment.intern(
            String::from("-"),
            Rc::new(Some(Object::Operator(operators::sub))),
        );
        Evaluator { environment }
    }

    pub fn eval(&self, obj: Rc<Option<Object>>) -> Rc<Option<Object>> {
        if Rc::ptr_eq(&obj, &self.environment.get_nil()) {
            return self.environment.get_nil();
        }
        match obj
            .as_ref()
            .as_ref()
            .expect("Invalid 'None' object. It should have matched the 'nil'.")
        {
            Object::Integer(_) => obj,
            Object::IString(_) => obj,
            Object::Symbol(_) => obj,
            Object::Cons(_, _) => unimplemented!(),
            Object::Function(_) => unimplemented!(),
            Object::Operator(_) => unimplemented!(),
        }
    }

    fn eva_list(&self, obj: Rc<Option<Object>>) -> Rc<Option<Object>> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::prelude::*;
    use std::io::Cursor;
    use crate::environment;
    use crate::reader;

    #[test]
    fn eval_test() {
        let input = "1000";
        let tokenizer = reader::tokenizer::Tokenizer::new(Cursor::new(input).bytes());
        let mut environment = environment::Environment::new();
        let value = reader::Reader::new(tokenizer, &mut environment).read().unwrap();
        eprintln!(">>>>>>{:?}", value);
        if let Some(_) = value.as_ref() {
            let evaluator = Evaluator::new(&mut environment);
            let result = evaluator.eval(value);
            if let Some(obj) = result.as_ref() {
                eprintln!("result: {}", obj);
            }
            
        } else {
            panic!("Ooops! Not an object...")
        }

        
    }
}