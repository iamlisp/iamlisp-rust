pub enum Token {
    Symbol(&'static Vec<char>),
    String(&'static Vec<char>),
    Punctuator(&'static char),
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

fn tokenize<'a>(reader: &'a Reader) -> Vec<&'a Token> {
    Vec::<&'a Token>::default()
}
