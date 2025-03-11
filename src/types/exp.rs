use crate::{CheckError, Fun, InvalidApplication, One, Top, Var};
use crate::{Typ, VarRc};
use derive_more::From;
use std::rc::Rc;

#[derive(From, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Debug)]
pub enum Exp {
    /// `Sol` means `solo` (a single variable)
    /// Must wrap [`Var`] in [`Rc`] because a single var can be used in multiple exps (e.g. `Nat` can be used in multiple exps)
    /// Name comes from "solo" (needed a three-letter term that doesn't conflict with other terms)
    Sol(VarRc),
    /// `App` means `application` (of one expression on another expression)
    /// Must wrap [`Exp`] in [`Box`] to avoid "recursive type" error
    App(ExpBox, ExpBox),
}

pub use Exp::*;

pub type ExpRc = Rc<Exp>;
pub type ExpBox = Box<Exp>;

impl Exp {
    /// This function accepts `var: &VarRc` to avoid `var.clone()` in the caller (which simplifies its code)
    /// If you want to create an [`Exp`] that takes ownership of the [`VarRc`], use [`Exp::Sol`] directly
    #[inline(always)]
    pub fn sol(var: &VarRc) -> Self {
        Sol(var.clone())
    }

    #[inline(always)]
    pub fn app(fun: impl Into<Exp>, arg: impl Into<Exp>) -> Self {
        App(Box::new(fun.into()), Box::new(arg.into()))
    }

    pub fn check(&self) -> Result<(), CheckError> {
        match self {
            Sol(_) => Ok(()),
            App(a, b) => match (a.typ()?, b.typ()?) {
                (Top, b_typ) => Err(InvalidApplication(Top, b_typ.clone())),
                (One(exp), b_typ) => Err(InvalidApplication(One(exp.clone()), b_typ.clone())),
                (Fun(var, typ), b_typ) => match var.typ() == b_typ {
                    true => Ok(()),
                    false => Err(InvalidApplication(Fun(var.clone(), typ.clone()), b_typ.clone())),
                },
            },
        }
    }

    pub fn typ(&self) -> Result<&Typ, CheckError> {
        match self {
            Sol(var) => Ok(var.typ()),
            App(a, b) => match a.typ()? {
                Top => Err(InvalidApplication(Top, b.typ()?.clone())),
                One(exp) => Err(InvalidApplication(One(exp.clone()), b.typ()?.clone())),
                Fun(_input, output) => Ok(output.as_ref()),
            },
        }
    }
}

impl From<Var> for Exp {
    fn from(var: Var) -> Self {
        Sol(var.into())
    }
}

impl From<(VarRc, VarRc)> for Exp {
    fn from((fun, arg): (VarRc, VarRc)) -> Self {
        App(Sol(fun).into(), Sol(arg).into())
    }
}

impl From<(&VarRc, &VarRc)> for Exp {
    fn from((fun, arg): (&VarRc, &VarRc)) -> Self {
        App(Sol(fun.clone()).into(), Sol(arg.clone()).into())
    }
}

impl From<(Exp, Exp)> for Exp {
    fn from((fun, arg): (Exp, Exp)) -> Self {
        App(Box::new(fun), Box::new(arg))
    }
}

/// This macro uses `$var.clone()` to avoid the "&" before variables.
/// `$var` should have a [`VarRc`] type.
/// `$var.clone()` is also used in [`typ`](crate::typ) macro.
#[macro_export]
macro_rules! exp {
    ($var: expr) => {
        $crate::Exp::Sol($var.clone())
    };
    ($a: expr, $b: expr) => {
        $crate::Exp::from(($crate::exp!($a), $crate::exp!($b)))
    };
}

#[cfg(test)]
mod tests {
    use crate::{Bool, List, Of};

    #[test]
    #[allow(unused_variables)]
    fn must_error_for_wrong_types() {
        let Bool {
            bool,
            yes,
            no,
        } = Bool::default();
        let List {
            list,
            nil,
            cons,
        } = List::default();
        let list_bool = list.of(&bool);

        // let list_bool = stub!();
    }
}
