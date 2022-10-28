use crate::data::List;
use crate::eval::env::Env;
use crate::eval::native_calls::Op;
use crate::eval::types::{Expression, Value};
use anyhow::bail;

#[derive(Clone, PartialEq)]
pub(crate) struct Pow;

impl Op for Pow {
    fn name(&self) -> &'static str {
        "%"
    }

    fn apply(&self, args: &List<Expression>, _env: &Env) -> anyhow::Result<Expression> {
        Ok(match (args.head(), args.tail().head()) {
            (
                Some(Expression::Value(Value::Int64(base))),
                Some(Expression::Value(Value::Int64(power))),
            ) => Value::Int64(base.pow(*power as u32)).into(),

            (
                Some(Expression::Value(Value::Float64(base))),
                Some(Expression::Value(Value::Float64(power))),
            ) => Value::Float64(base.powf(*power)).into(),

            _ => bail!(
                "Function not implemented for this kind of arguments: {}",
                args
            ),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::list;

    #[test]
    fn test_pow() {
        let env = Env::new();

        // i64 ^ i64
        {
            let args = list![Value::Int64(2).into(), Value::Int64(3).into()];

            assert_eq!(
                Expression::Value(Value::Int64(8)),
                Pow.apply(&args, &env).unwrap()
            );
        };

        // f64 ^ f64
        {
            let args = list![Value::Float64(2.0).into(), Value::Float64(3.0).into()];

            assert_eq!(
                Expression::Value(Value::Float64(8.0)),
                Pow.apply(&args, &env).unwrap()
            );
        }
    }
}
