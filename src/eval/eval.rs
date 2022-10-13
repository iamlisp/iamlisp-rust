use crate::eval::types::{Env, Expression, List, Value, EMPTY_LIST};
use std::collections::HashMap;

#[derive(Clone)]
pub(crate) struct Env {
    values: HashMap<&'static str, Expression>,
}

impl Env {
    pub(crate) fn get(&self, name: &str) -> Expression {
        Expression::Value(Value::String(name.to_string()))
    }
}

#[derive(Clone)]
struct StackEntry {
    input: List,
    output: Vec<Expression>,
    env: Env,
}

fn eval_list_iterative(list: List, env: Env) -> Expression {
    let mut stack = vec![];
    let mut result = Expression::Value(Value::Nil);

    stack.push(StackEntry {
        input: list,
        output: vec![],
        env: env.clone(),
    });

    loop {
        let mut last_entry = match stack.last_mut().cloned() {
            Some(entry) => entry,
            None => return result,
        };

        match last_entry.input.car() {
            Some(car) => {
                match car {
                    Expression::Value(value) => {
                        last_entry.output.push(Expression::Value(value.clone()))
                    }
                    Expression::List(list) => stack.push(StackEntry {
                        input: *list.clone(),
                        output: vec![],
                        env: env.clone(),
                    }),
                    Expression::Symbol(name) => {
                        let value = env.get(name);
                        last_entry.output.push(value);
                    }
                }
                last_entry.input = last_entry.input.cdr().clone()
            }
            None => {
                result = if last_entry.output.is_empty() {
                    Expression::List(Box::new(EMPTY_LIST))
                } else {
                    let callable = &last_entry.output[0];
                    let args = &last_entry.output[1..];

                    apply_fn(callable, args, &last_entry.env)
                };

                let _ = stack.pop();

                if let Some(entry) = stack.last_mut() {
                    entry.output.push(result.clone())
                }
            }
        }
    }
}

fn apply_fn(_callable: &Expression, _args: &[Expression], _env: &Env) -> Expression {
    Expression::Value(Value::Nil)
}
