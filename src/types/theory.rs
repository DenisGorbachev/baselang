use crate::{Scope, Validator, ValidatorValidateError};
use derive_getters::Getters;
use errgonomic::handle;
use std::rc::Rc;
use thiserror::Error;

#[derive(Getters, Clone, Debug)]
pub struct Theory<S = Scope> {
    scope: S,
}

pub type TheoryRc<S = Scope> = Rc<Theory<S>>;

impl Theory {
    pub fn new(scope: Scope) -> Result<Self, TheoryNewError> {
        use TheoryNewError::*;
        handle!(Validator::new(&scope).validate(), ValidateFailed, scope);
        Ok(Self {
            scope,
        })
    }
}

#[derive(Error, Debug)]
pub enum TheoryNewError {
    #[error("the theory is invalid")]
    ValidateFailed { source: ValidatorValidateError, scope: Box<Scope> },
}
