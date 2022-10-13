use crate::eval::types::{Env, Expression, List, Value, EMPTY_LIST};
use anyhow::bail;

pub(crate) enum EvalError {}

#[derive(Clone)]
struct StackEntry {
    input: List,
    output: List,
    env: Env,
}

pub(crate) fn eval_list_iterative(list: List, env: Env) -> anyhow::Result<Expression> {
    let mut stack = vec![];
    let mut result = Expression::Value(Value::Nil);

    stack.push(StackEntry {
        input: list,
        output: EMPTY_LIST,
        env: env.clone(),
    });

    loop {
        let StackEntry {
            input: mut last_input,
            output: mut last_output,
            env: last_env,
        } = match stack.last_mut() {
            Some(entry) => entry.clone(),
            None => return Ok(result),
        };

        match last_input.take_mut() {
            Some(car) => {
                match car {
                    Expression::Value(value) => {
                        last_output.push_mut(Expression::Value(value.clone()))
                    }
                    Expression::List(list) => stack.push(StackEntry {
                        input: *list.clone(),
                        output: EMPTY_LIST,
                        env: env.clone(),
                    }),
                    Expression::Symbol(name) => {
                        //let value = env.get(name);
                        last_output.push_mut(Expression::Symbol(name));
                    }
                }
            }
            None => {
                result = match last_output.car() {
                    Some(callable) => {
                        let args = last_output.cdr().clone().into_iter().collect::<Vec<_>>();
                        let env = &last_env;

                        apply_fn(callable, &args, env)?
                    }
                    None => Expression::List(Box::new(EMPTY_LIST)),
                };

                let _ = stack.pop();

                if let Some(entry) = stack.last_mut() {
                    entry.output.push_mut(result.clone())
                }
            }
        }
    }
}

fn apply_fn(callable: &Expression, args: &[Expression], _env: &Env) -> anyhow::Result<Expression> {
    match callable {
        Expression::Symbol("+") => match args {
            [Expression::Value(Value::Int64(a)), Expression::Value(Value::Int64(b))] => {
                Ok(Expression::Value(Value::Int64(a + b)))
            }
            _ => bail!("Unsupported arguments"),
        },
        _ => bail!("Unsupported callable"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum() {
        let env = Env::new();

        let result = eval_list_iterative(
            vec![
                Expression::Symbol("+"),
                Expression::Value(Value::Int64(1)),
                Expression::Value(Value::Int64(2)),
            ]
            .into(),
            env,
        );
    }
}
