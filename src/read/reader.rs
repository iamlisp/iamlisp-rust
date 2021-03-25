use crate::eval::types::Expression;
use crate::read::tokenizer::Token;

pub fn read(tokens: Vec<Token>) -> Vec<Expression> {
    let mut expressions = vec![];

    while let Some(token) = tokens.iter().next() {
        match token {}
    }

    expressions
}
