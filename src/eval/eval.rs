use crate::data::List;
use crate::eval::basic_ops::{Divide, Multiply, Op, Subtract, Sum};
use crate::eval::types::{Env, Expression, Value};
use crate::list;
use anyhow::{anyhow, bail};
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

struct StackEntry {
    input: List<Expression>,
    output: List<Expression>,
    env: Env,
}

type CallStack = List<StackEntry>;

/*

 Variables definition steps:

 []         (def a 10 b (+ a 20))           {}

 [def]      (a 10 b (+ a 20))               {}

 [def a]    (10 b (+ a 20))                 {}

 [def a 10] (b (+ a 20))                    {}

 [def b]    ((+ a 20))                      {a: 10}

 []         (+ a 20)                        {a: 10}
 [def b]    ()

 [+]        (a 20)                          {a: 10}
 [def b]    ()

 [+ a]      (20)                            {a: 10}
 [def b]    ()

 [+ a 20]   ()                              {a: 10}
 [def b]    ()

 [def b 30] ()                              {a: 10}

 []         ()                              {a: 10, b: 30}

*/

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

fn iamlisp_is_lambda_definition(stack_entry: &StackEntry) -> bool {
    matches!(stack_entry.input.head(), Some(Expression::Symbol("lambda")))
}

fn iamlisp_eval_lambda_definition(
    stack_entry: &mut StackEntry,
    stack: &mut CallStack,
    return_value: &mut Expression,
) -> anyhow::Result<()> {
    match stack_entry.input.pop() {
        Some(Expression::Symbol("lambda")) => (),
        _ => {
            bail!("Invalid lambda expression");
        }
    };

    let lambda_args = match stack_entry.input.pop() {
        Some(Expression::List(lambda_args)) => lambda_args,
        _ => {
            bail!("Invalid lambda arguments");
        }
    };

    let lambda = Value::Lambda {
        env: stack_entry.env.clone(),
        args: lambda_args,
        body: Box::from(stack_entry.input.clone()),
    }
    .into();

    iamlisp_return_result(lambda, stack, return_value)?;

    Ok(())
}

fn iamlisp_is_macro_definition(stack_entry: &StackEntry) -> bool {
    matches!(stack_entry.input.head(), Some(Expression::Symbol("macro")))
}

fn iamlisp_eval_macro_definition(
    stack_entry: &mut StackEntry,
    stack: &mut CallStack,
    return_value: &mut Expression,
) -> anyhow::Result<()> {
    match stack_entry.input.pop() {
        Some(Expression::Symbol("macro")) => (),
        _ => {
            bail!("Invalid macro expression");
        }
    };

    let lambda_args = match stack_entry.input.pop() {
        Some(Expression::List(lambda_args)) => lambda_args,
        _ => {
            bail!("Invalid macro arguments");
        }
    };

    let r#macro = Value::Macro {
        args: lambda_args,
        body: Box::from(stack_entry.input.clone()),
    }
    .into();

    iamlisp_return_result(r#macro, stack, return_value)?;

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

fn iamlisp_call_function(
    func: &Expression,
    args_values: &List<Expression>,
    current_stack_entry: &mut StackEntry,
    call_stack: &mut CallStack,
    return_value: &mut Expression,
) -> anyhow::Result<()> {
    let env = &current_stack_entry.env;

    let result = match func {
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
        Expression::Symbol("begin") => match args_values.iter().last() {
            Some(expression) => expression.clone(),
            None => Value::Nil.into(),
        },

        Expression::Value(Value::Lambda {
            args: args_names,
            env,
            body,
        }) => {
            let env = env.child();
            let mut values = List::clone(&args_values);
            let mut body = List::clone(&body);

            for arg_name in args_names.clone().into_iter() {
                match arg_name {
                    Expression::Symbol(name) => {
                        let value = match values.pop() {
                            Some(value) => value,
                            None => {
                                bail!(
                                    "Lambda expects {} arguments but were provided - {}",
                                    args_names.len(),
                                    args_values.len()
                                )
                            }
                        };

                        env.set(name, value);
                    }
                    _ => {
                        bail!("Unexpected symbol in lambda arguments");
                    }
                }
            }

            body.push_top(Expression::Symbol("begin"));

            call_stack.push_top(StackEntry {
                env,
                input: body,
                output: list![],
            });

            return Ok(());
        }
        ex => {
            bail!(
                "Expression is not callable type: {} (args: {})",
                ex,
                args_values
            );
        }
    };

    iamlisp_return_result(result, call_stack, return_value)
}

fn iamlisp_return_result(
    result: Expression,
    call_stack: &mut CallStack,
    return_value: &mut Expression,
) -> anyhow::Result<()> {
    if let Some(StackEntry { output, .. }) = call_stack.head_mut() {
        output.push(result);
    } else {
        *return_value = result;
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

                if iamlisp_is_lambda_definition(&mut stack_entry) {
                    iamlisp_eval_lambda_definition(
                        &mut stack_entry,
                        &mut stack,
                        &mut last_return_value,
                    )?;
                    continue;
                }

                if iamlisp_is_macro_definition(&mut stack_entry) {
                    iamlisp_eval_macro_definition(
                        &mut stack_entry,
                        &mut stack,
                        &mut last_return_value,
                    )?;
                    continue;
                }

                if stack_entry.input.is_empty() {
                    match stack_entry.output.head().cloned() {
                        Some(callable) => {
                            iamlisp_call_function(
                                &callable,
                                &stack_entry.output.tail().clone(),
                                &mut stack_entry,
                                &mut stack,
                                &mut last_return_value,
                            )?;
                        }
                        None => {
                            iamlisp_return_result(
                                list![].into(),
                                &mut stack,
                                &mut last_return_value,
                            )?;
                        }
                    }

                    continue;
                }

                match stack_entry.input.pop() {
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
                    None => (),
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
                args: Box::new(list![Expression::Symbol("a")]),
                body: Box::new(list![list![
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
