use crate::data::List;
use crate::eval::env::Env;
use crate::eval::native_calls::Op;
use crate::eval::types::{Expression, Value};
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

#[derive(Clone, PartialEq)]
pub(crate) struct Divide;

impl Op for Divide {
    fn name(&self) -> &'static str {
        "/"
    }

    fn apply(&self, args: &List<Expression>, _env: &Env) -> anyhow::Result<Expression> {
        Ok(match args.head() {
            Some(Expression::Value(Value::Int64(init))) => {
                Value::Int64(args.tail().iter().try_fold(*init, |acc, exp| match exp {
                    Expression::Value(Value::Int64(val)) => Ok(acc / *val),
                    Expression::Value(Value::Float64(val)) => Ok(acc / (*val as i64)),
                    x => bail!("Unable to divide {} by {}", acc, x),
                })?)
                .into()
            }

            Some(Expression::Value(Value::Float64(init))) => {
                Value::Float64(args.tail().iter().try_fold(*init, |acc, exp| match exp {
                    Expression::Value(Value::Int64(val)) => Ok(acc / (*val as f64)),
                    Expression::Value(Value::Float64(val)) => Ok(acc / *val),
                    x => bail!("Unable to divide {} by {}", acc, x),
                })?)
                .into()
            }

            _ => bail!(
                "Function not implemented for this kind of arguments: {}",
                args
            ),
        })
    }
}

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

#[derive(Clone, PartialEq)]
pub(crate) struct Subtract;

impl Op for Subtract {
    fn name(&self) -> &'static str {
        "-"
    }

    fn apply(&self, args: &List<Expression>, _env: &Env) -> anyhow::Result<Expression> {
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

#[derive(Clone, PartialEq)]
pub(crate) struct Sum;

impl Op for Sum {
    fn name(&self) -> &'static str {
        "+"
    }

    fn apply(&self, args: &List<Expression>, _env: &Env) -> anyhow::Result<Expression> {
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
