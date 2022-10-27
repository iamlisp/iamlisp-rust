use crate::data::List;
use crate::eval::native_calls::{Divide, Multiply, Op, Subtract, Sum};
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use std::ops::Deref;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub(crate) struct NativeCall(pub(crate) Arc<Box<dyn Op>>);

impl Deref for NativeCall {
    type Target = Arc<Box<dyn Op>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl PartialEq for NativeCall {
    fn eq(&self, other: &Self) -> bool {
        self.0.name() == other.0.name()
    }

    fn ne(&self, other: &Self) -> bool {
        self.0.name() != other.0.name()
    }
}

impl Into<Expression> for NativeCall {
    fn into(self) -> Expression {
        Expression::Value(Value::NativeCall(self))
    }
}

impl Debug for NativeCall {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NativeCall")
            .field("name", &self.0.name())
            .finish()
    }
}

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

        env.set("+", NativeCall(Arc::new(Box::from(Sum))).into());
        env.set("-", NativeCall(Arc::new(Box::from(Subtract))).into());
        env.set("/", NativeCall(Arc::new(Box::from(Divide))).into());
        env.set("*", NativeCall(Arc::new(Box::from(Multiply))).into());

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

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum Value {
    Int64(i64),
    Float64(f64),
    String(String),
    Bool(bool),
    Nil,
    NativeCall(NativeCall),
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
            Expression::Value(Value::Int64(int)) => format!("{}", int),
            Expression::Value(Value::Float64(float)) => format!("{}", float),
            Expression::Value(Value::String(string)) => format!("{}", string),
            Expression::Value(Value::Bool(bool)) => format!("{}", bool),
            Expression::Value(Value::Nil) => "Nil".to_string(),
            Expression::Value(Value::NativeCall(c)) => c.0.name().to_string(),
            Expression::Symbol(symbol) => format!("{}", symbol),
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
