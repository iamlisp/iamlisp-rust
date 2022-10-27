use crate::data::List;
use crate::eval::types::{Expression, Value};
use crate::list;
use crate::read::tokenize::Token;
use anyhow::bail;
use std::cell::{RefCell, RefMut};
use std::slice::SliceIndex;
use std::sync::atomic::{AtomicIsize, Ordering};

struct Compiler {
    program_iter: Box<dyn Iterator<Item = Token>>,
}

impl Compiler {
    pub(crate) fn new(program_tokens: Vec<Token>) -> Self {
        Self {
            program_iter: Box::new(program_tokens.into_iter()),
        }
    }

    pub(crate) fn compile(&mut self) -> anyhow::Result<List<Expression>> {
        let mut expressions = list![];

        while let Some(token) = self.program_iter.next() {
            expressions.push(match token {
                Token::Symbol(name) => Expression::Symbol(Box::leak(name.into_boxed_str())),
                Token::String(text) => Value::String(text).into(),
                Token::Int64(int) => Value::Int64(int).into(),
                Token::Float64(float) => Value::Float64(float).into(),
                Token::Boolean(bool) => Value::Bool(bool).into(),
                Token::LeftParen => self.parse_list()?.into(),
                t => bail!("Compile error: unexpected token: {:?}", t),
            });
        }

        Ok(expressions)
    }

    fn parse_list(&mut self) -> anyhow::Result<List<Expression>> {
        let mut expressions = list![];

        while let Some(token) = self.program_iter.next() {
            expressions.push(match token {
                Token::Symbol(name) => Expression::Symbol(Box::leak(name.into_boxed_str())),
                Token::String(text) => Value::String(text).into(),
                Token::Int64(int) => Value::Int64(int).into(),
                Token::Float64(float) => Value::Float64(float).into(),
                Token::Boolean(bool) => Value::Bool(bool).into(),
                Token::LeftParen => self.parse_list()?.into(),
                Token::RightParen => {
                    return Ok(expressions);
                }
                t => bail!("Compile error: unexpected token: {:?}", t),
            });
        }

        bail!("Compile error: unexpected end of program while reading list")
    }
}

#[cfg(test)]
mod tests {
    use super::Compiler;
    use crate::eval::types::Value;
    use crate::read::tokenize::tokenize;
    use crate::{list, symbol};

    #[test]
    fn read_whole_program() {
        let program = r#"(+ (foo 1 "hello") 12.5)"#;
        let tokens = tokenize(program.chars()).unwrap();

        let mut compiler = Compiler::new(tokens);

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
            compiler.compile().unwrap()
        );
    }
}
