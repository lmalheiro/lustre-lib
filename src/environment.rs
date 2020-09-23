use std::collections::HashMap;
use std::rc::Rc;

use crate::object;
use crate::object::Object;

pub struct Environment {
    layers: Vec<HashMap<String, Rc<Option<Object>>>>,
    nil: Rc<Option<Object>>,
}

impl Environment {
    pub fn new() -> Self {
        let mut value = Self {
            layers: vec![HashMap::new()],
            nil: Rc::new(None),
        };
        use crate::object::Environment;
        value.intern("nil".to_string(), Rc::clone(&value.nil));
        value
    }
}

impl object::Environment for Environment {
    fn get_nil(&self) -> Rc<Option<Object>> {
        Rc::clone(&self.nil)
    }

    fn find_symbol(&self, symbol: &String) -> Option<Rc<Option<Object>>> {
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
    fn new_layer(&mut self) {
        self.layers
            .push(HashMap::<String, Rc<Option<Object>>>::new());
    }
    fn drop_layer(&mut self) {
        assert!(self.layers.len() > 1); // Should not drop the last one
        self.layers.pop();
    }
    fn intern(&mut self, symbol: String, value: Rc<Option<Object>>) -> Rc<Option<Object>> {
        self.layers
            .last_mut()
            .unwrap()
            .insert(symbol, Rc::clone(&value));
        value
    }
    fn unintern(&mut self, symbol: &String) {
        self.layers.last_mut().unwrap().remove(symbol);
    }
}
