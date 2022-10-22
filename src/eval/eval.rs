use crate::data::List;
use crate::eval::basic_ops::{Divide, Multiply, Op, Subtract, Sum};
use crate::eval::types::{Env, Expression, Value};
use crate::list;
use anyhow::bail;
use std::mem::take;
use std::ops::Deref;

/*
    Lambda call explanation:

     1.   []                                ((lambda (a) (+ a 1)) 10)

     2.   []                                (10)
          []                                (lambda (a) (+ a 1))

     3.   []                                (10)
          [quote (lambda (a) (+ a 1))]      ()

     4.   [(lambda (a) (+ a 1))]            (10)

     5.   [(lambda (a) (+ a 1)) 10]         ()

     6.   []                                (begin (+ 10 1))

     7.   [begin]                           ()
          []                                (+ 10 1)

    ...

     9.   [begin]                           ()
          [+ 10 1]                          ()


    10.   [begin 11]                           ()

    11.   result = 11
*/

/*

 []         (def a 10 b (+ a 20))

 [def]      (a 10 b (+ a 20))

 [def a]    (10 b (+ a 20))

 [def a 10] (b (+ a 20))

 [def b]    ((+ a 20))

 [def b]    ()
 []         (+ a 20)

 [def b]    ()
 [+]        (a 20)

 [def b]    ()
 [+ a]      (20)

 [def b]    ()
 [+ a 20]   ()

 [def b 30] ()

*/

struct StackEntry {
    input: List<Expression>,
    output: List<Expression>,
    env: Env,
}

type CallStack = List<StackEntry>;

fn iamlisp_is_variables_definition(stack_entry: &StackEntry) -> bool {
    let input_is_def = matches!(stack_entry.input.head(), Some(Expression::Symbol("def")));
    let output_is_def = matches!(stack_entry.output.head(), Some(Expression::Symbol("def")));

    input_is_def || output_is_def
}

fn iamlisp_eval_variables_definition(
    stack_entry: &mut StackEntry,
    stack: &mut CallStack,
) -> anyhow::Result<()> {
    let output_vec = stack_entry.output.iter().collect::<Vec<_>>();

    match output_vec.as_slice() {
        &[] => match stack_entry.input.pop() {
            Some(Expression::Symbol("def")) => {
                stack_entry.output.push(Expression::Symbol("def"));
            }
            _ => {
                bail!(
                    "Unexpected variable definition input state: {}",
                    stack_entry.input
                );
            }
        },
        &[Expression::Symbol("def"), Expression::Symbol(name), value] => {
            stack_entry.env.set(name, value.clone());
            stack_entry.output = list![Expression::Symbol("def")];
        }
        _ => bail!(
            "Unexpected variable definition output state: {}",
            stack_entry.output
        ),
    }

    match (stack_entry.input.pop(), stack_entry.input.pop()) {
        (Some(Expression::Symbol(name)), Some(expr)) => {
            stack_entry.output.push(Expression::Symbol(name));

            iamlisp_eval_expression(&expr, stack_entry, stack)?;
        }
        _ => (),
    }

    Ok(())
}

fn iamlisp_eval_expression(
    expression: &Expression,
    current_stack_entry: &mut StackEntry,
    stack: &mut CallStack,
) -> anyhow::Result<()> {
    match expression {
        Expression::List(list) => {
            stack.push_top(StackEntry {
                env: current_stack_entry.env.clone(),
                input: current_stack_entry.input.clone(),
                output: current_stack_entry.output.clone(),
            });

            stack.push_top(StackEntry {
                env: current_stack_entry.env.clone(),
                input: *list.clone(),
                output: list![],
            });
        }
        Expression::Value(value) => {
            current_stack_entry.output.push(value.clone().into());

            stack.push_top(StackEntry {
                env: current_stack_entry.env.clone(),
                input: current_stack_entry.input.clone(),
                output: current_stack_entry.output.clone(),
            });
        }
        Expression::Symbol(name) => {
            current_stack_entry.output.push(
                current_stack_entry
                    .env
                    .get(name)
                    .unwrap_or_else(move || Expression::Symbol(name)),
            );

            stack.push_top(StackEntry {
                env: current_stack_entry.env.clone(),
                input: current_stack_entry.input.clone(),
                output: current_stack_entry.output.clone(),
            });
        }
    }

    Ok(())
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
        match stack.pop() {
            Some(mut stack_entry) => {
                if iamlisp_is_variables_definition(&mut stack_entry) {
                    iamlisp_eval_variables_definition(&mut stack_entry, &mut stack)?;
                    continue;
                }

                match stack_entry.input.pop() {
                    Some(Expression::Symbol("lambda")) if stack_entry.output.is_empty() => {
                        let lambda_args = match stack_entry.input.head() {
                            Some(Expression::List(args)) => List::clone(&args),
                            Some(ex) => {
                                bail!("Syntax error: unexpected token in lambda arguments: {}", ex);
                            }
                            None => {
                                bail!("Syntax error: lambda does not have arguments token");
                            }
                        };
                        let lambda_body = List::clone(stack_entry.input.tail());

                        stack_entry.input = List::new();
                        stack_entry.output.push(Expression::Symbol("quote"));
                        stack_entry.output.push(
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
                            Some(Expression::List(args)) => List::clone(&args),
                            Some(ex) => {
                                bail!("Syntax error: unexpected token in macro arguments: {}", ex);
                            }
                            None => {
                                bail!("Syntax error: macro does not have arguments token");
                            }
                        };
                        let macro_body = List::clone(stack_entry.input.tail());

                        stack_entry.input = List::new();
                        stack_entry.output.push(Expression::Symbol("quote"));
                        stack_entry.output.push(
                            Value::Macro {
                                args: Box::from(macro_args),
                                body: Box::from(macro_body),
                            }
                            .into(),
                        );
                    }
                    Some(Expression::Symbol("quote")) if stack_entry.output.is_empty() => {
                        stack_entry.output.push(Expression::Symbol("quote"));
                        let quoted_expression = match stack_entry.input.pop() {
                            Some(expression) => expression,
                            None => Value::Nil.into(),
                        };
                        // Ignore other than first argument
                        stack_entry.input = List::new();
                        stack_entry.output.push(quoted_expression);
                    }

                    Some(Expression::Symbol(name)) => {
                        stack_entry.output.push(
                            stack_entry
                                .env
                                .get(name)
                                .unwrap_or_else(move || Expression::Symbol(name)),
                        );
                    }
                    Some(Expression::List(list)) => {
                        let new_env = stack_entry.env.clone();
                        stack.push_top(stack_entry).push_top(StackEntry {
                            input: *list,
                            output: List::new(),
                            env: new_env,
                        });
                        continue;
                    }
                    Some(Expression::Value(value)) => {
                        stack_entry.output.push(value.into());
                    }
                    None => {
                        let result = match take(&mut stack_entry.output) {
                            List::Normal {
                                car: callable,
                                cdr: args_values,
                            } => {
                                let env = &stack_entry.env;
                                match callable {
                                    // Math expressions
                                    Expression::Symbol("+") => Sum::apply(&args_values, env)?,
                                    Expression::Symbol("*") => Multiply::apply(&args_values, env)?,
                                    Expression::Symbol("-") => Subtract::apply(&args_values, env)?,
                                    Expression::Symbol("/") => Divide::apply(&args_values, env)?,
                                    Expression::Symbol("list") => List::clone(&args_values).into(),

                                    // Special forms
                                    Expression::Symbol("quote") => match args_values.head() {
                                        Some(expression) => expression.clone(),
                                        None => Value::Nil.into(),
                                    },
                                    Expression::Symbol("begin") => {
                                        match List::clone(&args_values).reverse().head() {
                                            Some(expression) => expression.clone(),
                                            None => Value::Nil.into(),
                                        }
                                    }
                                    Expression::Symbol("def") => {
                                        let mut items = List::clone(&args_values);

                                        while !items.is_empty() {
                                            let name = match items.pop() {
                                                Some(Expression::Symbol(name)) => name,
                                                _ => bail!("Syntax error: unexpected token at variable name position"),
                                            };
                                            let value = match items.pop() {
                                                Some(value) => value,
                                                None => bail!(
                                                    "Syntax error: variable should have value"
                                                ),
                                            };
                                            stack_entry.env.set(name, value);
                                        }

                                        Value::Nil.into()
                                    }

                                    Expression::Value(Value::Lambda {
                                        args: args_names,
                                        body,
                                        env,
                                    }) => {
                                        let new_env = env.child();

                                        let mut values = List::clone(&args_values);

                                        let mut names_iter = List::clone(&args_names).into_iter();
                                        while let Some(name_expression) = names_iter.next() {
                                            let name = match name_expression {
                                                Expression::Symbol(name) => name,
                                                exp => bail!(
                                                    "Syntax error: unexpected argument name: {}",
                                                    exp
                                                ),
                                            };
                                            let value = match values.pop() {
                                                Some(value) => value,
                                                None => bail!("Runtime error: lambda expects {} arguments, but called with {}", args_names.len(), args_values.len())
                                            };
                                            new_env.set(name, value);
                                        }

                                        stack.push_top(StackEntry {
                                            input: List::cons(Expression::Symbol("begin"), *body),
                                            output: List::new(),
                                            env: new_env,
                                        });

                                        continue;
                                    }
                                    Expression::Value(Value::Macro { args, body }) => {
                                        todo!()
                                    }

                                    // exception
                                    other => bail!(
                                        "Expression is not callable type: {} (args: {})",
                                        other,
                                        args_values
                                    ),
                                }
                            }
                            List::Empty => list![].into(),
                        };

                        if let Some(StackEntry {
                            output: prev_output,
                            ..
                        }) = stack.head_mut()
                        {
                            prev_output.push(result);
                        } else {
                            last_return_value = result;
                        }
                        continue;
                    }
                }

                stack.push_top(stack_entry);
            }
            None => return Ok(last_return_value),
        }
    }
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
    fn test_lambda_call() {
        let env = Env::new();
        let lambda: List<_> = list![
            Expression::Symbol("lambda"),
            list![Expression::Symbol("a")].into(),
            list![
                Expression::Symbol("+"),
                Expression::Symbol("a"),
                Expression::Symbol("a")
            ]
            .into()
        ];
        let expression = list![lambda.into(), Value::Int64(10).into()];

        let result = eval_iterative(expression, env.clone()).unwrap();

        assert_eq!(Expression::Value(Value::Int64(20)), result);
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
        let env = Env::new();
        let expression: List<_> = list![
            Expression::Symbol("def"),
            Expression::Symbol("a"),
            list![
                Expression::Symbol("+"),
                Value::Int64(1).into(),
                Value::Int64(2).into()
            ]
            .into(),
            Expression::Symbol("b"),
            list![
                Expression::Symbol("*"),
                Expression::Symbol("a"),
                Value::Int64(2).into()
            ]
            .into()
        ];

        let result = eval_iterative(expression, env.clone()).unwrap();

        assert_eq!(Expression::Value(Value::Nil), result);

        assert_eq!(Expression::Value(Value::Int64(3)), env.get("a").unwrap());
        assert_eq!(Expression::Value(Value::Int64(6)), env.get("b").unwrap());
    }
}
