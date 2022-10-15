use crate::data::List;
use crate::eval::types::{Env, Expression, Value};
use crate::list;
use anyhow::bail;
use std::mem::take;

struct StackEntry {
    input: List<Expression>,
    output: List<Expression>,
    env: Env,
}

pub(crate) fn eval_iterative(exp: List<Expression>, env: Env) -> anyhow::Result<Expression> {
    let stack_entry = StackEntry {
        input: exp,
        output: list![],
        env,
    };

    let mut stack = list![stack_entry];
    let mut last_return_value = Expression::Value(Value::Nil);

    loop {
        match stack.pop_mut() {
            Some(StackEntry {
                mut input,
                mut output,
                env,
            }) => {
                match (input.pop_mut(), output.is_empty()) {
                    (Some(Expression::Symbol("def")), true) => todo!(),
                    (Some(Expression::Symbol("list")), true) => todo!(),
                    (Some(Expression::Symbol("macro")), true) => {
                        let args = match input.pop_mut() {
                            Some(Expression::List(args)) => args,
                            _ => bail!("Unexpected type of lambda arguments"),
                        };
                        output = output.push(Expression::Value(Value::Macro {
                            args,
                            body: Box::new(input),
                        }));
                        input = List::new();
                    }
                    (Some(Expression::Symbol("lambda")), true) => {
                        let env = env.clone();
                        let args = match input.pop_mut() {
                            Some(Expression::List(args)) => args,
                            _ => bail!("Unexpected type of lambda arguments"),
                        };
                        output = output.push(Expression::Value(Value::Lambda {
                            args,
                            env,
                            body: Box::new(input),
                        }));
                        input = List::new();
                    }
                    (Some(Expression::Symbol(name)), _) => {
                        output = output.push(env.get(name));
                    }
                    (Some(Expression::List(list)), _) => {
                        stack = stack
                            .unshift(StackEntry {
                                input,
                                output,
                                env: env.clone(),
                            })
                            .unshift(StackEntry {
                                input: *list,
                                output: List::new(),
                                env: env.clone(),
                            });
                        continue;
                    }
                    (Some(expression), _) => output = output.push(expression),
                    (None, _) => {
                        let result = match output {
                            List::Normal {
                                car: callable,
                                cdr: args,
                            } => apply_fn(&callable, &args, &env)?,
                            List::Empty => Expression::List(Box::new(List::new())),
                        };
                        output = List::new();

                        if let Some(StackEntry {
                            output: prev_output,
                            ..
                        }) = stack.head_mut()
                        {
                            *prev_output = take(prev_output).unshift(result);
                        } else {
                            last_return_value = result;
                        }
                    }
                }
                stack = stack.unshift(StackEntry { input, output, env });
            }
            None => {
                break;
            }
        }
    }

    Ok(last_return_value)
}

fn apply_fn(
    callable: &Expression,
    args: &List<Expression>,
    _env: &Env,
) -> anyhow::Result<Expression> {
    let args_vec = List::clone(&args).into_iter().collect::<Vec<_>>();

    match callable {
        Expression::Symbol("+") => match &args_vec.as_slice() {
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
    fn test_eval_empty_list_into_empty_list() {
        let env = Env::new();
        let exp = list![];

        let result = eval_iterative(exp, env).ok();

        assert_eq!(Some(Expression::List(Box::new(list![]))), result)
    }
}
