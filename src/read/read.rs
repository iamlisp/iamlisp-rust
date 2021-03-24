use std::ops::Deref;

pub enum Token {
    Symbol(Vec<char>),
    String(Vec<char>),
    LeftParen,
    RightParen,
    Dot,
    ThreeDot
}

pub struct Reader<'a> {
    program: &'a Vec<char>,
    cursor: usize,
}

impl<'a> Reader<'a> {
    pub fn new(program: &'a Vec<char>) -> Self {
        Reader { program, cursor: 0 }
    }

    pub fn current(&self) -> Option<&'a char> {
        self.program.get(self.cursor)
    }

    pub fn is_eof(&self) -> bool {
        self.cursor >= self.program.len()
    }

    pub fn next(&mut self) {
        self.cursor += 1
    }
}

enum TokenizerState {
    Initial,
    String,
    Symbol,
    Dot
}

fn tokenize(reader: &mut Reader) -> Result<Vec<Token>, String> {
    let mut tokens = vec![];
    let mut tokenizer_state = TokenizerState::Initial;

    loop {
        match tokenizer_state {
            TokenizerState::Initial => {
                match reader.current() {
                    Some('"') => {
                        tokenizer_state = TokenizerState::String;
                    }
                    Some(' ') | Some('\t') | Some('\n') | Some('\r') => {
                        reader.next();
                    }
                    Some('(') => {
                        tokens.push(Token::LeftParen);
                    }
                    Some(')') => {
                        tokens.push(Token::RightParen)
                    }
                    Some('.') => {
                        tokenizer_state = TokenizerState::Dot;
                    }
                    Some(_) => {
                        tokenizer_state = TokenizerState::Symbol;
                    },
                    None => {
                        return Ok(tokens);
                    }
                }
            },
            TokenizerState::String => {
                let mut chars = vec![];
                let mut escape = false;

                loop {
                    match reader.current() {
                        Some('"') if escape == true => {
                            chars.push('"');
                            escape = false;
                            reader.next();
                        },
                        Some('"') => {
                            tokens.push(Token::String(chars));
                            reader.next();
                            break;
                        },
                        Some('/') => {
                            escape = true;
                            reader.next();
                        },
                        Some(c) => {
                            chars.push(c.clone());
                            reader.next();
                            break;
                        },
                        None => {
                            return Err("Unexpected end of input on reading string".to_owned())
                        },
                    }
                }
            },
            TokenizerState::Symbol => {
                unimplemented!()
            },
            TokenizerState::Dot => {
                let mut total_dots = 0;

                loop {
                    match reader.current() {
                        Some('.') => {
                            total_dots += 1;
                            reader.next();
                        },
                        _ => {
                            if total_dots == 1 {
                                tokens.push(Token::Dot);
                                tokenizer_state = TokenizerState::Initial;
                                break;
                            } else if total_dots == 3 {
                                tokens.push(Token::ThreeDot);
                                tokenizer_state = TokenizerState::Initial;
                                break;
                            } else {
                                return Err("Unexpected token".to_owned())
                            }
                        }
                    }
                }
            },
        }
    }
}
