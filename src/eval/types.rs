use std::collections::HashMap;

#[derive(Clone)]
pub(crate) enum List {
    Empty,
    Normal { car: Expression, cdr: Box<List> },
}

pub(crate) const EMPTY_LIST: List = List::Empty;

impl List {
    pub(crate) fn new(car: Expression, cdr: List) -> Self {
        List::Normal {
            car,
            cdr: Box::new(cdr),
        }
    }

    pub(crate) fn car(&self) -> Option<&Expression> {
        match self {
            List::Empty => None,
            List::Normal { car, cdr: _ } => Some(car),
        }
    }

    pub(crate) fn cdr(&self) -> &List {
        match self {
            List::Empty => &EMPTY_LIST,
            List::Normal { car: _, cdr } => cdr,
        }
    }

    pub(crate) fn is_empty(&self) -> bool {
        matches!(self, List::Empty)
    }
}

impl Iterator for List {
    type Item = Expression;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.car().cloned();

        *self = self.cdr().clone();

        current
    }
}

#[derive(Clone)]
pub(crate) enum Expression {
    Int(i64),
    List(Box<List>),
    Nil,
}

#[derive(Clone)]
pub(crate) struct Env {
    values: HashMap<&'static str, Expression>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_iter() {
        let vec = List::Empty.into_iter().collect::<Vec<_>>();

        assert_eq!(vec.len(), 0);
    }

    #[test]
    fn one_item_iter() {
        let vec = List::new(Expression::Nil, List::Empty)
            .into_iter()
            .collect::<Vec<_>>();

        assert_eq!(vec.len(), 1);
    }

    #[test]
    fn two_items_iter() {
        let vec = List::new(Expression::Nil, List::new(Expression::Nil, List::Empty))
            .into_iter()
            .collect::<Vec<_>>();

        assert_eq!(vec.len(), 2);
    }
}
