use crate::data::List;
use crate::eval::types::{Expression, Value};
use crate::list;
use crate::read::tokenize::Token;
use anyhow::bail;

pub(crate) struct Parser {
    program_iter: Box<dyn Iterator<Item = Token>>,
}

impl Parser {
    pub(crate) fn new(program_tokens: Vec<Token>) -> Self {
        Self {
            program_iter: Box::new(program_tokens.into_iter()),
        }
    }

    pub(crate) fn parse(mut self) -> anyhow::Result<List<Expression>> {
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
