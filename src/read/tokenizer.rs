use std::collections::HashSet;

lazy_static! {
    static ref DELIMITER_CHARS: HashSet<char> = vec![' ', '\t', '\n', '\r'].into_iter().collect();
    static ref NON_SYMBOL_CHARS: HashSet<char> = vec!['(', ')', '"', '\''].into_iter().collect();
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Symbol(Vec<char>),
    String(Vec<char>),
    LeftParen,
    RightParen,
    Dot
}

pub struct Reader<'a> {
    program: &'a Vec<char>,
    cursor: usize,
}

impl<'a> Reader<'a> {
    pub fn new(program: &'a Vec<char>) -> Self {
        Reader { program, cursor: 0 }
    }

    pub fn current_char(&self) -> Option<&'a char> {
        self.program.get(self.cursor)
    }

    pub fn goto_next_char(&mut self) {
        self.cursor += 1
    }
}

enum TokenizerState {
    Outside,
    InsideString,
    InsideSymbol
}

fn tokenize(reader: &mut Reader) -> Result<Vec<Token>, String> {
    let mut tokens = vec![];
    let mut tokenizer_state = TokenizerState::Outside;

    loop {
        match tokenizer_state {
            TokenizerState::Outside => {
                match reader.current_char() {
                    Some('"') => {
                        tokenizer_state = TokenizerState::InsideString;
                    }
                    Some(c) if DELIMITER_CHARS.contains(c) => {
                        // Skip delimiters
                        reader.goto_next_char();
                    }
                    Some('(') => {
                        tokens.push(Token::LeftParen);
                        reader.goto_next_char();
                    }
                    Some(')') => {
                        tokens.push(Token::RightParen);
                        reader.goto_next_char();
                    }
                    Some(_) => {
                        tokenizer_state = TokenizerState::InsideSymbol;
                    },
                    None => {
                        return Ok(tokens);
                    }
                }
            },
            TokenizerState::InsideString => {
                let mut chars = vec![];
                let mut escape = false;

                loop {
                    match reader.current_char() {
                        Some('"') if escape == true => {
                            chars.push('"');
                            escape = false;
                            reader.goto_next_char();
                        },
                        Some('"') => {
                            tokens.push(Token::String(chars));
                            reader.goto_next_char();
                            break;
                        },
                        Some('/') => {
                            escape = true;
                            reader.goto_next_char();
                        },
                        Some(c) => {
                            chars.push(c.clone());
                            reader.goto_next_char();
                            break;
                        },
                        None => {
                            return Err("Unexpected end of input on reading string".to_owned())
                        },
                    }
                }
            },
            TokenizerState::InsideSymbol => {
                loop {
                    let mut chars = vec![];
                    match reader.current_char() {
                        Some(c) if DELIMITER_CHARS.contains(c) || NON_SYMBOL_CHARS.contains(c) => {
                            if chars.len() == 1 && chars[0] == '.' {
                                tokens.push(Token::Dot)
                            } else {
                                tokens.push(Token::Symbol(chars));
                            }
                            tokenizer_state = TokenizerState::Outside;
                            break;
                        }
                        Some(c) => {
                            chars.push(c.clone());
                            reader.goto_next_char();
                        }
                        None => {
                            tokenizer_state = TokenizerState::Outside;
                            break;
                        }
                    }
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{tokenize, Reader, Token};

    #[test]
    fn read_empty_program() {
        let program: Vec<char> = "".chars().collect();
        let mut reader = Reader::new(&program);
        assert_eq!(Ok(vec![]), tokenize(&mut reader));
    }

    #[test]
    fn read_parens() {
        let program: Vec<char> = "()".chars().collect();
        let mut reader = Reader::new(&program);
        assert_eq!(Ok(vec![Token::LeftParen, Token::RightParen]), tokenize(&mut reader));
    }
}
