use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use crate::object::{nil, RefObject};

type Symbols = RwLock<HashMap<String, RefObject>>;

#[derive(Clone)]
pub struct RefEnvironment(pub Arc<RwLock<Environment>>);

pub struct Environment {
    previous: Option<RefEnvironment>,
    symbols: Symbols,
}

impl Environment {
    fn new() -> Self {
        let mut value = Self {
            previous: None,
            symbols: RwLock::new(HashMap::new()),
        };
        value.intern("nil".to_string(), nil());
        value
    }
}

impl RefEnvironment {
    pub fn new() -> Self {
        Self(Arc::new(RwLock::new(Environment::new())))
    }
    pub fn from(previous: &RefEnvironment) -> RefEnvironment {

        Self (Arc::new(RwLock::new(Environment {
            previous: Some(previous.clone()),
            symbols: RwLock::new(HashMap::new()),
        })))
    }
}

impl Environment {
    pub fn find_symbol(&self, symbol: &String) -> Option<RefObject> {
        if let Some(value) = self.symbols.read().unwrap().get(&symbol.to_uppercase()) {
            Some(Arc::clone(value))
        } else {
            if let Some(previous) = &self.previous {
                previous.0.read().unwrap().find_symbol(symbol)
            } else {
                None
            }
        }
    }
    
    pub fn intern(&mut self, symbol: String, value: RefObject) -> RefObject {
        let mut symbols = self.symbols.write().unwrap();
        symbols.insert(symbol.to_uppercase(), Arc::clone(&value));
        value
    }
    pub fn unintern(&mut self, symbol: &String) {
        let mut symbols = self.symbols.write().unwrap();
        symbols.remove(symbol);
    }
}

//unsafe impl<'a> Sync for Environment {}
