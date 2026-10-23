use crate::Apply;
use derive_getters::Getters;
use std::rc::Rc;

#[derive(Getters, Clone, Debug)]
pub struct App<F, A, T> {
    fun: F,
    arg: A,
    typ: T,
}

pub type AppRc<F, A, T> = Rc<App<F, A, T>>;

impl<F, A, T> App<F, A, T>
where
    F: Apply<Arg = A, Typ = T>,
{
    pub fn new(fun: F, arg: A) -> Result<Self, F::Err> {
        fun.apply(&arg).map(|typ| Self {
            fun,
            arg,
            typ,
        })
    }
}
