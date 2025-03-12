use crate::Typ;
use derive_more::{Error, From};
use derive_new::new;
use fmt_derive::Display;

#[derive(new, Error, Display, From, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Debug)]
pub struct InvalidApplicationError {
    fun: Typ,
    arg: Typ,
}

impl InvalidApplicationError {}
