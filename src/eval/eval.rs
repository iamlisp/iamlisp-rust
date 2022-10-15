use crate::data::List;
use crate::eval::types::{Env, Expression, Value};
use crate::list;
use anyhow::bail;

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

    while let Some(StackEntry {
        mut input,
        mut output,
        env,
    }) = stack.pop_mut()
    {
        match input.pop_mut() {
            Some(exp) => {
                match exp {
                    Expression::List(list) => match *list {
                        List::Empty => {
                            output = output.push(Expression::List(Box::new(List::new())));
                        }
                        // @todo check special form
                        _ => (),
                    },
                    Expression::Value(value) => {
                        output = output.push(Expression::Value(value));
                    }
                    Expression::Symbol(name) => {
                        output = output.push(env.get(name));
                    }
                }
                stack = stack.unshift(StackEntry { input, output, env });
            }
            None => {
                // @todo evaluate output

                let result = match output {
                    List::Empty => Expression::List(Box::new(List::Empty)),
                    _list => Expression::Value(Value::Nil),
                };

                if let Some(StackEntry {
                    output: mut prev_output,
                    ..
                }) = stack.pop_mut()
                {
                    prev_output = prev_output.unshift(result);
                } else {
                    last_return_value = result;
                }
            }
        }
    }

    Ok(last_return_value)
}

// pub(crate) fn eval_list_iterative(list: List<Expression>, env: Env) -> anyhow::Result<Expression> {
//     let mut stack = List::new();
//     let mut result = None;
//
//     stack = stack.push(StackEntry {
//         input: list,
//         output: List::new(),
//         env: env.clone(),
//     });
//
//     loop {
//         let StackEntry {
//             input: mut last_input,
//             output: mut last_output,
//             env: last_env,
//         } = match stack.last_mut() {
//             Some(entry) => entry.clone(),
//             None => return Ok(result),
//         };
//
//         match last_input.take_mut() {
//             Some(car) => {
//                 match car {
//                     Expression::Value(value) => {
//                         last_output.push_mut(Expression::Value(value.clone()))
//                     }
//                     Expression::List(list) => stack.push(StackEntry {
//                         input: *list.clone(),
//                         output: List::new(),
//                         env: env.clone(),
//                     }),
//                     Expression::Symbol(name) => {
//                         //let value = env.get(name);
//                         last_output.push_mut(Expression::Symbol(name));
//                     }
//                 }
//             }
//             None => {
//                 result = match last_output.car() {
//                     Some(callable) => {
//                         let args = last_output.cdr().clone().into_iter().collect::<Vec<_>>();
//                         let env = &last_env;
//
//                         apply_fn(callable, &args, env)?
//                     }
//                     None => Expression::List(Box::new(EMPTY_LIST)),
//                 };
//
//                 let _ = stack.pop();
//
//                 if let Some(entry) = stack.last_mut() {
//                     entry.output.push_mut(result.clone())
//                 }
//             }
//         }
//     }
// }
//
// fn apply_fn(callable: &Expression, args: &[Expression], _env: &Env) -> anyhow::Result<Expression> {
//     match callable {
//         Expression::Symbol("+") => match args {
//             [Expression::Value(Value::Int64(a)), Expression::Value(Value::Int64(b))] => {
//                 Ok(Expression::Value(Value::Int64(a + b)))
//             }
//             _ => bail!("Unsupported arguments"),
//         },
//         _ => bail!("Unsupported callable"),
//     }
// }
//
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
