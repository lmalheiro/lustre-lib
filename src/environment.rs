use std::collections::HashMap;
use std::sync::Arc;

use crate::object;
use crate::object::{RefObject, Nil};

pub struct Environment {
    layers: Vec<HashMap<String, RefObject>>,
}

impl Environment {
    pub fn new() -> Self {
        let mut value = Self {
            layers: vec![HashMap::new()]
        };
        use crate::object::Environment;
        value.intern("nil".to_string(), Nil());
        value
    }
}

impl object::Environment for Environment {
    
    fn find_symbol(&self, symbol: &String) -> Option<RefObject> {
        eprintln!("FIND: {:?}", self.layers.last());

        let mut i = self.layers.iter().rev();

        loop {
            if let Some(layer) = i.next() {
                if let Some(value) = layer.get(symbol) {
                    break Some(Arc::clone(value));
                }
            } else {
                break None;
            };
        }
    }
    fn new_layer(&mut self) {
        self.layers
            .push(HashMap::<String, RefObject>::new());
    }
    fn drop_layer(&mut self) {
        assert!(self.layers.len() > 1); // Should not drop the last one
        self.layers.pop();
    }
    fn intern(&mut self, symbol: String, value: RefObject) -> RefObject {
        self.layers
            .last_mut()
            .unwrap()
            .insert(symbol, Arc::clone(&value));
        eprintln!("INTERN: {:?}", self.layers.last());
        value
    }
    fn unintern(&mut self, symbol: &String) {
        self.layers.last_mut().unwrap().remove(symbol);
    }
}
