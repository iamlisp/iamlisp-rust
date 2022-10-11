#[derive(Clone)]
pub(crate) struct List {
    car: Expression,
    cdr: Option<List>,
}

impl List {
    pub(crate) fn new(car: Expression, cdr: Option<List>) -> Self {
        List { car, cdr }
    }

    pub(crate) fn car(&self) -> &Expression {
        &self.car
    }

    pub(crate) fn cdr(&self) -> &Option<List> {
        &self.cdr
    }
}

#[derive(Clone)]
pub(crate) enum Expression {
    Int(i64),
    List(List),
    Nil,
}

#[derive(Clone)]
pub(crate) struct Env {}
