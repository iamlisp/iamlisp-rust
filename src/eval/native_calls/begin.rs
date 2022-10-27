use crate::data::List;
use crate::eval::env::Env;
use crate::eval::native_calls::Op;
use crate::eval::types::{Expression, Value};
use anyhow::bail;

#[derive(Clone, PartialEq)]
pub(crate) struct Begin;

impl Op for Begin {
    fn name(&self) -> &'static str {
        "/"
    }

    fn apply(&self, args: &List<Expression>, _env: &Env) -> anyhow::Result<Expression> {
        Ok(args.iter().last().map(Clone::clone).unwrap_or_default())
    }
}
