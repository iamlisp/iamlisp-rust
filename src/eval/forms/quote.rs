use crate::eval::eval::{iamlisp_pass_value_to_next_stack_entry, CallStack, StackEntry};
use crate::eval::types::Expression;
use crate::quote_symbol;
use anyhow::bail;

pub(crate) fn iamlisp_is_quote_expression(stack_entry: &StackEntry) -> bool {
    matches!(stack_entry.input.head(), Some(quote_symbol!()))
}

pub(crate) fn iamlisp_eval_quote_expression(
    mut stack_entry: StackEntry,
    stack: &mut CallStack,
    return_value: &mut Expression,
) -> anyhow::Result<()> {
    let arg = match stack_entry.input.tail_mut().shift() {
        Some(arg) => arg,
        None => bail!("Too few parameters for special operator: QUOTE"),
    };

    iamlisp_pass_value_to_next_stack_entry(arg, stack, return_value)
}
