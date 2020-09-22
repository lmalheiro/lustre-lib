use std::collections::HashMap;
use std::rc::Rc;

use crate::object::Object;

pub struct Environment {
    layers: Vec<HashMap<String, Rc<Option<Object>>>>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            layers: vec![HashMap::new()],
        }
    }
    pub fn find_symbol(&self, symbol: &String) -> Option<Rc<Option<Object>>> {
        let mut i = self.layers.iter().rev();

        loop {
            if let Some(layer) = i.next() {
                if let Some(value) = layer.get(symbol) {
                    break Some(Rc::clone(value));
                }
            } else {
                break None;
            };
        }
    }
    pub fn new_layer(&mut self) {
        self.layers.push(HashMap::<String, Rc<Option<Object>>>::new());
    }
    pub fn drop_layer(&mut self) {
        assert!(self.layers.len() > 1); // Should not drop the last one
        self.layers.pop();
    }
    pub fn intern(&mut self, symbol: String, value: Object) -> Rc<Option<Object>> {
        let value = Rc::new(Some(value));
        self.layers.last_mut().unwrap().insert(symbol, Rc::clone(&value));
        value
    }
    pub fn unintern(&mut self, symbol: &String) {
        self.layers.last_mut().unwrap().remove(symbol);
    }
}
