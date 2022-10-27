use crate::data::List;
use crate::eval::types::Expression;
use crate::read::parser::Parser;
use crate::read::tokenize::tokenize;

mod parser;
mod tokenize;

pub(crate) fn parse(program: &str) -> anyhow::Result<List<Expression>> {
    let tokens = tokenize(program.chars()).unwrap();

    Parser::new(tokens).parse()
}

#[cfg(test)]
mod tests {
    use super::parse;
    use crate::eval::types::Value;
    use crate::{list, symbol};

    #[test]
    fn read_nested_lists() {
        let program = r#"(() ()) ()"#;

        assert_eq!(
            list![list![list![].into(), list![].into()].into(), list![].into()],
            parse(program).unwrap()
        );
    }

    #[test]
    fn read_boolean() {
        assert_eq!(list![Value::Bool(true).into()], parse("true").unwrap());
        assert_eq!(list![Value::Bool(false).into()], parse("false").unwrap());
    }

    #[test]
    fn read_int() {
        assert_eq!(list![Value::Int64(0).into()], parse("0").unwrap());
        assert_eq!(list![Value::Int64(10).into()], parse("10").unwrap());
        assert_eq!(list![Value::Int64(-10).into()], parse("-10").unwrap());
    }

    #[test]
    fn read_float() {
        assert_eq!(list![Value::Float64(0.0).into()], parse("0.0").unwrap());
        assert_eq!(list![Value::Float64(10.0).into()], parse("10.0").unwrap());
        assert_eq!(list![Value::Float64(-10.0).into()], parse("-10.0").unwrap());
    }

    #[test]
    fn read_string() {
        assert_eq!(
            list![Value::String("".to_string()).into()],
            parse(r#""""#).unwrap()
        );

        assert_eq!(
            list![Value::String("hello world".to_string()).into()],
            parse(r#""hello world""#).unwrap()
        );
    }

    #[test]
    fn read_symbol() {
        assert_eq!(
            list![symbol!("foo"), symbol!("bar")],
            parse("foo bar").unwrap()
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
            parse(program).unwrap()
        );
    }
}
