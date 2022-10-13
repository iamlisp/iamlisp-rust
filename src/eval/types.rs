use std::collections::HashMap;
use std::fmt::{format, Display, Formatter};

#[derive(Clone)]
pub(crate) enum List {
    Empty,
    Normal { car: Expression, cdr: Box<List> },
}

pub(crate) const EMPTY_LIST: List = List::Empty;

impl List {
    pub(crate) fn cons(car: Expression, cdr: List) -> Self {
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

impl Display for List {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut str = String::new();
        str.push_str("(");
        str.push_str(
            &*self
                .clone()
                .into_iter()
                .map(|i| format!("{}", i))
                .collect::<Vec<_>>()
                .join(" "),
        );
        str.push_str(")");

        write!(f, "{}", str)
    }
}

impl Into<List> for Vec<Expression> {
    fn into(self) -> List {
        let mut list = EMPTY_LIST;

        for item in self.into_iter().rev() {
            list = List::cons(item, list)
        }

        list
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
pub(crate) enum Value {
    Int64(i64),
    Float64(f64),
    String(String),
}

#[derive(Clone)]
pub(crate) enum Expression {
    Value(Value),
    List(Box<List>),
    Nil,
}

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Expression::Nil => "Nil".to_string(),
            Expression::List(l) => format!("{}", l),
            Expression::Value(Value::Int64(i)) => format!("{}", i),
            Expression::Value(Value::Float64(f)) => format!("{}", f),
            Expression::Value(Value::String(s)) => format!("{}", s),
        };

        write!(f, "{}", str)
    }
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
        let lst: List = vec![Expression::Value(Value::Int64(0))].into();
        let mut iter = lst.into_iter();

        assert!(matches!(
            iter.next(),
            Some(Expression::Value(Value::Int64(0)))
        ));
        assert!(matches!(iter.next(), None));
    }

    #[test]
    fn two_items_iter() {
        let lst: List = vec![Expression::Nil, Expression::Nil].into();
        let mut iter = lst.into_iter();

        assert!(matches!(iter.next(), Some(Expression::Nil)));
        assert!(matches!(iter.next(), Some(Expression::Nil)));
        assert!(matches!(iter.next(), None));
    }

    #[test]
    fn list_to_string() {
        assert_eq!("()", format!("{}", EMPTY_LIST));

        {
            let list: List = vec![
                Expression::Value(Value::Int64(0)),
                Expression::Value(Value::Int64(1)),
            ]
            .into();
            assert_eq!("(0 1)", format!("{}", list));
        }
    }
}
