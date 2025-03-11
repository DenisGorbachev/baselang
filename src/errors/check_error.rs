use crate::Typ;
use derive_more::{Error, From};
use fmt_derive::Display;
pub use CheckError::*;

#[derive(Error, Display, From, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Debug)]
pub enum CheckError {
    InvalidApplication(Typ, Typ),
}

impl CheckError {}
