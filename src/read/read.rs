pub enum Token {
    Symbol(Vec<char>),
    String(Vec<char>),
    Punctuator(char),
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

fn tokenize(reader: &Reader) -> Vec<Token> {
    Vec::<Token>::default()
}
