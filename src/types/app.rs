use crate::Apply;
use derive_getters::Getters;
use errgonomic::handle;
use thiserror::Error;

#[derive(Getters, Clone, Debug)]
pub struct App<F: Apply, A, T> {
    fun: F,
    arg: A,
    typ: T,
}

impl<F, A, T> App<F, A, T>
where
    F: Apply<Arg = A, Typ = T>,
{
    pub fn new(fun: F, arg: A) -> Result<Self, AppNewError<F, A, F::Error>> {
        use AppNewError::*;
        let typ = handle!(fun.apply(&arg), ApplyFailed, fun, arg);
        Ok(Self {
            fun,
            arg,
            typ,
        })
    }
}

#[derive(Error, Clone, Copy, Debug)]
pub enum AppNewError<F, A, E> {
    #[error("failed to apply function to argument")]
    ApplyFailed { source: E, fun: F, arg: A },
}
