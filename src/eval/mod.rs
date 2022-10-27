use crate::data::List;
use crate::eval::env::Env;
use crate::eval::eval::iamlisp_eval;
use crate::eval::native_calls::load_native_calls;
use crate::eval::types::Expression;

pub(crate) mod env;
pub(crate) mod eval;
pub(crate) mod forms;
pub(crate) mod native_calls;
pub(crate) mod symbols;
pub(crate) mod types;

pub(crate) fn create_env() -> Env {
    let mut env = Env::new();

    load_native_calls(&mut env);

    env
}

pub(crate) fn eval(expressions: &List<Expression>, env: &Env) -> anyhow::Result<Expression> {
    let mut last_result = Expression::default();

    for expr in expressions.iter() {
        last_result = iamlisp_eval(expr, env)?;
    }

    Ok(last_result)
}
