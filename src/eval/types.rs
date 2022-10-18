use crate::data::List;
use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub(crate) struct Env {
    values: Arc<Mutex<HashMap<&'static str, Expression>>>,
    parent: Option<Box<Env>>,
}

impl Env {
    pub(crate) fn new() -> Self {
        Self {
            values: Arc::new(Mutex::new(HashMap::new())),
            parent: None,
        }
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

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum Value {
    Int64(i64),
    Float64(f64),
    String(String),
    Nil,
    Lambda {
        env: Env,
        args: Box<List<Expression>>,
        body: Box<List<Expression>>,
    },
    Macro {
        args: Box<List<Expression>>,
        body: Box<List<Expression>>,
    },
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum Expression {
    Value(Value),
    List(Box<List<Expression>>),
    Symbol(&'static str),
}

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Expression::List(l) => format!("{}", l),
            Expression::Value(Value::Int64(i)) => format!("{}", i),
            Expression::Value(Value::Float64(f)) => format!("{}", f),
            Expression::Value(Value::String(s)) => format!("{}", s),
            Expression::Value(Value::Nil) => "Nil".to_string(),
            Expression::Symbol(s) => format!("{}", s),
            Expression::Value(Value::Lambda { args, body, env: _ }) => {
                format!("(lambda {} {})", args, body)
            }
            Expression::Value(Value::Macro { args, body }) => {
                format!("(macro {} {})", args, body)
            }
        };

        write!(f, "{}", str)
    }
}

impl Into<Expression> for Value {
    fn into(self) -> Expression {
        Expression::Value(self)
    }
}

impl Into<Expression> for List<Expression> {
    fn into(self) -> Expression {
        Expression::List(Box::from(self))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
