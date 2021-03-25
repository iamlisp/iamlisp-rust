use std::collections::HashSet;

lazy_static! {
    static ref DELIMITER_CHARS: HashSet<char> = vec![' ', '\t', '\n', '\r'].into_iter().collect();
    static ref NON_SYMBOL_CHARS: HashSet<char> =
        vec!['(', ')', '{', '}', '[', ']', '"', '\'', '^', '\'', '#', ';']
            .into_iter()
            .collect();
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Symbol(Vec<char>),
    String(Vec<char>),
    Integer(isize),
    Float(f32),
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
    InsideSymbol,
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
                        reader.goto_next_char();
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
                    }
                    None => {
                        return Ok(tokens);
                    }
                }
            }
            TokenizerState::InsideString => {
                let mut chars = vec![];
                let mut escape = false;

                loop {
                    match reader.current_char() {
                        Some('"') if escape == true => {
                            chars.push('"');
                            escape = false;
                            reader.goto_next_char();
                        }
                        Some('"') => {
                            tokens.push(Token::String(chars));
                            tokenizer_state = TokenizerState::Outside;
                            reader.goto_next_char();
                            break;
                        }
                        Some('\\') => {
                            escape = true;
                            reader.goto_next_char();
                        }
                        Some(c) => {
                            chars.push(c.clone());
                            reader.goto_next_char();
                        }
                        None => return Err("Unexpected end of input on reading string".to_owned()),
                    }
                }
            }
            TokenizerState::InsideSymbol => {
                let mut chars = vec![];

                loop {
                    match reader.current_char() {
                        Some(c) if DELIMITER_CHARS.contains(c) || NON_SYMBOL_CHARS.contains(c) => {
                            break;
                        }
                        Some(c) => {
                            chars.push(c.clone());
                            reader.goto_next_char();
                        }
                        None => {
                            break;
                        }
                    }
                }

                // TODO Interpret numbers, booleans, etc...
                if chars.len() == 1 && chars[0] == '.' {
                    tokens.push(Token::Dot)
                } else {
                    tokens.push(Token::Symbol(chars));
                }

                tokenizer_state = TokenizerState::Outside;
            }
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
        assert_eq!(
            Ok(vec![Token::LeftParen, Token::RightParen]),
            tokenize(&mut reader)
        );
    }

    #[test]
    fn read_string() {
        let program: Vec<char> = "\"hello world\"".chars().collect();
        let mut reader = Reader::new(&program);
        assert_eq!(
            Ok(vec![Token::String("hello world".chars().collect())]),
            tokenize(&mut reader)
        );
    }

    #[test]
    fn read_escaped_string() {
        let program: Vec<char> = "\"program \\\"lisp\\\"\"".chars().collect();
        let mut reader = Reader::new(&program);
        assert_eq!(
            Ok(vec![Token::String("program \"lisp\"".chars().collect())]),
            tokenize(&mut reader)
        );
    }

    #[test]
    fn read_symbol() {
        let program: Vec<char> = "foo bar baz 123 11.22 true false".chars().collect();
        let mut reader = Reader::new(&program);

        assert_eq!(
            Ok(vec![
                Token::Symbol("foo".chars().collect()),
                Token::Symbol("bar".chars().collect()),
                Token::Symbol("baz".chars().collect()),
                Token::Symbol("123".chars().collect()),
                Token::Symbol("11.22".chars().collect()),
                Token::Symbol("true".chars().collect()),
                Token::Symbol("false".chars().collect()),
            ]),
            tokenize(&mut reader)
        );
    }

    #[test]
    fn read_whole_program() {
        let program: Vec<char> = "(+ (foo 1 \"hello\") 12.5)".chars().collect();
        let mut reader = Reader::new(&program);

        assert_eq!(
            Ok(vec![
                Token::LeftParen,
                Token::Symbol("+".chars().collect()),
                Token::LeftParen,
                Token::Symbol("foo".chars().collect()),
                Token::Symbol("1".chars().collect()),
                Token::String("hello".chars().collect()),
                Token::RightParen,
                Token::Symbol("12.5".chars().collect()),
                Token::RightParen
            ]),
            tokenize(&mut reader)
        );
    }
}
