use crate::Sol;
use crate::types::exp::Exp;
use crate::types::var::{Var, VarRc};
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
    #[inline(always)]
    pub fn top() -> Self {
        Top
    }

    #[inline(always)]
    pub fn one(exp: impl Into<Exp>) -> Self {
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
                // If this is a different variable, substitute in the resulting type
                // TODO: I think this case should not be possible (in the introduction position, a single var must be referenced at most once)
                // TODO: We can enforce it by changing the `Typ::fun` constructor to take a (nym, typ) instead of (var) and create the var within the constructor itself
                // TODO: It might be necessary to also change the arg `typ` to a closure `impl FnOnce(&VarRc) -> Typ`, so that we could use the newly created var to construct a typ
                if Rc::ptr_eq(fun_var, var) {
                    // If it's the same variable, it shadows the outer one, no substitution needed
                    Fun(fun_var.clone(), typ_box.clone())
                } else {
                    // Create a new variable with potentially updated type
                    let fun_var_typ = fun_var.typ();
                    let substituted_var = match fun_var_typ {
                        // If fun_var's type is directly the var we're substituting
                        One(Sol(v)) if Rc::ptr_eq(v, var) => {
                            // Create a new variable with the type of arg
                            // TODO: is it correct to clone the nym?
                            // TODO: the whole branch for Fun might be incorrect
                            let nym = fun_var.nym().clone();
                            let new_typ = One(arg.clone());
                            Var::new_rc(nym, new_typ)
                        }
                        // For other cases, keep the original variable
                        _ => fun_var.clone(),
                    };
                    Fun(substituted_var, Box::new(typ_box.substitute(var, arg)))
                }
            }
        }
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
