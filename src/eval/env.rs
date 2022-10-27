use crate::eval::types::Expression;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub(crate) struct Env {
    values: Arc<Mutex<HashMap<&'static str, Expression>>>,
    parent: Option<Box<Env>>,
}

impl Env {
    pub(crate) fn new() -> Self {
        let env = Self {
            values: Arc::new(Mutex::new(HashMap::new())),
            parent: None,
        };

        env
    }

    pub(crate) fn get(&self, name: &'static str) -> Option<Expression> {
        self.values
            .lock()
            .unwrap()
            .get(name)
            .map(Clone::clone)
            .or_else(|| self.parent.as_ref().map(|e| e.get(name)).flatten())
    }

    pub(crate) fn set(&self, name: &'static str, value: Expression) {
        self.values.lock().unwrap().insert(name, value);
    }

    pub(crate) fn child(&self) -> Env {
        Env {
            values: Arc::new(Mutex::new(HashMap::new())),
            parent: Some(Box::from(self.clone())),
        }
    }
}

impl PartialEq for Env {
    fn eq(&self, _other: &Self) -> bool {
        true
    }

    fn ne(&self, _other: &Self) -> bool {
        false
    }
}
