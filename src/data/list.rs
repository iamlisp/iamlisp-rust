use std::fmt::{Display, Formatter};
use std::fs::read;
use std::mem::take;
use std::ptr::replace;

#[macro_export]
macro_rules! list {
    () => {{
        $crate::data::List::<_>::new()
    }};
    ($($args:tt),*) => {{
        let mut list = $crate::data::List::new();
        let mut index = 0;

        $(
            #[allow(unused_assignments)]
            {
                list = list.unshift($args);
                index = index + 1;
            }
        )*

        list.reverse()
    }};
}

#[derive(Debug, PartialEq)]
pub(crate) enum List<T> {
    Empty,
    Normal { car: T, cdr: Box<List<T>> },
}

impl<T> List<T> {
    pub(crate) fn new() -> Self {
        List::Empty
    }

    pub(crate) fn cons(car: T, cdr: List<T>) -> Self {
        List::Normal {
            car,
            cdr: Box::new(cdr),
        }
    }

    pub(crate) fn head(&self) -> Option<&T> {
        match self {
            List::Empty => None,
            List::Normal { car, cdr: _ } => Some(car),
        }
    }

    pub(crate) fn tail(&self) -> &List<T> {
        match self {
            List::Empty => &self,
            List::Normal { car: _, cdr } => cdr,
        }
    }

    pub(crate) fn is_empty(&self) -> bool {
        matches!(self, List::Empty)
    }

    pub(crate) fn push(self, item: T) -> List<T> {
        match self {
            List::Empty => self.unshift(item),
            List::Normal { car, cdr } => List::Normal {
                car,
                cdr: Box::new(cdr.push(item)),
            },
        }
    }

    pub(crate) fn unshift(self, item: T) -> List<T> {
        List::Normal {
            car: item,
            cdr: Box::new(self),
        }
    }

    pub(crate) fn reverse(self) -> List<T> {
        let mut acc = List::new();
        let mut current = self;

        while let List::Normal { car, cdr } = current {
            acc = acc.unshift(car);
            current = *cdr;
        }

        acc
    }

    pub(crate) fn map<CB, R: Display>(self, cb: CB) -> List<R>
    where
        CB: Fn(T) -> R,
    {
        let mut acc = List::new();
        let mut current = self.reverse();

        while let List::Normal { car, cdr } = current {
            acc = acc.unshift(cb(car));
            current = *cdr;
        }

        acc
    }

    pub(crate) fn filter<CB>(self, cb: CB) -> List<T>
    where
        CB: Fn(&T) -> bool,
    {
        let mut acc = List::new();
        let mut current = self.reverse();

        while let List::Normal { car, cdr } = current {
            if cb(&car) {
                acc = acc.unshift(car);
            }

            current = *cdr;
        }

        acc
    }

    pub(crate) fn pop_mut(&mut self) -> Option<T> {
        match take(self) {
            List::Normal { car, cdr } => {
                *self = *cdr;

                Some(car)
            }
            List::Empty => None,
        }
    }
}

impl<T: Display> Display for List<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut str = String::new();
        let mut first = true;
        let mut current = self;

        str.push_str("(");
        while let List::Normal { car, cdr } = current {
            if !first {
                str.push_str(" ");
            }
            str.push_str(&format!("{}", car));
            current = cdr;
            first = false
        }
        str.push_str(")");

        write!(f, "{}", str)
    }
}

impl<T> FromIterator<T> for List<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list = List::new();
        for item in iter {
            list = list.push(item)
        }
        list
    }
}

impl<T> IntoIterator for List<T> {
    type Item = T;
    type IntoIter = ListIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        ListIter { list: Some(self) }
    }
}

pub(crate) struct ListIter<T> {
    list: Option<List<T>>,
}

impl<T> Iterator for ListIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.list.take() {
            Some(List::Normal { car, cdr }) => {
                self.list = Some(*cdr);
                Some(car)
            }
            _ => None,
        }
    }
}

impl<T> Default for List<T> {
    fn default() -> Self {
        List::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_macro_constructor() {
        assert_eq!("(0 1 2 3 4 5)", list![0, 1, 2, 3, 4, 5].to_string());
    }

    #[test]
    fn test_push() {
        let list = List::new().push(10).push(20);

        assert_eq!("(10 20)", list.to_string());
    }

    #[test]
    fn test_unshift() {
        let list = List::new().unshift(10).unshift(20);

        assert_eq!("(20 10)", list.to_string());
    }

    #[test]
    fn test_reverse() {
        let list = list![10, 20].reverse();

        assert_eq!("(20 10)", list.to_string());
    }

    #[test]
    fn test_map() {
        let list = list![10, 20].map(|a| a * 2);

        assert_eq!("(20 40)", list.to_string());
    }

    #[test]
    fn test_filter() {
        let list: List<_> = (0..10).collect::<List<_>>().filter(|i| i % 2 == 0);

        assert_eq!("(0 2 4 6 8)", list.to_string());
    }

    #[test]
    fn test_into_list() {
        let list: List<_> = (0..4).collect();

        assert_eq!("(0 1 2 3)", list.to_string());
    }

    #[test]
    fn test_pop_mut() {
        let mut list: List<_> = (0..4).collect();

        assert_eq!("(0 1 2 3)", list.to_string());

        assert_eq!(Some(0), list.pop_mut());

        assert_eq!("(1 2 3)", list.to_string());
    }
}
