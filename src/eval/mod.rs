use crate::eval::env::Env;
use crate::eval::native_calls::load_native_calls;

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
