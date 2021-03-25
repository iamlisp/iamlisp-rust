#[derive(Debug, PartialEq)]
pub enum Expression {
    Symbol(Vec<char>),
    String(Vec<char>),
    Integer(isize),
    Float(f32),
    Boolean(bool),
    List(List),
    // Vector(List),
    // Map(List),
    Nil,
}

pub struct List {
    pub car: Expression,
    pub cdr: Option<List>,
}

pub const NIL: Expression = Expression::Nil;

impl List {
    pub fn new(car: Expression, cdr: Option<List>) -> Self {
        List { car, cdr }
    }
}
