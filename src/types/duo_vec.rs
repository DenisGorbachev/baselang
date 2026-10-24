use std::rc::Rc;
use thiserror::Error;

#[doc = "Invariant: `self.iter().take(2).count() == 2`; iteration yields the prefix followed by the tail."]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DuoVec<T> {
    prefix: [T; 2],
    tail: Vec<T>,
}

pub type DuoVecRc<T> = Rc<DuoVec<T>>;

impl<T> DuoVec<T> {
    pub fn first(&self) -> &T {
        let [first, _] = &self.prefix;
        first
    }

    pub fn last(&self) -> &T {
        match self.tail.last() {
            Some(last) => last,
            None => {
                let [_, last] = &self.prefix;
                last
            }
        }
    }

    pub fn iter(&self) -> impl DoubleEndedIterator<Item = &T> {
        self.prefix.iter().chain(&self.tail)
    }
}

impl<T> From<[T; 2]> for DuoVec<T> {
    fn from(prefix: [T; 2]) -> Self {
        Self {
            prefix,
            tail: Vec::new(),
        }
    }
}

impl<T> TryFrom<Vec<T>> for DuoVec<T> {
    type Error = TryFromVecForDuoVecError<T>;

    fn try_from(values: Vec<T>) -> Result<Self, Self::Error> {
        use TryFromVecForDuoVecError::*;
        let mut values = values.into_iter();
        match (values.next(), values.next()) {
            (Some(first), Some(second)) => Ok(Self {
                prefix: [first, second],
                tail: values.collect(),
            }),
            (first, second) => Err(ConversionFailed {
                values: first.into_iter().chain(second).chain(values).collect(),
            }),
        }
    }
}

impl<T> From<DuoVec<T>> for Vec<T> {
    fn from(value: DuoVec<T>) -> Self {
        value.prefix.into_iter().chain(value.tail).collect()
    }
}

#[derive(Error, Debug)]
pub enum TryFromVecForDuoVecError<T> {
    #[error("expected at least two elements, received {len}", len = values.len())]
    ConversionFailed { values: Vec<T> },
}
