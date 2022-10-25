use crate::data::List;
use crate::eval::native_calls::Op;
use crate::eval::types::{Env, Expression, Value};
use anyhow::bail;

#[derive(Clone, PartialEq)]
pub(crate) struct Multiply;

impl Op for Multiply {
    fn name(&self) -> &'static str {
        "*"
    }

    fn apply(&self, args: &List<Expression>, _env: &Env) -> anyhow::Result<Expression> {
        Ok(match args.head() {
            Some(Expression::Value(Value::Int64(_))) => Value::Int64(
                List::clone(&args)
                    .into_iter()
                    .try_fold(1, |acc, exp| match exp {
                        Expression::Value(Value::Int64(val)) => Ok(acc * val),
                        Expression::Value(Value::Float64(val)) => Ok(acc * (val as i64)),
                        x => bail!("Unable to multiply {} by {}", acc, x),
                    })?,
            )
            .into(),
            Some(Expression::Value(Value::Float64(_))) => Value::Float64(
                List::clone(&args)
                    .into_iter()
                    .try_fold(1.0, |acc, exp| match exp {
                        Expression::Value(Value::Int64(val)) => Ok(acc * (val as f64)),
                        Expression::Value(Value::Float64(val)) => Ok(acc * val),
                        x => bail!("Unable to multiply {} by {}", acc, x),
                    })?,
            )
            .into(),
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
    fn test_multiply_two_numbers() {
        let env = Env::new();

        // i64 * i64
        {
            let args = list![Value::Int64(2).into(), Value::Int64(3).into()];

            assert_eq!(
                Expression::Value(Value::Int64(6)),
                Multiply.apply(&args, &env).unwrap()
            );
        };

        // f64 * f64
        {
            let args = list![Value::Float64(2.5).into(), Value::Float64(3.2).into()];

            assert_eq!(
                Expression::Value(Value::Float64(8.0)),
                Multiply.apply(&args, &env).unwrap()
            );
        }

        // f64 * i64
        {
            let args = list![Value::Float64(2.5).into(), Value::Int64(3).into()];

            assert_eq!(
                Expression::Value(Value::Float64(7.5)),
                Multiply.apply(&args, &env).unwrap()
            );
        };

        // i64 * f64
        {
            let args = list![Value::Int64(2).into(), Value::Float64(3.2).into()];

            assert_eq!(
                Expression::Value(Value::Int64(6)),
                Multiply.apply(&args, &env).unwrap()
            );
        }
    }
}
