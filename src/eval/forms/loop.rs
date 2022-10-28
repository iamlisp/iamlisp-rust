use crate::data::List;
use crate::eval::eval::{iamlisp_eval_next_input_expression, CallStack, StackEntry};
use crate::eval::types::{Expression, Value};
use crate::{begin_symbol, def_symbol, list, loop_symbol};
use anyhow::bail;

pub(crate) fn iamlisp_is_loop_expression(stack_entry: &StackEntry) -> bool {
    let input_is_loop = matches!(stack_entry.input.head(), Some(loop_symbol!()));
    let output_is_loop = matches!(stack_entry.output.head(), Some(loop_symbol!()));

    input_is_loop || output_is_loop
}

pub(crate) fn iamlisp_eval_loop_expression(
    mut stack_entry: StackEntry,
    stack: &mut CallStack,
    return_value: &mut Expression,
) -> anyhow::Result<()> {
    let output_vec = stack_entry.output.iter().collect::<Vec<_>>();

    match output_vec.as_slice() {
        &[] => match stack_entry.input.shift() {
            Some(loop_symbol!()) => {
                stack_entry.output.push(loop_symbol!());
                stack_entry.env = stack_entry.env.child();

                // stack_entry.env.set("recur", todo!());

                stack.push_top(stack_entry);
            }
            _ => {
                bail!(
                    "Unexpected loop definition input state: {}",
                    stack_entry.input
                );
            }
        },
        &[loop_symbol!()] => match stack_entry.input.shift() {
            Some(Expression::List(mut args_list)) => {
                let mut body = stack_entry.input.clone();
                body.push_top(begin_symbol!());
                stack.push_top(StackEntry {
                    input: body,
                    output: list![],
                    env: stack_entry.env.clone(),
                });

                args_list.push_top(def_symbol!());
                stack.push_top(StackEntry {
                    input: *args_list,
                    output: list![],
                    env: stack_entry.env.clone(),
                });
            }

            _ => {
                bail!("Parameters should be of type list");
            }
        },

        &[loop_symbol!(), Expression::List(args_names), Expression::List(args_values)] => {
            bail!("{}: {}", args_names, args_values)
        }

        _ => bail!(
            "Unexpected variable definition output state: {}",
            stack_entry.output
        ),
    }

    Ok(())
}
