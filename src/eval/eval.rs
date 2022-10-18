use crate::data::List;
use crate::eval::basic_ops::{Divide, Multiply, Op, Subtract, Sum};
use crate::eval::types::{Env, Expression, Value};
use crate::list;
use anyhow::bail;
use std::mem::take;

struct StackEntry {
    operator: Option<Expression>,
    input: List<Expression>,
    output: List<Expression>,
    env: Env,
}

const SPECIAL_FORMS: [&str; 4] = ["+", "-", "/", "*"];

pub(crate) fn eval_iterative(exp: List<Expression>, env: Env) -> anyhow::Result<Expression> {
    let initial_entry = StackEntry {
        operator: None,
        input: exp,
        output: list![],
        env,
    };
    let mut stack = list![initial_entry];

    let mut last_return_value = Value::Nil.into();

    loop {
        match stack.pop_mut() {
            Some(StackEntry {
                mut operator,
                mut input,
                mut output,
                mut env,
            }) => {
                while let Some(expression) = input.pop_mut() {
                    match expression {
                        Expression::Symbol("lambda") if operator.is_none() => {
                            let lambda_args = match input.head() {
                                Some(Expression::List(args)) => List::clone(args),
                                Some(ex) => {
                                    bail!(
                                        "Syntax error: unexpected token in lambda arguments: {}",
                                        ex
                                    );
                                }
                                None => {
                                    bail!("Syntax error: lambda does not have arguments token");
                                }
                            };
                            let lambda_body = List::clone(input.tail());

                            output = output.push(
                                Value::Lambda {
                                    args: Box::from(lambda_args),
                                    body: Box::from(lambda_body),
                                    env: env.clone(),
                                }
                                .into(),
                            );
                        }
                        Expression::Symbol("macro") if operator.is_none() => {
                            let macro_args = match input.head() {
                                Some(Expression::List(args)) => List::clone(args),
                                Some(ex) => {
                                    bail!(
                                        "Syntax error: unexpected token in macro arguments: {}",
                                        ex
                                    );
                                }
                                None => {
                                    bail!("Syntax error: macro does not have arguments token");
                                }
                            };
                            let macro_body = List::clone(input.tail());

                            output = output.push(
                                Value::Macro {
                                    args: Box::from(macro_args),
                                    body: Box::from(macro_body),
                                }
                                .into(),
                            );
                        }
                        Expression::Symbol("def") => operator.replace(Expression::Symbol("def")),
                        expression => (),
                    }
                }

                match (input.pop_mut(), output.is_empty()) {
                    (Some(Expression::Symbol("def")), true) => {
                        output = output.push(Expression::Symbol("def"));
                    }
                    (Some(Expression::Symbol("macro")), true) => {
                        output = List::cons(Expression::Symbol("macro"), input);
                        input = List::new();
                    }
                    (Some(Expression::Symbol("lambda")), true) => {
                        output = List::cons(Expression::Symbol("lambda"), input);
                        input = List::new();
                    }
                    (Some(Expression::Symbol(name)), false)
                        if matches!(output.head(), Some(Expression::Symbol("def")))
                            && output.len() % 2 == 1 =>
                    {
                        output = output.push(Expression::Symbol(name));
                    }
                    (Some(Expression::Symbol(name)), _) if SPECIAL_FORMS.contains(&name) => {
                        output = output.push(Expression::Symbol(name));
                    }
                    (Some(Expression::Symbol(name)), _) => {
                        output = output.push(env.get(name)?);
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
                            } => apply_callable(&callable, &args, &mut env)?,
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

fn apply_callable(
    callable: &Expression,
    args: &List<Expression>,
    env: &mut Env,
) -> anyhow::Result<Expression> {
    Ok(match callable {
        Expression::Symbol("+") => Sum::apply(args, env)?,
        Expression::Symbol("*") => Multiply::apply(args, env)?,
        Expression::Symbol("-") => Subtract::apply(args, env)?,
        Expression::Symbol("/") => Divide::apply(args, env)?,
        Expression::Symbol("list") => List::clone(&args).into(),

        // special forms
        Expression::Symbol("lambda") => {
            let lambda_args = match args.head() {
                Some(Expression::List(args)) => List::clone(args),
                Some(ex) => bail!(
                    "Syntax error: lambda arguments should be a list, but it was: {}",
                    ex
                ),
                None => bail!("Syntax error: lambda should have arguments list"),
            };
            let lambda_body = List::clone(args.tail());

            Value::Lambda {
                args: Box::from(lambda_args),
                body: Box::from(lambda_body),
                env: env.clone(),
            }
            .into()
        }
        Expression::Symbol("macro") => {
            let lambda_args = match args.head() {
                Some(Expression::List(args)) => List::clone(args),
                Some(ex) => bail!(
                    "Syntax error: macro arguments should be a list, but it was: {}",
                    ex
                ),
                None => bail!("Syntax error: macro should have arguments list"),
            };
            let lambda_body = List::clone(args.tail());

            Value::Macro {
                args: Box::from(lambda_args),
                body: Box::from(lambda_body),
            }
            .into()
        }
        Expression::Symbol("def") => {
            let mut args = List::clone(&args);

            while !args.is_empty() {
                let name = match args.pop_mut() {
                    Some(Expression::Symbol(name)) => name,
                    Some(other) => bail!(
                        "Syntax error: variable name should be a symbol, but it was: {}",
                        other
                    ),
                    None => bail!("Syntax error: def should have even number of arguments"),
                };
                let value = match args.pop_mut() {
                    Some(expr) => expr,
                    None => bail!("Syntax error: def should have even number of arguments"),
                };

                env.set(name, value);
            }

            Value::Nil.into()
        }

        // exception
        other => bail!("Expression is not callable type: {}", other),
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

    #[test]
    fn test_lambda_definition() {
        let env = Env::new();
        let expression: List<_> = list![
            Expression::Symbol("lambda"),
            list![Expression::Symbol("a")].into(),
            list![
                Expression::Symbol("+"),
                Expression::Symbol("a"),
                Value::Int64(3).into()
            ]
            .into()
        ];

        let result = eval_iterative(expression, env.clone()).unwrap();

        assert_eq!(
            Expression::Value(Value::Lambda {
                args: Box::from(list![Expression::Symbol("a")]),
                body: Box::from(list![list![
                    Expression::Symbol("+"),
                    Expression::Symbol("a"),
                    Value::Int64(3).into()
                ]
                .into()]),
                env,
            }),
            result
        )
    }

    #[test]
    fn test_macro_definition() {
        let env = Env::new();
        let expression: List<_> = list![
            Expression::Symbol("macro"),
            list![Expression::Symbol("a")].into(),
            list![
                Expression::Symbol("+"),
                Expression::Symbol("a"),
                Value::Int64(3).into()
            ]
            .into()
        ];

        let result = eval_iterative(expression, env.clone()).unwrap();

        assert_eq!(
            Expression::Value(Value::Macro {
                args: Box::from(list![Expression::Symbol("a")]),
                body: Box::from(list![list![
                    Expression::Symbol("+"),
                    Expression::Symbol("a"),
                    Value::Int64(3).into()
                ]
                .into()]),
            }),
            result
        )
    }

    #[test]
    fn test_def_definition() {
        let mut env = Env::new();
        let expression: List<_> = list![
            Expression::Symbol("def"),
            Expression::Symbol("a"),
            list![
                Expression::Symbol("+"),
                Value::Int64(1).into(),
                Value::Int64(2).into()
            ]
            .into()
        ];

        let result = eval_iterative(expression, env.clone()).unwrap();

        assert_eq!(Expression::Value(Value::Nil), result);

        assert_eq!(Expression::Value(Value::Int64(3)), env.get("a").unwrap())
    }
}
