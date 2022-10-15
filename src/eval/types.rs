use crate::data::List;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct Env {}

impl Env {
    pub(crate) fn new() -> Self {
        Self {}
    }

    pub(crate) fn get(&self, name: &str) -> Expression {
        Expression::Value(Value::String(name.to_string()))
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

#[cfg(test)]
mod tests {
    use super::*;
}
