use crate::data::List;
use crate::eval::env::Env;
use crate::eval::native_calls::Op;
use crate::eval::types::{Expression, Value};
use anyhow::bail;

#[derive(Clone, PartialEq)]
pub(crate) struct Divide;

impl Op for Divide {
    fn name(&self) -> &'static str {
        "/"
    }

    fn apply(&self, args: &List<Expression>, _env: &Env) -> anyhow::Result<Expression> {
        Ok(match args.head() {
            Some(Expression::Value(Value::Int64(init))) => Value::Int64(
                List::clone(args.tail())
                    .into_iter()
                    .try_fold(*init, |acc, exp| match exp {
                        Expression::Value(Value::Int64(val)) => Ok(acc / val),
                        Expression::Value(Value::Float64(val)) => Ok(acc / (val as i64)),
                        x => bail!("Unable to divide {} by {}", acc, x),
                    })?,
            )
            .into(),
            Some(Expression::Value(Value::Float64(init))) => Value::Float64(
                List::clone(args.tail())
                    .into_iter()
                    .try_fold(*init, |acc, exp| match exp {
                        Expression::Value(Value::Int64(val)) => Ok(acc / (val as f64)),
                        Expression::Value(Value::Float64(val)) => Ok(acc / val),
                        x => bail!("Unable to divide {} by {}", acc, x),
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
    fn test_divide_two_numbers() {
        let env = Env::new();

        // i64 / i64
        {
            let args = list![Value::Int64(10).into(), Value::Int64(2).into()];

            assert_eq!(
                Expression::Value(Value::Int64(5)),
                Divide.apply(&args, &env).unwrap()
            );
        };

        // f64 / f64
        {
            let args = list![Value::Float64(1.0).into(), Value::Float64(0.5).into()];

            assert_eq!(
                Expression::Value(Value::Float64(2.0)),
                Divide.apply(&args, &env).unwrap()
            );
        }

        // f64 / i64
        {
            let args = list![Value::Float64(5.0).into(), Value::Int64(2).into()];

            assert_eq!(
                Expression::Value(Value::Float64(2.5)),
                Divide.apply(&args, &env).unwrap()
            );
        };

        // i64 / f64
        {
            let args = list![Value::Int64(10).into(), Value::Float64(2.0).into()];

            assert_eq!(
                Expression::Value(Value::Int64(5)),
                Divide.apply(&args, &env).unwrap()
            );
        }
    }
}
