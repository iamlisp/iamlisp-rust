use crate::data::List;
use crate::eval::basic_ops::{Divide, Multiply, Op, Subtract, Sum};
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
    let initial_entry = StackEntry {
        input: exp,
        output: list![],
        env,
    };
    let mut stack = list![initial_entry];

    let mut last_return_value = Value::Nil.into();

    loop {
        match stack.pop_mut() {
            Some(StackEntry {
                mut input,
                mut output,
                env,
            }) => {
                match (input.pop_mut(), output.is_empty()) {
                    // check special forms only if they are at begin of the list
                    // we assume that "begin" means that output is empty
                    (Some(Expression::Symbol("def")), true) => todo!(),
                    (Some(Expression::Symbol("macro")), true) => {
                        let args = match input.pop_mut() {
                            Some(Expression::List(args)) => args,
                            _ => bail!("Unexpected type of lambda arguments"),
                        };
                        output = output.push(
                            Value::Macro {
                                args,
                                body: Box::new(input),
                            }
                            .into(),
                        );
                        input = List::new();
                    }
                    (Some(Expression::Symbol("lambda")), true) => {
                        let env = env.clone();
                        let args = match input.pop_mut() {
                            Some(Expression::List(args)) => args,
                            _ => bail!("Unexpected type of lambda arguments"),
                        };
                        output = output.push(
                            Value::Lambda {
                                args,
                                env,
                                body: Box::new(input),
                            }
                            .into(),
                        );
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
                        let result = match take(&mut output) {
                            List::Normal {
                                car: callable,
                                cdr: args,
                            } => apply_fn(&callable, &args, &env)?,
                            List::Empty => List::new().into(),
                        };

                        if let Some(StackEntry {
                            output: prev_output,
                            ..
                        }) = stack.head_mut()
                        {
                            *prev_output = take(prev_output).push(result);
                        } else {
                            last_return_value = result;
                        }
                        continue;
                    }
                }
                stack = stack.unshift(StackEntry { input, output, env });
            }
            None => return Ok(last_return_value),
        }
    }
}

fn apply_fn(
    callable: &Expression,
    args: &List<Expression>,
    env: &Env,
) -> anyhow::Result<Expression> {
    Ok(match callable {
        Expression::Symbol("+") => Sum::apply(args, env)?,
        Expression::Symbol("*") => Multiply::apply(args, env)?,
        Expression::Symbol("-") => Subtract::apply(args, env)?,
        Expression::Symbol("list") => List::clone(&args).into(),
        expression => bail!("Expression is not callable type: {}", expression),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval_empty_list_into_empty_list() {
        let env = Env::new();
        let exp = list![];

        let result = eval_iterative(exp, env).unwrap();

        assert_eq!(Expression::List(Box::new(list![])), result)
    }

    #[test]
    fn test_eval_sum_of_two_numbers() {
        let env = Env::new();

        // i64 + i64
        {
            let exp = list![
                Expression::Symbol("+"),
                Value::Int64(2).into(),
                Value::Int64(3).into()
            ];

            assert_eq!(
                Expression::Value(Value::Int64(5)),
                eval_iterative(exp, env.clone(),).unwrap()
            );
        };

        // f64 + f64
        {
            let exp = list![
                Expression::Symbol("+"),
                Value::Float64(2.5).into(),
                Value::Float64(3.2).into()
            ];

            assert_eq!(
                Expression::Value(Value::Float64(5.7)),
                eval_iterative(exp, env.clone(),).unwrap()
            );
        }

        // f64 + i64
        {
            let exp = list![
                Expression::Symbol("+"),
                Value::Float64(2.5).into(),
                Value::Int64(3).into()
            ];

            assert_eq!(
                Expression::Value(Value::Float64(5.5)),
                eval_iterative(exp, env.clone(),).unwrap()
            );
        };

        // i64 + f64
        {
            let exp = list![
                Expression::Symbol("+"),
                Value::Int64(2).into(),
                Value::Float64(3.2).into()
            ];

            assert_eq!(
                Expression::Value(Value::Int64(5)),
                eval_iterative(exp, env.clone(),).unwrap()
            );
        }
    }

    #[test]
    fn test_eval_nested_sum() {
        let env = Env::new();
        let exp1: List<_> = list![
            Expression::Symbol("+"),
            Value::Int64(2).into(),
            Value::Int64(3).into()
        ];
        let exp2: List<_> = list![
            Expression::Symbol("+"),
            exp1.into(),
            Value::Int64(10).into()
        ];

        let result = eval_iterative(exp2, env).unwrap();

        assert_eq!(Expression::Value(Value::Int64(15)), result)
    }
}
