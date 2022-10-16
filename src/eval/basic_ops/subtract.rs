use crate::data::List;
use crate::eval::basic_ops::Op;
use crate::eval::types::{Env, Expression, Value};
use anyhow::bail;

pub(crate) struct Subtract {}

impl Op for Subtract {
    fn apply(args: &List<Expression>, _env: &Env) -> anyhow::Result<Expression> {
        Ok(match args.head() {
            Some(Expression::Value(Value::Int64(init))) if args.tail().is_empty() => {
                Value::Int64(-*init).into()
            }
            Some(Expression::Value(Value::Float64(init))) if args.tail().is_empty() => {
                Value::Float64(-*init).into()
            }
            Some(Expression::Value(Value::Int64(init))) => Value::Int64(
                List::clone(args.tail())
                    .into_iter()
                    .try_fold(*init, |acc, exp| match exp {
                        Expression::Value(Value::Int64(val)) => Ok(acc - val),
                        Expression::Value(Value::Float64(val)) => Ok(acc - (val as i64)),
                        x => bail!("Unable to subtract {} and {}", acc, x),
                    })?,
            )
            .into(),
            Some(Expression::Value(Value::Float64(init))) => Value::Float64(
                List::clone(args.tail())
                    .into_iter()
                    .try_fold(*init, |acc, exp| match exp {
                        Expression::Value(Value::Int64(val)) => Ok(acc - (val as f64)),
                        Expression::Value(Value::Float64(val)) => Ok(acc - val),
                        x => bail!("Unable to subtract {} and {}", acc, x),
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
    fn test_negate_number() {
        let env = Env::new();

        assert_eq!(
            Expression::Value(Value::Int64(-10)),
            Subtract::apply(&list![Value::Int64(10).into()], &env).unwrap()
        );

        assert_eq!(
            Expression::Value(Value::Float64(-10.0)),
            Subtract::apply(&list![Value::Float64(10.0).into()], &env).unwrap()
        );
    }

    #[test]
    fn test_subtract_two_numbers() {
        let env = Env::new();

        // i64 - i64
        {
            let args = list![Value::Int64(2).into(), Value::Int64(3).into()];

            assert_eq!(
                Expression::Value(Value::Int64(-1)),
                Subtract::apply(&args, &env).unwrap()
            );
        };

        // f64 - f64
        {
            let args = list![Value::Float64(2.5).into(), Value::Float64(3.0).into()];

            assert_eq!(
                Expression::Value(Value::Float64(-0.5)),
                Subtract::apply(&args, &env).unwrap()
            );
        }

        // f64 - i64
        {
            let args = list![Value::Float64(2.5).into(), Value::Int64(3).into()];

            assert_eq!(
                Expression::Value(Value::Float64(-0.5)),
                Subtract::apply(&args, &env).unwrap()
            );
        };

        // i64 - f64
        {
            let args = list![Value::Int64(2).into(), Value::Float64(3.2).into()];

            assert_eq!(
                Expression::Value(Value::Int64(-1)),
                Subtract::apply(&args, &env).unwrap()
            );
        }
    }
}
