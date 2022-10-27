use crate::data::List;
use crate::eval::types::{Env, Expression};
use anyhow::Result;

pub(crate) use divide::Divide;
pub(crate) use multiply::Multiply;
pub(crate) use subtract::Subtract;
pub(crate) use sum::Sum;

mod divide;
mod multiply;
mod subtract;
mod sum;

pub(crate) trait Op {
    fn name(&self) -> &'static str;
    fn apply(&self, args: &List<Expression>, env: &Env) -> Result<Expression>;
}
