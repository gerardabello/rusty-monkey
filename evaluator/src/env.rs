use super::object::Object;
use std::collections::HashMap;

pub struct Environment {
    store: HashMap<String, Object>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }

    pub fn set(&mut self, name: String, value: Object) {
        self.store.insert(name, value);
    }

    pub fn get(&self, name: &str) -> Object {
        match self.store.get(name) {
            None => Object::Null,
            Some(v) => v.clone(),
        }
    }
}
