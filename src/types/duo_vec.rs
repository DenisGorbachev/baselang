use derive_more::{Deref, Into};
use thiserror::Error;

/// A vector that always contains at least two items.
#[derive(Deref, Into, Clone, Debug)]
pub struct DuoVec<T>(Vec<T>);

impl<T> DuoVec<T> {
    pub fn first(&self) -> &T {
        self.0
            .first()
            .expect("always succeeds because DuoVec always has at least 2 elements")
    }

    pub fn first_mut(&mut self) -> &mut T {
        self.0
            .first_mut()
            .expect("always succeeds because DuoVec always has at least 2 elements")
    }

    pub fn last(&self) -> &T {
        self.0
            .last()
            .expect("always succeeds because DuoVec always has at least 2 elements")
    }

    pub fn last_mut(&mut self) -> &mut T {
        self.0
            .last_mut()
            .expect("always succeeds because DuoVec always has at least 2 elements")
    }

    pub fn prelast(&self) -> &T {
        let (_last, rest) = self
            .split_last()
            .expect("always succeeds because DuoVec always has at least 2 elements");
        rest.last()
            .expect("always succeeds because DuoVec always has at least 2 elements")
    }

    pub fn prelast_mut(&mut self) -> &mut T {
        let (_last, rest) = self
            .0
            .split_last_mut()
            .expect("always succeeds because DuoVec always has at least 2 elements");
        rest.last_mut()
            .expect("always succeeds because DuoVec always has at least 2 elements")
    }
}

impl<T> TryFrom<Vec<T>> for DuoVec<T> {
    type Error = TryFromVecForDuoVecError<T>;

    fn try_from(vec: Vec<T>) -> Result<Self, Self::Error> {
        match vec.get(1) {
            Some(_) => Ok(Self(vec)),
            None => Err(TryFromVecForDuoVecError::VecLenCheckFailed {
                vec,
            }),
        }
    }
}

impl<T> IntoIterator for DuoVec<T> {
    type Item = T;
    type IntoIter = <Vec<T> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a DuoVec<T> {
    type Item = &'a T;
    type IntoIter = core::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

#[derive(Error, Debug)]
pub enum TryFromVecForDuoVecError<T> {
    #[error("expected a vector with len >= 2, found a vec with len = {len}", len = vec.len())]
    VecLenCheckFailed { vec: Vec<T> },
}
