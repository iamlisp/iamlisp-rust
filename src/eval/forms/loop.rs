use crate::data::List;
use crate::eval::eval::{iamlisp_eval_next_input_expression, CallStack, StackEntry};
use crate::eval::types::{Expression, Value};
use crate::{begin_symbol, list, loop_symbol};
use anyhow::bail;

fn is_even(n: usize) -> bool {
    n % 2 == 0
}

pub(crate) fn iamlisp_is_loop_expression(stack_entry: &StackEntry) -> bool {
    let input_is_loop = matches!(stack_entry.input.head(), Some(loop_symbol!()));
    let output_is_loop = matches!(stack_entry.output.head(), Some(loop_symbol!()));

    input_is_loop || output_is_loop
}

pub(crate) fn iamlisp_eval_loop_expression(
    mut stack_entry: StackEntry,
    stack: &mut CallStack,
) -> anyhow::Result<()> {
    let output_vec = stack_entry.output.iter().collect::<Vec<_>>();

    match output_vec.as_slice() {
        &[] => match stack_entry.input.shift() {
            Some(loop_symbol!()) => {
                stack_entry.output.push(loop_symbol!());
                stack_entry.env = stack_entry.env.child();

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
            Some(Expression::List(args_list)) => {
                let mut args_names = list![];
                let mut args_values = list![];

                for arg in args_list.iter().enumerate() {
                    match arg {
                        (i, Expression::Symbol(name)) if is_even(i) => {
                            args_names.push(name.clone());
                        }

                        (i, e) if is_even(i) => {
                            bail!(
                                "Unexpected expression while reading loop init definition: {}",
                                e
                            );
                        }

                        (_, expression) => {
                            args_values.push(expression.clone());
                        }
                    }
                }

                todo!();
            }
            _ => {
                bail!("Initial definition expression is expected in loop construct");
            }
        },
        _ => bail!(
            "Unexpected variable definition output state: {}",
            stack_entry.output
        ),
    }

    Ok(())
}
