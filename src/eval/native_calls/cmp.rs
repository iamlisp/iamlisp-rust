use crate::data::List;
use crate::eval::env::Env;
use crate::eval::native_calls::Op;
use crate::eval::types::{Expression, Value};

fn bin_cmp<CMP>(cmp_fn: CMP, args: &List<Expression>) -> anyhow::Result<bool>
where
    CMP: Fn(&Expression, &Expression) -> anyhow::Result<bool>,
{
    if args.len() < 2 {
        anyhow::bail!("Too few arguments given: {}", args);
    }

    let mut args_iter = args.iter().peekable();

    while let Some(current_operand) = args_iter.next() {
        match args_iter.peek() {
            Some(next_operand) => {
                if !cmp_fn(current_operand, next_operand)? {
                    return Ok(false);
                }
            }
            None => (),
        }
    }

    Ok(true)
}

#[derive(Clone, PartialEq)]
pub(crate) struct Eq;

impl Op for Eq {
    fn name(&self) -> &'static str {
        "="
    }

    fn apply(&self, args: &List<Expression>, _env: &Env) -> anyhow::Result<Expression> {
        Ok(Value::Bool(bin_cmp(
            |a, b| match (a, b) {
                (Expression::Value(Value::Int64(a)), Expression::Value(Value::Int64(b))) => {
                    Ok(a == b)
                }
                (Expression::Value(Value::Float64(a)), Expression::Value(Value::Float64(b))) => {
                    Ok(a == b)
                }
                (Expression::Value(Value::String(a)), Expression::Value(Value::String(b))) => {
                    Ok(a == b)
                }
                (Expression::Value(Value::Bool(a)), Expression::Value(Value::Bool(b))) => {
                    Ok(a == b)
                }
                _ => unimplemented!(),
            },
            args,
        )?)
        .into())
    }
}

#[derive(Clone, PartialEq)]
pub(crate) struct Ne;

impl Op for Ne {
    fn name(&self) -> &'static str {
        "!="
    }

    fn apply(&self, args: &List<Expression>, _env: &Env) -> anyhow::Result<Expression> {
        Ok(Value::Bool(bin_cmp(
            |a, b| match (a, b) {
                (Expression::Value(Value::Int64(a)), Expression::Value(Value::Int64(b))) => {
                    Ok(a != b)
                }
                (Expression::Value(Value::Float64(a)), Expression::Value(Value::Float64(b))) => {
                    Ok(a != b)
                }
                (Expression::Value(Value::String(a)), Expression::Value(Value::String(b))) => {
                    Ok(a != b)
                }
                (Expression::Value(Value::Bool(a)), Expression::Value(Value::Bool(b))) => {
                    Ok(a != b)
                }
                _ => unimplemented!(),
            },
            args,
        )?)
        .into())
    }
}

#[derive(Clone, PartialEq)]
pub(crate) struct Gt;

impl Op for Gt {
    fn name(&self) -> &'static str {
        ">"
    }

    fn apply(&self, args: &List<Expression>, _env: &Env) -> anyhow::Result<Expression> {
        Ok(Value::Bool(bin_cmp(
            |a, b| match (a, b) {
                (Expression::Value(Value::Int64(a)), Expression::Value(Value::Int64(b))) => {
                    Ok(a > b)
                }
                (Expression::Value(Value::Float64(a)), Expression::Value(Value::Float64(b))) => {
                    Ok(a > b)
                }
                _ => unimplemented!(),
            },
            args,
        )?)
        .into())
    }
}

#[derive(Clone, PartialEq)]
pub(crate) struct Lt;

impl Op for Lt {
    fn name(&self) -> &'static str {
        "<"
    }

    fn apply(&self, args: &List<Expression>, _env: &Env) -> anyhow::Result<Expression> {
        Ok(Value::Bool(bin_cmp(
            |a, b| match (a, b) {
                (Expression::Value(Value::Int64(a)), Expression::Value(Value::Int64(b))) => {
                    Ok(a < b)
                }
                (Expression::Value(Value::Float64(a)), Expression::Value(Value::Float64(b))) => {
                    Ok(a < b)
                }
                _ => unimplemented!(),
            },
            args,
        )?)
        .into())
    }
}

#[derive(Clone, PartialEq)]
pub(crate) struct Ge;

impl Op for Ge {
    fn name(&self) -> &'static str {
        ">="
    }

    fn apply(&self, args: &List<Expression>, _env: &Env) -> anyhow::Result<Expression> {
        Ok(Value::Bool(bin_cmp(
            |a, b| match (a, b) {
                (Expression::Value(Value::Int64(a)), Expression::Value(Value::Int64(b))) => {
                    Ok(a >= b)
                }
                (Expression::Value(Value::Float64(a)), Expression::Value(Value::Float64(b))) => {
                    Ok(a >= b)
                }
                _ => unimplemented!(),
            },
            args,
        )?)
        .into())
    }
}

#[derive(Clone, PartialEq)]
pub(crate) struct Le;

impl Op for Le {
    fn name(&self) -> &'static str {
        "<="
    }

    fn apply(&self, args: &List<Expression>, _env: &Env) -> anyhow::Result<Expression> {
        Ok(Value::Bool(bin_cmp(
            |a, b| match (a, b) {
                (Expression::Value(Value::Int64(a)), Expression::Value(Value::Int64(b))) => {
                    Ok(a <= b)
                }
                (Expression::Value(Value::Float64(a)), Expression::Value(Value::Float64(b))) => {
                    Ok(a <= b)
                }
                _ => unimplemented!(),
            },
            args,
        )?)
        .into())
    }
}
