use crate::eval::types::Expression;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub(crate) struct Env {
    values: Rc<RefCell<HashMap<&'static str, Expression>>>,
    parent: Option<Box<Env>>,
}

impl Env {
    pub(crate) fn new() -> Self {
        let env = Self {
            values: Rc::default(),
            parent: None,
        };

        env
    }

    pub(crate) fn get(&self, name: &'static str) -> Option<Expression> {
        self.values
            .borrow()
            .get(name)
            .cloned()
            .or_else(|| self.parent.as_ref().map(|e| e.get(name)).flatten())
    }

    pub(crate) fn set(&mut self, name: &'static str, value: Expression) {
        self.values.borrow_mut().insert(name, value);
    }

    pub(crate) fn child(&self) -> Env {
        Env {
            values: Rc::default(),
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
