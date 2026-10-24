use crate::{AlphaEq, Exp, Substitution, Sym, SymRc, alpha_eq_typ};
use Exp::Sol;
use Typ::*;
use std::iter::successors;
use std::ptr::NonNull;
use std::rc::Rc;

/// [`Typ`] may be compared semantically via [`AlphaEq`].
#[derive(Clone, Debug)]
pub enum Typ {
    /// Needed because some syms have fun types that end in a top (e.g. `List : (t : Top) -> Top`)
    Top,
    /// Maybe it can own the exp
    /// Name is chosen so that it could be exported (doesn't conflict with other names)
    One(/* exp */ Exp),
    /// Must wrap [`Sym`] in [`Rc`] because this sym may be used in the following typ (e.g. in `Nil : (t : Top) -> List t`, the `t` sym is used in `List t`)
    /// Must wrap [`Typ`] in [`Box`] to avoid "recursive type" error
    Fun(/* param */ SymRc, /* typ */ TypBox),
}

pub type TypRc = Rc<Typ>;
pub type TypBox = Box<Typ>;

impl Typ {
    pub fn parameters(&self) -> impl Iterator<Item = &SymRc> {
        successors(Some(self), |typ| match typ {
            Fun(_, body) => Some(body.as_ref()),
            _ => None,
        })
        .filter_map(|typ| match typ {
            Fun(param, _) => Some(param),
            _ => None,
        })
    }

    pub fn instantiate(&self, substitution: &Substitution) -> Self {
        match self {
            Top => Top,
            One(exp) => One(exp.instantiate(substitution)),
            Fun(param, body) => {
                let typ = param.typ().instantiate(substitution);
                let mut inner = substitution.clone();
                let key = NonNull::from(param.as_ref());
                inner.remove(&key);
                let captures = substitution.values().any(|value| value.contains_sym(param));
                let param_new = if typ.alpha_eq(param.typ()) && !captures {
                    param.clone()
                } else {
                    let fresh = Sym::new_rc(param.nym().clone(), typ);
                    inner.insert(key, Exp::sol(&fresh));
                    fresh
                };
                Fun(param_new, Box::new(body.instantiate(&inner)))
            }
        }
    }

    #[inline(always)]
    pub fn top() -> Self {
        Top
    }

    #[inline(always)]
    pub fn one(exp: impl Into<Exp>) -> Self {
        // TODO: validate that Exp contains only syms that are either constructors or bound variables (e.g. reject a case where `n : nat` is created with the intention of being a bound sym, but not added to a fun type)
        // TODO: This is related to a TODO in Self::substitute
        One(exp.into())
    }

    #[inline(always)]
    pub fn fun(param: &SymRc, typ: impl Into<Typ>) -> Self {
        Fun(param.clone(), Box::new(typ.into()))
    }

    /// Returns output type of the function (or self if it's not a function)
    pub fn last(&self) -> &Self {
        let mut typ = self;
        while let Fun(_, output) = typ {
            typ = output;
        }
        typ
    }

    pub fn head(&self) -> Option<&SymRc> {
        match self {
            One(exp) => Some(exp.head()),
            Top | Fun(_, _) => None,
        }
    }

    pub fn substitute(&self, sym: &SymRc, arg: &Exp) -> Self {
        match self {
            Top => Top,
            One(exp) => One(exp.substitute(sym, arg)),
            Fun(param, typ) => {
                if Rc::ptr_eq(param, sym) {
                    self.clone()
                } else {
                    let substituted_param = param.substitute_rc(sym, arg);
                    let substituted_typ = typ.substitute(sym, arg);
                    let substituted_typ = if Rc::ptr_eq(&substituted_param, param) {
                        substituted_typ
                    } else {
                        substituted_typ.replace(param, &substituted_param)
                    };
                    Fun(substituted_param, Box::new(substituted_typ))
                }
            }
        }
    }

    pub fn contains_sym(&self, target: &SymRc) -> bool {
        match self {
            Top => false,
            One(exp) => exp.contains_sym(target),
            Fun(param, typ) => param.contains_sym(target) || typ.contains_sym(target),
        }
    }

    pub fn replace(&self, from: &SymRc, to: &SymRc) -> Self {
        match self {
            Top => Top,
            One(exp) => One(exp.replace(from, to)),
            Fun(param, typ) => {
                let replaced_param = param.replace_rc(from, to);
                let replaced_typ = typ.replace(from, to);
                let replaced_typ = if Rc::ptr_eq(&replaced_param, param) {
                    replaced_typ
                } else {
                    replaced_typ.replace(param, &replaced_param)
                };
                Fun(replaced_param, Box::new(replaced_typ))
            }
        }
    }
}

impl From<Sym> for Typ {
    #[inline(always)]
    fn from(sym: Sym) -> Self {
        One(Sol(Rc::new(sym)))
    }
}

impl From<SymRc> for Typ {
    #[inline(always)]
    fn from(sym: SymRc) -> Self {
        One(Sol(sym))
    }
}

impl From<&SymRc> for Typ {
    #[inline(always)]
    fn from(sym: &SymRc) -> Self {
        One(Sol(sym.clone()))
    }
}

impl From<Exp> for Typ {
    #[inline(always)]
    fn from(exp: Exp) -> Self {
        One(exp)
    }
}

impl From<(Sym, Typ)> for Typ {
    #[inline(always)]
    fn from((param, typ): (Sym, Typ)) -> Self {
        Fun(Rc::new(param), Box::new(typ))
    }
}

impl From<(SymRc, Typ)> for Typ {
    #[inline(always)]
    fn from((param, typ): (SymRc, Typ)) -> Self {
        Fun(param, Box::new(typ))
    }
}

impl From<(&SymRc, Typ)> for Typ {
    #[inline(always)]
    fn from((sym, typ): (&SymRc, Typ)) -> Self {
        Self::from((sym.clone(), typ))
    }
}

impl From<(&SymRc, Exp)> for Typ {
    #[inline(always)]
    fn from((sym, exp): (&SymRc, Exp)) -> Self {
        Self::from((sym.clone(), Self::from(exp)))
    }
}

impl AlphaEq for Typ {
    fn alpha_eq(&self, other: &Self) -> bool {
        alpha_eq_typ(self, other)
    }
}

/// This macro uses `$sym.clone()` to avoid the "&" before variables.
/// `$sym` should have a [`SymRc`] type.
/// `$sym.clone()` is also used in [`exp`](crate::exp) macro.
#[macro_export]
macro_rules! typ {
    () => {
        $crate::Typ::top()
    };
    ($exp: expr) => {
        $crate::Typ::one($exp)
    };
    ($sym: ident => $typ: expr) => {
        $crate::Typ::fun(&$sym, $typ)
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
    use crate::{Equality, InvalidApplicationError, Of, exp, sym};
    use pretty_assertions::assert_matches;

    #[test]
    fn must_compare_types_up_to_alpha_equivalence() {
        sym!(x);
        sym!(y);
        sym!(z);

        let left = typ!(x => &x);
        let right = typ!(y => &y);
        let different = typ!(y => &z);

        assert!(left.alpha_eq(&right));
        assert!(!left.alpha_eq(&different));
    }

    #[test]
    fn must_substitute_all_instances() {
        sym!(t);
        sym!(u);
        sym!(a: typ!(&t));
        sym!(b: typ!(&t));

        let (eq, refl) = Equality::default().into();

        // NOTE: `eq t t` and `refl t` are not even expressible in Rust, because `Typ::Top` is not a `Sym`, so it can't be passed as a first argument to `eq` or `refl`

        let eq_t_a_a = exp!(&eq, &t, &a, &a);
        let eq_t_b_b = exp!(&eq, &t, &b, &b); // can be expressed but can't be constructed
        let refl_t = exp!(&refl, &t);
        let refl_t_a = exp!(&refl_t, &a);
        let refl_t_b = exp!(&refl_t, &b);

        assert_matches!(refl_t.of(&t), Err(InvalidApplicationError { .. }), "`refl t t` is a type error because it's not the case that `t : t`");
        assert_matches!(refl_t.of(&u), Err(InvalidApplicationError { .. }), "`refl t u` is a type error because it's not the case that `u : t`");

        assert!(refl_t_a.typ().alpha_eq(&Typ::from(eq_t_a_a)), "`refl t a : eq t a a`");
        assert!(refl_t_b.typ().alpha_eq(&Typ::from(eq_t_b_b)), "`refl t b : eq t b b`");
    }

    #[test]
    #[ignore = "known bug: substitution captures free occurrences of later binders"]
    fn must_not_capture_free_sym_in_substituted_argument() {
        sym!(x);
        sym!(y);

        let f_typ = typ!(x => &y);
        let actual = f_typ.substitute(&y, &Exp::sol(&x));

        sym!(x_fresh);
        let expected = typ!(x_fresh => &x);

        assert!(!f_typ.alpha_eq(&expected));
        assert!(actual.alpha_eq(&expected));
    }

    #[test]
    fn must_refresh_binder_identity_when_substitution_changes_it_to_equal_sym() {
        // syms are still different even if they have the same name
        sym!(y1: top!(); "y");
        sym!(y2: top!(); "y");
        sym!(z);

        sym!(x1: typ!(&y1));
        let f_typ = typ!(x1 => &y1);

        // this substitution must change the type
        let actual_1 = f_typ.substitute(&y1, &Exp::sol(&y2));

        // this substitution must not change the type (because y1 has already been substituted)
        let actual_2 = actual_1.substitute(&y1, &Exp::sol(&z));

        sym!(v2: typ!(&y2));
        let expected = typ!(v2 => &y2);

        assert!(!f_typ.alpha_eq(&expected));
        assert!(actual_1.alpha_eq(&expected));
        assert!(actual_2.alpha_eq(&expected));
    }

    #[ignore]
    #[test]
    fn must_admit_proof_of_add_2_2_eq_4() {
        todo!()
    }
}
