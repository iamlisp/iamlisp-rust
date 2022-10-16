use crate::data::List;
use crate::eval::basic_ops::Op;
use crate::eval::types::{Env, Expression, Value};
use anyhow::bail;

pub(crate) struct Sum {}

impl Op for Sum {
    fn apply(args: &List<Expression>, _env: &Env) -> anyhow::Result<Expression> {
        Ok(match args.head() {
            Some(Expression::Value(Value::Int64(_))) => Value::Int64(
                List::clone(&args)
                    .into_iter()
                    .try_fold(0, |acc, exp| match exp {
                        Expression::Value(Value::Int64(val)) => Ok(acc + val),
                        Expression::Value(Value::Float64(val)) => Ok(acc + (val as i64)),
                        x => bail!("Unable to sum {} and {}", acc, x),
                    })?,
            )
            .into(),
            Some(Expression::Value(Value::Float64(_))) => Value::Float64(
                List::clone(&args)
                    .into_iter()
                    .try_fold(0.0, |acc, exp| match exp {
                        Expression::Value(Value::Int64(val)) => Ok(acc + (val as f64)),
                        Expression::Value(Value::Float64(val)) => Ok(acc + val),
                        x => bail!("Unable to sum {} and {}", acc, x),
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
