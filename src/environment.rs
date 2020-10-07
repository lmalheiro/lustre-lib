use std::sync::Mutex;
use std::collections::HashMap;
use std::sync::Arc;

use crate::object::{nil, RefObject};

pub struct Environment<'a> {
    previous: Option<&'a Environment<'a>>,
    symbols: Mutex<HashMap<String, RefObject>>,
}

impl<'a> Environment<'a> {
    pub fn new() -> Self {
        let mut value = Self {
            previous: None,
            symbols: Mutex::new(HashMap::new()),
        };
        value.intern("nil".to_string(), nil());
        value
    }
}

impl<'a> Environment<'a> {
    pub fn find_symbol(&self, symbol: &String) -> Option<RefObject> {
        let symbols = self.symbols.lock().unwrap();
        if let Some(value) = symbols.get(&symbol.to_uppercase()) {
            Some(Arc::clone(value))
        } else {
            drop(symbols);
            if let Some(previous) = self.previous {
                previous.find_symbol(symbol)
            } else {
                None
            }
        }
    }
    pub fn from(previous: &'a Self) -> Environment<'a> {
        Self {
            previous: Some(previous),
            symbols: Mutex::new(HashMap::new()),
        }
    }
    pub fn intern(&mut self, symbol: String, value: RefObject) -> RefObject {
        let mut symbols = self.symbols.lock().unwrap();
        symbols
            .insert(symbol.to_uppercase(), Arc::clone(&value));
        value
    }
    pub fn unintern(&mut self, symbol: &String) {
        let mut symbols = self.symbols.lock().unwrap();
        symbols.remove(symbol);
    }
}

unsafe impl<'a> Sync for Environment<'a> {}

