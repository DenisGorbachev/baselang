use crate::{Fun, InvalidApplicationError, One, Top, TypBox, Var};
use crate::{Typ, VarRc};
use std::rc::Rc;

#[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Debug)]
pub enum Exp {
    /// [`Sol`] means `solo` (a single variable)
    /// Must wrap [`Var`] in [`Rc`] because a single var can be used in multiple exps (e.g. `Nat` can be used in multiple exps)
    /// Name comes from "solo" (needed a three-letter term that doesn't conflict with other terms)
    Sol(VarRc),
    /// [`App`] means `application` (of one expression on another expression)
    /// IMPORTANT: Never construct this variant directly, call [`Exp::app`] instead (which performs the type check)
    /// Must wrap [`Exp`] in [`Box`] to avoid "recursive type" error
    /// [`Typ`] is a cached type of this expression (calculated in [`Exp::app`])
    App(ExpBox, ExpBox, TypBox),
}

pub use Exp::*;

pub type ExpRc = Rc<Exp>;
pub type ExpBox = Box<Exp>;

impl Exp {
    /// This function accepts `var: &VarRc` to avoid `var.clone()` in the caller (which simplifies its code)
    /// If you want to create an [`Exp`] that takes ownership of the [`VarRc`], use [`Sol`] directly
    #[inline(always)]
    pub fn sol(var: &VarRc) -> Self {
        Sol(var.clone())
    }

    pub fn app(fun: impl Into<Exp>, arg: impl Into<Exp>) -> Result<Self, InvalidApplicationError> {
        let fun = fun.into();
        let arg = arg.into();
        match (fun.typ().clone(), arg.typ().clone()) {
            (Top, b_typ) => Err(InvalidApplicationError::new(Top, b_typ)),
            (One(exp), b_typ) => Err(InvalidApplicationError::new(One(exp), b_typ)),
            (Fun(var, typ), b_typ) => {
                if *var.typ() == b_typ {
                    Ok(App(Box::new(fun), Box::new(arg), typ))
                } else {
                    Err(InvalidApplicationError::new(Fun(var, typ), b_typ))
                }
            }
        }
    }

    pub fn typ(&self) -> &Typ {
        match self {
            Sol(var) => var.typ(),
            App(_, _, typ) => typ,
        }
    }
}

impl From<Var> for Exp {
    fn from(var: Var) -> Self {
        Sol(var.into())
    }
}

impl From<VarRc> for Exp {
    fn from(var: VarRc) -> Self {
        Sol(var)
    }
}

impl TryFrom<(VarRc, VarRc)> for Exp {
    type Error = InvalidApplicationError;

    fn try_from((fun, arg): (VarRc, VarRc)) -> Result<Self, Self::Error> {
        Self::app(fun, arg)
    }
}

impl TryFrom<(&VarRc, &VarRc)> for Exp {
    type Error = InvalidApplicationError;

    fn try_from((fun, arg): (&VarRc, &VarRc)) -> Result<Self, Self::Error> {
        Self::try_from((fun.clone(), arg.clone()))
    }
}

impl TryFrom<(Exp, Exp)> for Exp {
    type Error = InvalidApplicationError;

    fn try_from((fun, arg): (Exp, Exp)) -> Result<Self, Self::Error> {
        Self::app(fun, arg)
    }
}

impl From<(ExpBox, ExpBox, TypBox)> for Exp {
    fn from((fun, arg, typ): (ExpBox, ExpBox, TypBox)) -> Self {
        App(fun, arg, typ)
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
        $crate::Exp::try_from(($crate::exp!($a), $crate::exp!($b))).expect("should pass the type check")
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
        let list_bool = list.of(&bool).expect("should succeed");

        // let list_bool = stub!();
    }
}
