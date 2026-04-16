use crate::{AlphaEq, Typ};
use derive_more::From;
use derive_new::new;
use thiserror::Error;

#[derive(new, Error, From, Clone, Debug)]
#[error("invalid application: {fun:?} on {arg:?}")]
pub struct InvalidApplicationError {
    fun: Typ,
    arg: Typ,
}

impl AlphaEq for InvalidApplicationError {
    fn alpha_eq(&self, other: &Self) -> bool {
        self.fun.alpha_eq(&other.fun) && self.arg.alpha_eq(&other.arg)
    }
}
