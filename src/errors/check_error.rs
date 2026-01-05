use crate::Typ;
use derive_more::{Display, Error, From};
use derive_new::new;

#[derive(new, Error, Display, From, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Debug)]
#[display("invalid application: {fun:?} on {arg:?}")]
pub struct InvalidApplicationError {
    fun: Typ,
    arg: Typ,
}

impl InvalidApplicationError {}
