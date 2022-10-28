use crate::data::List;
use crate::eval::env::Env;
use crate::eval::native_calls::Op;
use crate::eval::types::{Expression, Value};
use anyhow::bail;

#[derive(Clone, PartialEq)]
pub(crate) struct ListConstructor;

impl Op for ListConstructor {
    fn name(&self) -> &'static str {
        "list"
    }

    fn apply(&self, args: &List<Expression>, _env: &Env) -> anyhow::Result<Expression> {
        Ok(args.clone().into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::list;

    #[test]
    fn empty_list_construct() {
        let env = Env::new();

        assert_eq!(
            Expression::List(Box::from(list![])),
            ListConstructor.apply(&list![], &env).unwrap()
        );
    }

    #[test]
    fn list_construct() {
        let env = Env::new();

        assert_eq!(
            Expression::List(Box::from(list![
                Value::Int64(10).into(),
                Value::Float64(10.0).into()
            ])),
            ListConstructor
                .apply(
                    &list![Value::Int64(10).into(), Value::Float64(10.0).into()],
                    &env
                )
                .unwrap()
        );
    }
}
