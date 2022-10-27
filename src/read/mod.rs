use crate::data::List;
use crate::eval::types::Expression;
use crate::read::compiler::Compiler;
use crate::read::tokenize::tokenize;

mod compiler;
mod tokenize;

pub(crate) fn compile(program: &str) -> anyhow::Result<List<Expression>> {
    let tokens = tokenize(program.chars()).unwrap();

    let mut compiler = Compiler::new(tokens);

    compiler.compile()
}

#[cfg(test)]
mod tests {
    use super::compile;
    use crate::eval::types::Value;
    use crate::{list, symbol};

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
