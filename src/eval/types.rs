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

impl IntoIterator for List {
    type Item = Expression;
    type IntoIter = ListIter;

    fn into_iter(self) -> Self::IntoIter {
        ListIter { list: self }
    }
}

pub(crate) struct ListIter {
    list: List,
}

impl Iterator for ListIter {
    type Item = Expression;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.list.car().cloned();

        self.list = self.list.cdr().clone();

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
        let mut iter = List::Empty.into_iter();

        assert!(matches!(iter.next(), None));
    }

    #[test]
    fn one_item_iter() {
        let mut iter = List::new(Expression::Nil, List::Empty).into_iter();

        assert!(matches!(iter.next(), Some(Expression::Nil)));
        assert!(matches!(iter.next(), None));
    }

    #[test]
    fn two_items_iter() {
        let mut iter =
            List::new(Expression::Nil, List::new(Expression::Nil, List::Empty)).into_iter();

        assert!(matches!(iter.next(), Some(Expression::Nil)));
        assert!(matches!(iter.next(), Some(Expression::Nil)));
        assert!(matches!(iter.next(), None));
    }
}
