use crate::{Exp, InvalidApplicationError, Nym, Of, Typ};
use derive_getters::Getters;
use derive_new::new;
use std::rc::Rc;

#[derive(new, Getters, Clone, Debug)]
pub struct Def<N = Nym, E = Exp> {
    #[new(into)]
    pub nym: N,
    #[new(into)]
    exp: E,
}

pub type DefRc<N = Nym, E = Exp> = Rc<Def<N, E>>;

impl<N, E> From<Def<N, E>> for (N, E) {
    fn from(def: Def<N, E>) -> Self {
        let Def {
            nym,
            exp,
        } = def;
        (nym, exp)
    }
}

impl<N> Def<N, Exp> {
    pub fn typ(&self) -> &Typ {
        self.exp.typ()
    }
}

impl<N, A: Into<Exp>> Of<A> for Def<N, Exp> {
    fn of(&self, arg: A) -> Result<Exp, InvalidApplicationError> {
        self.exp.of(arg)
    }

    fn of_at(&self, arg: A, index: usize) -> Result<Exp, InvalidApplicationError> {
        self.exp.of_at(arg, index)
    }

    fn of_smart(&self, arg: A) -> Option<Exp> {
        self.exp.of_smart(arg)
    }
}

#[macro_export]
macro_rules! def {
    ($name:ident = $exp:expr) => {
        let $name = $crate::Def::<$crate::Nym, $crate::Exp>::new(stringify!($name), $exp);
    };
    ($name:ident = $exp:expr; $nym:expr) => {
        let $name = $crate::Def::<$crate::Nym, $crate::Exp>::new($nym, $exp);
    };
}
