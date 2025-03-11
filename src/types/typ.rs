use crate::types::exp::Exp;
use crate::types::var::{Var, VarRc};
use crate::Sol;
use std::rc::Rc;

#[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Debug)]
pub enum Typ {
    /// Needed because some vars have fun types that end in a top (e.g. `List : (t : Top) -> Top`)
    Top,
    /// Maybe it can own the exp
    /// Name is chosen so that it could be exported (doesn't conflict with other names)
    One(Exp),
    /// Must wrap [`Var`] in [`Rc`] because this var may be used in the following typ (e.g. in `Nil : (t : Top) -> List t`, the `t` var is used in `List t`)
    /// Must wrap [`Typ`] in [`Box`] to avoid "recursive type" error
    Fun(VarRc, TypBox),
}

pub use Typ::*;

pub type TypRc = Rc<Typ>;
pub type TypBox = Box<Typ>;

impl Typ {
    pub fn top() -> Self {
        Top
    }

    pub fn one(exp: impl Into<Exp>) -> Self {
        One(exp.into())
    }

    pub fn fun(var: &VarRc, typ: impl Into<Typ>) -> Self {
        Fun(var.clone(), Box::new(typ.into()))
    }
}

impl From<Var> for Typ {
    #[inline(always)]
    fn from(var: Var) -> Self {
        One(Sol(Rc::new(var)))
    }
}

impl From<VarRc> for Typ {
    #[inline(always)]
    fn from(var: VarRc) -> Self {
        One(Sol(var))
    }
}

impl From<&VarRc> for Typ {
    #[inline(always)]
    fn from(var: &VarRc) -> Self {
        One(Sol(var.clone()))
    }
}

impl From<Exp> for Typ {
    #[inline(always)]
    fn from(exp: Exp) -> Self {
        One(exp)
    }
}

impl From<(Var, Typ)> for Typ {
    #[inline(always)]
    fn from((var, typ): (Var, Typ)) -> Self {
        Fun(Rc::new(var), Box::new(typ))
    }
}

impl From<(VarRc, Typ)> for Typ {
    #[inline(always)]
    fn from((var, typ): (VarRc, Typ)) -> Self {
        Fun(var, Box::new(typ))
    }
}

impl From<(&VarRc, Typ)> for Typ {
    #[inline(always)]
    fn from((var, typ): (&VarRc, Typ)) -> Self {
        Self::from((var.clone(), typ))
    }
}

impl From<(&VarRc, Exp)> for Typ {
    #[inline(always)]
    fn from((var, exp): (&VarRc, Exp)) -> Self {
        Self::from((var.clone(), Self::from(exp)))
    }
}

/// This macro uses `$var.clone()` to avoid the "&" before variables.
/// `$var` should have a [`VarRc`] type.
/// `$var.clone()` is also used in [`exp`](crate::exp) macro.
#[macro_export]
macro_rules! typ {
    () => {
        $crate::Typ::Top
    };
    ($exp: expr) => {
        $crate::Typ::One($exp)
    };
    ($var: ident => $typ: expr) => {
        $crate::Typ::Fun($var.clone(), Box::new($typ))
    };
}
