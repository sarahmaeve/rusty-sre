use std::cell::RefCell;
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct Registry {
    services: RefCell<HashMap<String, String>>,
}

impl Registry {
    pub fn insert(&self, name: impl Into<String>, owner: impl Into<String>) {
        self.services.borrow_mut().insert(name.into(), owner.into());
    }

    pub fn contains(&self, name: &str) -> bool {
        self.services.borrow().contains_key(name)
    }

    pub fn rename(&self, old: &str, new: &str) -> bool {
        let mut services = self.services.borrow_mut();
        if self.contains(new) {
            return false;
        }
        let Some(owner) = services.remove(old) else {
            return false;
        };
        services.insert(new.to_owned(), owner);
        true
    }
}
