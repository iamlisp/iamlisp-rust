use crate::data::List;
use crate::eval::types::Expression;
use crate::read::compiler::Compiler;
use crate::read::tokenize::tokenize;

mod compiler;
mod tokenize;

pub(crate) fn compile(program: &str) -> anyhow::Result<List<Expression>> {
    let tokens = tokenize(program.chars()).unwrap();

    Compiler::new(tokens).compile()
}

#[cfg(test)]
mod tests {
    use super::compile;
    use crate::eval::types::Value;
    use crate::{list, symbol};

    #[test]
    fn read_nested_lists() {
        let program = r#"(() ()) ()"#;

        assert_eq!(
            list![list![list![].into(), list![].into()].into(), list![].into()],
            compile(program).unwrap()
        );
    }

    #[test]
    fn read_boolean() {
        assert_eq!(list![Value::Bool(true).into()], compile("true").unwrap());
        assert_eq!(list![Value::Bool(false).into()], compile("false").unwrap());
    }

    #[test]
    fn read_int() {
        assert_eq!(list![Value::Int64(0).into()], compile("0").unwrap());
        assert_eq!(list![Value::Int64(10).into()], compile("10").unwrap());
        assert_eq!(list![Value::Int64(-10).into()], compile("-10").unwrap());
    }

    #[test]
    fn read_float() {
        assert_eq!(list![Value::Float64(0.0).into()], compile("0.0").unwrap());
        assert_eq!(list![Value::Float64(10.0).into()], compile("10.0").unwrap());
        assert_eq!(
            list![Value::Float64(-10.0).into()],
            compile("-10.0").unwrap()
        );
    }

    #[test]
    fn read_string() {
        assert_eq!(
            list![Value::String("".to_string()).into()],
            compile(r#""""#).unwrap()
        );

        assert_eq!(
            list![Value::String("hello world".to_string()).into()],
            compile(r#""hello world""#).unwrap()
        );
    }

    #[test]
    fn read_symbol() {
        assert_eq!(
            list![symbol!("foo"), symbol!("bar")],
            compile("foo bar").unwrap()
        );
    }

    #[test]
    fn read_whole_program() {
        let program = r#"(+ (foo 1 "hello") 12.5)"#;

        assert_eq!(
            list![list![
                symbol!("+"),
                list![
                    symbol!("foo"),
                    Value::Int64(1).into(),
                    Value::String("hello".to_string()).into()
                ]
                .into(),
                Value::Float64(12.5).into()
            ]
            .into()],
            compile(program).unwrap()
        );
    }
}
