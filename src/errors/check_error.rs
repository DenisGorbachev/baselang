use crate::Typ;
use derive_more::From;
use derive_new::new;
use thiserror::Error;

#[derive(new, Error, From, Eq, PartialEq, Hash, Clone, Debug)]
#[error("invalid application: {fun:?} on {arg:?}")]
pub struct InvalidApplicationError {
    fun: Typ,
    arg: Typ,
}

impl InvalidApplicationError {}
