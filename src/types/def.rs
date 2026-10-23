use crate::{Exp, InvalidApplicationError, Nym, Of, Typ};
use derive_getters::Getters;
use derive_new::new;

#[derive(new, Getters, Clone, Debug)]
pub struct Def {
    #[new(into)]
    pub nym: Nym,
    #[new(into)]
    pub exp: Exp,
}

impl Def {
    pub fn typ(&self) -> &Typ {
        self.exp.typ()
    }
}

impl<A: Into<Exp>> Of<A> for Def {
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
        let $name = $crate::Def::new(stringify!($name), $exp);
    };
    ($name:ident = $exp:expr; $nym:expr) => {
        let $name = $crate::Def::new($nym, $exp);
    };
}
