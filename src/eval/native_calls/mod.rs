use crate::data::List;
use crate::eval::env::Env;
use crate::eval::native_calls::begin::Begin;
use crate::eval::native_calls::list_constructor::ListConstructor;
use crate::eval::native_calls::pow::Pow;
use crate::eval::types::{Expression, NativeCall};
use anyhow::Result;
use divide::Divide;
use multiply::Multiply;
use std::sync::Arc;
use subtract::Subtract;
use sum::Sum;

mod begin;
mod divide;
mod list_constructor;
mod multiply;
mod pow;
mod subtract;
mod sum;

pub(crate) trait Op {
    fn name(&self) -> &'static str;
    fn apply(&self, args: &List<Expression>, env: &Env) -> Result<Expression>;
}

pub(crate) fn load_native_calls(env: &mut Env) {
    env.set("+", NativeCall(Arc::new(Box::from(Sum))).into());
    env.set("-", NativeCall(Arc::new(Box::from(Subtract))).into());
    env.set("/", NativeCall(Arc::new(Box::from(Divide))).into());
    env.set("*", NativeCall(Arc::new(Box::from(Multiply))).into());
    env.set("pow", NativeCall(Arc::new(Box::from(Pow))).into());
    env.set("begin", NativeCall(Arc::new(Box::from(Begin))).into());
    env.set(
        "list",
        NativeCall(Arc::new(Box::from(ListConstructor))).into(),
    );
}
