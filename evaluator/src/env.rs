use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use super::object::Object;

pub struct Environment {
    store: HashMap<String, Object>,
    parent: Option<Rc<RefCell<Environment>>>,
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}

impl Environment {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
            parent: None,
        }
    }

    pub fn with_parent(parent: &Rc<RefCell<Environment>>) -> Self {
        Self {
            store: HashMap::new(),
            parent: Some(Rc::clone(parent)),
        }
    }

    pub fn set_rr(rr: &Rc<RefCell<Self>>, name: String, value: Object) {
        let mut mutref = rr.borrow_mut();
        mutref.set(name, value);
    }

    pub fn get_rr(rr: &Rc<RefCell<Self>>, name: &str) -> Object {
        let mutref = rr.borrow();
        mutref.get(name)
    }

    pub fn set(&mut self, name: String, value: Object) {
        self.store.insert(name, value);
    }

    pub fn get(&self, name: &str) -> Object {
        match self.store.get(name) {
            Some(v) => v.clone(),
            None => match &self.parent {
                Some(parent) => Self::get_rr(&parent, name),
                None => Object::Null,
            },
        }
    }
}
