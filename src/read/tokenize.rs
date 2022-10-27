#[derive(Debug, PartialEq)]
pub enum Token {
    Symbol(String),
    String(String),
    Int64(i64),
    Float64(f64),
    Boolean(bool),
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftSquareBracket,
    RightSquareBracket,
    Caret,
    SingleQuote,
    Sharp,
    Dot,
}

enum TokenizerState {
    Outside,
    InsideString,
    InsideSymbol,
    InsideComment,
}

pub(crate) fn tokenize<I>(mut source_iter: I) -> Result<Vec<Token>, String>
where
    I: Iterator<Item = char>,
{
    let spaces = " \t\n\r".to_owned();
    let non_symbols = r#"(){}[]"'^#;"#.to_owned();

    let mut tokens = vec![];
    let mut tokenizer_state = TokenizerState::Outside;

    let mut buffered_char: Option<char> = None;

    loop {
        match tokenizer_state {
            TokenizerState::Outside => {
                match buffered_char.take().or_else(|| source_iter.next()) {
                    Some('"') => {
                        tokenizer_state = TokenizerState::InsideString;
                    }
                    Some(c) if spaces.contains(c) => {
                        // Skip delimiters
                    }
                    Some('(') => {
                        tokens.push(Token::LeftParen);
                    }
                    Some(')') => {
                        tokens.push(Token::RightParen);
                    }
                    Some('{') => {
                        tokens.push(Token::LeftBracket);
                    }
                    Some('}') => {
                        tokens.push(Token::RightBracket);
                    }
                    Some('[') => {
                        tokens.push(Token::LeftSquareBracket);
                    }
                    Some(']') => {
                        tokens.push(Token::RightSquareBracket);
                    }
                    Some('^') => {
                        tokens.push(Token::Caret);
                    }
                    Some('#') => {
                        tokens.push(Token::Sharp);
                    }
                    Some('\'') => {
                        tokens.push(Token::SingleQuote);
                    }
                    Some(';') => {
                        tokenizer_state = TokenizerState::InsideComment;
                    }
                    Some(c) => {
                        buffered_char.replace(c);
                        tokenizer_state = TokenizerState::InsideSymbol;
                    }
                    None => {
                        return Ok(tokens);
                    }
                }
            }
            TokenizerState::InsideString => {
                let mut buff = String::new();
                let mut escape = false;

                loop {
                    match source_iter.next() {
                        Some('"') if escape == true => {
                            buff.push('"');
                            escape = false;
                        }
                        Some('"') => {
                            break;
                        }
                        Some('\\') => {
                            escape = true;
                        }
                        Some(c) => {
                            buff.push(c);
                        }
                        None => return Err("Unexpected end of input on reading string".to_owned()),
                    }
                }

                tokens.push(Token::String(buff));

                tokenizer_state = TokenizerState::Outside;
            }
            TokenizerState::InsideSymbol => {
                let mut buff = String::new();

                loop {
                    match buffered_char.take().or_else(|| source_iter.next()) {
                        Some(c) if spaces.contains(c) || non_symbols.contains(c) => {
                            buffered_char.replace(c);
                            break;
                        }
                        Some(c) => {
                            buff.push(c);
                        }
                        None => {
                            break;
                        }
                    }
                }

                if buff == "." {
                    tokens.push(Token::Dot)
                } else if let Ok(bool) = buff.parse::<bool>() {
                    tokens.push(Token::Boolean(bool));
                } else if let Ok(integer) = buff.parse::<i64>() {
                    tokens.push(Token::Int64(integer));
                } else if let Ok(float) = buff.parse::<f64>() {
                    tokens.push(Token::Float64(float));
                } else {
                    tokens.push(Token::Symbol(buff));
                }

                tokenizer_state = TokenizerState::Outside;
            }
            TokenizerState::InsideComment => {
                loop {
                    match source_iter.next() {
                        Some('\n') | None => {
                            break;
                        }
                        Some(_) => {
                            // Skip comment
                        }
                    }
                }
                tokenizer_state = TokenizerState::Outside;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{tokenize, Token};

    #[test]
    fn read_whole_program() {
        let program = r#"(+ (foo . "hello") 12.5 44 true false)"#;

        assert_eq!(
            Ok(vec![
                Token::LeftParen,
                Token::Symbol("+".to_owned()),
                Token::LeftParen,
                Token::Symbol("foo".to_owned()),
                Token::Dot,
                Token::String("hello".to_owned()),
                Token::RightParen,
                Token::Float64(12.5),
                Token::Int64(44),
                Token::Boolean(true),
                Token::Boolean(false),
                Token::RightParen
            ]),
            tokenize(program.chars())
        );
    }
}
