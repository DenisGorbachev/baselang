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
                        substituted_typ.replace(fun_var, &substituted_fun_var)
                    };
                    Fun(substituted_fun_var, Box::new(substituted_typ))
                }
            }
        }
    }

    pub fn contains_var(&self, target: &VarRc) -> bool {
        match self {
            Top => false,
            One(exp) => exp.contains_var(target),
            Fun(fun, typ) => fun.contains_var(target) || typ.contains_var(target),
        }
    }

    pub fn replace(&self, from: &VarRc, to: &VarRc) -> Self {
        match self {
            Top => Top,
            One(exp) => One(exp.replace(from, to)),
            Fun(fun, typ) => {
                let replaced_fun = replace_var_rc(fun, from, to);
                let replaced_typ = typ.replace(from, to);
                Fun(replaced_fun, Box::new(replaced_typ))
            }
        }
    }
}

fn substitute_var_rc(fun: &VarRc, var: &VarRc, arg: &Exp) -> VarRc {
    if fun.contains_var(var) { Rc::new(fun.substitute(var, arg)) } else { fun.clone() }
}

fn replace_var_rc(fun: &VarRc, from: &VarRc, to: &VarRc) -> VarRc {
    if Rc::ptr_eq(fun, from) {
        to.clone()
    } else if fun.contains_var(from) {
        Rc::new(fun.replace_var(from, to))
    } else {
        fun.clone()
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::var;
    use pretty_assertions::assert_eq;

    #[test]
    #[ignore = "known bug: substitution captures free occurrences of later binders"]
    fn must_not_capture_free_var_in_substituted_argument() {
        var!(x);
        var!(y);

        let f_typ = typ!(x => &y);
        let actual = f_typ.substitute(&y, &Exp::sol(&x));

        var!(x_fresh);
        let expected = typ!(x_fresh => &x);

        assert_eq!(actual, expected);
    }

    #[test]
    #[ignore = "known bug: binder freshness is decided by structural equality instead of pointer identity"]
    fn must_refresh_binder_identity_when_substitution_changes_it_to_equal_var() {
        var!(x1: top!(); "x");
        var!(x2: top!(); "x");
        var!(z);

        var!(value: typ!(&x1));
        let input = typ!(value => &x1);
        let after_first = input.substitute(&x1, &Exp::sol(&x2));
        let actual = after_first.substitute(&x1, &Exp::sol(&z));

        var!(expected_value: typ!(&x2));
        let expected = typ!(expected_value => &x2);

        assert_eq!(actual, expected);
    }
}
