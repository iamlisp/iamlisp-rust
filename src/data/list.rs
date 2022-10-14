use std::fmt::{Display, Formatter};

#[derive(Clone)]
pub(crate) enum List<T: Display> {
    Empty,
    Normal { car: T, cdr: Box<List<T>> },
}

impl<T: Display> List<T> {
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
            List::Empty => &List::Empty,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let list: List<u8> = List::Empty;
        assert_eq!("()", list.to_string());
    }

    #[test]
    fn test_push() {
        let list: List<u8> = List::Empty.push(10).push(20);
        assert_eq!("(10 20)", list.to_string());
    }

    #[test]
    fn test_unshift() {
        let list: List<u8> = List::Empty.unshift(10).unshift(20);
        assert_eq!("(20 10)", list.to_string());
    }
}
