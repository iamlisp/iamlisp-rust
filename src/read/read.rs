pub enum Token {
    String(&'static str),
    Punctuator(&'static str),
}

pub struct Reader {
    program: &'static Vec<char>,
    cursor: usize,
}

impl Reader {
    pub fn new(program: &'static Vec<char>) -> Self {
        Reader { program, cursor: 0 }
    }

    pub fn current(&self) -> Option<&'static char> {
        self.program.get(self.cursor)
    }

    pub fn is_eof(&self) -> bool {
        self.cursor >= self.program.len()
    }

    pub fn next(&mut self) {
        self.cursor += 1
    }
}

fn tokenize(reader: Reader) -> Vec<Token> {
    Vec::<Token>::default()
}
