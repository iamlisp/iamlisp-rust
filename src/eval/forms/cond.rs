use crate::eval::eval::{iamlisp_eval_expression, CallStack, StackEntry};
use crate::eval::types::{Expression, Value};
use crate::{begin_symbol, cond_symbol, list};
use anyhow::bail;

pub(crate) fn iamlisp_is_cond_expression(stack_entry: &StackEntry) -> bool {
    let input_is_cond = matches!(stack_entry.input.head(), Some(cond_symbol!()));
    let output_is_cond = matches!(stack_entry.output.head(), Some(cond_symbol!()));

    input_is_cond || output_is_cond
}

pub(crate) fn iamlisp_eval_cond_expression(
    mut stack_entry: StackEntry,
    stack: &mut CallStack,
) -> anyhow::Result<()> {
    let output_vec = stack_entry.output.iter().collect::<Vec<_>>();

    match output_vec.as_slice() {
        &[] => match stack_entry.input.shift() {
            Some(cond_symbol!()) => {
                stack_entry.output.push(cond_symbol!());

                stack.push_top(stack_entry);
            }
            _ => {
                bail!(
                    "Unexpected variable definition input state: {}",
                    stack_entry.input
                );
            }
        },
        &[cond_symbol!()] if stack_entry.input.len() == 1 => {
            let default_expr = stack_entry
                .input
                .shift()
                .unwrap_or_else(|| Value::Nil.into());

            stack.push_top(StackEntry {
                env: stack_entry.env.clone(),
                input: list![begin_symbol!(), default_expr],
                output: list![],
            });
        }
        &[cond_symbol!()] => match stack_entry.input.shift() {
            Some(test_expr) => {
                iamlisp_eval_expression(&test_expr, stack_entry, stack)?;
            }
            _ => {
                bail!("Test expression is expected in cond construct");
            }
        },
        &[cond_symbol!(), Expression::Value(Value::Bool(false))] => {
            let _ = stack_entry.input.shift();
            let _ = stack_entry.output.pop();

            stack.push_top(stack_entry);
        }
        &[cond_symbol!(), Expression::Value(Value::Bool(true))] => {
            let true_expr = stack_entry
                .input
                .shift()
                .unwrap_or_else(|| Value::Nil.into());

            stack.push_top(StackEntry {
                env: stack_entry.env.clone(),
                input: list![begin_symbol!(), true_expr],
                output: list![],
            });
        }
        _ => bail!(
            "Unexpected variable definition output state: {}",
            stack_entry.output
        ),
    }

    Ok(())
}
