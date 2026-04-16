use crate::types::exp::Exp;
use crate::types::var::{Var, VarRc};
use Exp::Sol;
use std::rc::Rc;

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
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
    #[inline(always)]
    pub fn top() -> Self {
        Top
    }

    #[inline(always)]
    pub fn one(exp: impl Into<Exp>) -> Self {
        // TODO: validate that Exp contains only vars that are either constructors or bound variables (e.g. reject a case where `n : nat` is created with the intention of being a bound var, but not added to a fun type)
        // TODO: This is related to a TODO in Self::substitute
        One(exp.into())
    }

    #[inline(always)]
    pub fn fun(var: &VarRc, typ: impl Into<Typ>) -> Self {
        Fun(var.clone(), Box::new(typ.into()))
    }

    pub fn substitute(&self, var: &VarRc, arg: &Exp) -> Self {
        debug_assert_eq!(var.typ(), arg.typ());
        match self {
            Top => Top,
            One(exp) => One(exp.substitute(var, arg)),
            Fun(fun_var, typ_box) => {
                if Rc::ptr_eq(fun_var, var) {
                    self.clone()
                } else {
                    let substituted_fun_var = substitute_var_rc(fun_var, var, arg);
                    let substituted_typ = typ_box.substitute(var, arg);
                    let substituted_typ = if Rc::ptr_eq(&substituted_fun_var, fun_var) {
                        substituted_typ
                    } else {
                        substituted_typ.replace_var(fun_var, &substituted_fun_var)
                    };
                    Fun(substituted_fun_var, Box::new(substituted_typ))
                }
            }
        }
    }

    pub fn replace_var(&self, from: &VarRc, to: &VarRc) -> Self {
        match self {
            Top => Top,
            One(exp) => One(exp.replace_var(from, to)),
            Fun(fun_var, typ_box) => {
                let replaced_fun_var = replace_var_rc(fun_var, from, to);
                let replaced_typ = typ_box.replace_var(from, to);
                Fun(replaced_fun_var, Box::new(replaced_typ))
            }
        }
    }
}

fn substitute_var_rc(fun_var: &VarRc, var: &VarRc, arg: &Exp) -> VarRc {
    let substituted_var = fun_var.as_ref().substitute(var, arg);
    if substituted_var == **fun_var { fun_var.clone() } else { Rc::new(substituted_var) }
}

fn replace_var_rc(fun_var: &VarRc, from: &VarRc, to: &VarRc) -> VarRc {
    if Rc::ptr_eq(fun_var, from) {
        to.clone()
    } else {
        let replaced_var = fun_var.as_ref().replace_var(from, to);
        if replaced_var == **fun_var { fun_var.clone() } else { Rc::new(replaced_var) }
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
        $crate::Typ::top()
    };
    ($exp: expr) => {
        $crate::Typ::one($exp)
    };
    ($var: ident => $typ: expr) => {
        $crate::Typ::fun(&$var, $typ)
    };
}

#[macro_export]
macro_rules! top {
    () => {
        $crate::Typ::top()
    };
}
