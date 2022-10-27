use crate::data::List;
use crate::eval::types::{Expression, Value};
use crate::list;
use crate::read::tokenize::Token;
use anyhow::bail;
use std::cell::{RefCell, RefMut};
use std::slice::SliceIndex;
use std::sync::atomic::{AtomicIsize, Ordering};

struct Compiler {
    program: Vec<Token>,
    cursor: usize,
}

// impl Compiler {
//     pub(crate) fn new(program: Vec<Token>) -> Self {
//         Self { program, cursor: 0 }
//     }
//
//     fn current_token(&self) -> Option<&Token> {
//         self.program.get(self.cursor)
//     }
//
//     fn goto_next_token(&mut self) {
//         self.cursor += 1;
//     }
//
//     pub(crate) fn compile(&mut self) -> anyhow::Result<List<Expression>> {
//         let mut expressions = list![];
//
//         while let Some(token) = self.current_token() {
//             expressions.push(match token {
//                 Token::Symbol(name) => Expression::Symbol(name.into_iter().collect()),
//                 Token::String(text) => Value::String(text.into_iter().collect()).into(),
//                 Token::Integer(int) => Value::Int64(*int).into(),
//                 Token::Float(float) => Value::Float64(*float).into(),
//                 Token::Boolean(b) => Value::Bool(*b).into(),
//                 Token::LeftParen => {
//                     self.goto_next_token();
//                     self.parse_list()?.into()
//                 }
//                 t => bail!("Compile error: unexpected token: {:?}", t),
//             });
//
//             self.goto_next_token();
//         }
//
//         Ok(expressions)
//     }
//
//     fn parse_list(&mut self) -> anyhow::Result<List<Expression>> {
//         let mut expressions = list![];
//
//         while let Some(token) = self.current_token() {
//             expressions.push(match token {
//                 Token::Symbol(name) => Expression::Symbol(name.into_iter().collect()),
//                 Token::String(text) => Value::String(text.into_iter().collect()).into(),
//                 Token::Integer(int) => Value::Int64(*int).into(),
//                 Token::Float(float) => Value::Float64(*float).into(),
//                 Token::Boolean(b) => Value::Bool(*b).into(),
//                 Token::LeftParen => {
//                     self.goto_next_token();
//                     self.parse_list()?.into()
//                 }
//                 Token::RightParen => {
//                     self.goto_next_token();
//                     return Ok(expressions);
//                 }
//                 t => bail!("Compile error: unexpected token: {:?}", t),
//             });
//
//             self.goto_next_token();
//         }
//
//         bail!("Compile error: unexpected end of program while reading list")
//     }
// }
//
// #[cfg(test)]
// mod tests {
//     use super::Compiler;
//     use crate::list;
//     use crate::read::tokenizer::{tokenize, Reader};
//
//     #[test]
//     fn read_whole_program() {
//         let program: Vec<char> = "(+ (foo 1 \"hello\") 12.5)".chars().collect();
//         let mut reader = Reader::new(&program);
//         let tokens = tokenize(&mut reader).unwrap();
//
//         let mut compiler = Compiler::new(tokens);
//
//         assert_eq!(list![], compiler.compile().unwrap());
//     }
// }
