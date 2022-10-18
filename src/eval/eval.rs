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

const SPECIAL_FORMS: [&str; 4] = ["+", "-", "/", "*"];

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
            Some(mut stack_entry) => {
                match stack_entry.input.pop_mut() {
                    Some(Expression::Symbol("lambda")) if stack_entry.output.is_empty() => {
                        let lambda_args = match stack_entry.input.head() {
                            Some(Expression::List(args)) => List::clone(args),
                            Some(ex) => {
                                bail!("Syntax error: unexpected token in lambda arguments: {}", ex);
                            }
                            None => {
                                bail!("Syntax error: lambda does not have arguments token");
                            }
                        };
                        let lambda_body = List::clone(stack_entry.input.tail());

                        stack_entry.input = List::new();
                        stack_entry.output = stack_entry.output.push(Expression::Symbol("quote"));
                        stack_entry.output = stack_entry.output.push(
                            Value::Lambda {
                                args: Box::from(lambda_args),
                                body: Box::from(lambda_body),
                                env: stack_entry.env.clone(),
                            }
                            .into(),
                        );
                    }
                    Some(Expression::Symbol("macro")) if stack_entry.output.is_empty() => {
                        let macro_args = match stack_entry.input.head() {
                            Some(Expression::List(args)) => List::clone(args),
                            Some(ex) => {
                                bail!("Syntax error: unexpected token in macro arguments: {}", ex);
                            }
                            None => {
                                bail!("Syntax error: macro does not have arguments token");
                            }
                        };
                        let macro_body = List::clone(stack_entry.input.tail());

                        stack_entry.input = List::new();
                        stack_entry.output = stack_entry.output.push(Expression::Symbol("quote"));
                        stack_entry.output = stack_entry.output.push(
                            Value::Macro {
                                args: Box::from(macro_args),
                                body: Box::from(macro_body),
                            }
                            .into(),
                        );
                    }
                    Some(Expression::Symbol("def")) if stack_entry.output.is_empty() => {
                        stack_entry.output = stack_entry.output.push(Expression::Symbol("def"));
                    }
                    Some(Expression::Symbol(name))
                        if matches!(stack_entry.output.head(), Some(Expression::Symbol("def"))) =>
                    {
                        stack_entry.output = stack_entry.output.push(Expression::Symbol(name));
                    }
                    Some(Expression::Symbol(name)) => {
                        stack_entry.output = stack_entry.output.push(
                            stack_entry
                                .env
                                .get(name)
                                .unwrap_or_else(move || Expression::Symbol(name)),
                        );
                    }
                    Some(Expression::List(list)) => {
                        let new_env = stack_entry.env.clone();
                        stack = stack.push_top(stack_entry).push_top(StackEntry {
                            input: *list,
                            output: List::new(),
                            env: new_env,
                        });
                        continue;
                    }
                    Some(Expression::Value(value)) => {
                        stack_entry.output = stack_entry.output.push(value.into());
                    }
                    None => {
                        let result = match take(&mut stack_entry.output) {
                            List::Normal {
                                car: callable,
                                cdr: args,
                            } => apply_callable(&callable, &args, &mut stack_entry.env)?,
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

                stack = stack.push_top(stack_entry);
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
    let result = match callable {
        // Math expressions
        Expression::Symbol("+") => Sum::apply(args, env)?,
        Expression::Symbol("*") => Multiply::apply(args, env)?,
        Expression::Symbol("-") => Subtract::apply(args, env)?,
        Expression::Symbol("/") => Divide::apply(args, env)?,
        Expression::Symbol("list") => List::clone(&args).into(),

        // Special forms
        Expression::Symbol("quote") => match args.head() {
            Some(expression) => expression.clone(),
            None => Value::Nil.into(),
        },
        Expression::Symbol("def") => {
            let mut items = List::clone(args);

            while !items.is_empty() {
                let name = match items.pop_mut() {
                    Some(Expression::Symbol(name)) => name,
                    _ => bail!("Syntax error: unexpected token at variable name position"),
                };
                let value = match items.pop_mut() {
                    Some(value) => value,
                    None => bail!("Syntax error: variable should have value"),
                };
                env.set(name, value);
            }

            Value::Nil.into()
        }

        // exception
        other => bail!("Expression is not callable type: {} ({})", other, args),
    };

    Ok(result)
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
