use crate::types::exp::Exp;
use crate::types::var::{Var, VarRc};
use crate::{AlphaEq, DuoVec, alpha_eq_typ};
use Exp::Sol;
use std::rc::Rc;

/// [`Typ`] must not derive [`Eq`], [`PartialEq`], [`Hash`] because [`Var`] must not derive them.
/// [`Typ`] may be compared semantically via [`AlphaEq`].
#[derive(Clone, Debug)]
pub enum Typ {
    /// Needed because some vars have fun types that end in a top (e.g. `List : (t : Top) -> Top`)
    Top,
    /// Maybe it can own the exp
    /// Name is chosen so that it could be exported (doesn't conflict with other names)
    One(/* exp */ Exp),
    /// Must wrap [`Var`] in [`Rc`] because function vars may be reused in dependent output types.
    Fun(DuoVec<VarRc>),
}

pub use Typ::*;

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
    pub fn fun(vars: impl IntoIterator<Item = VarRc>) -> Self {
        let vars = vars.into_iter().collect::<Vec<_>>();
        let vars = DuoVec::try_from(vars).expect("always succeeds because Typ::fun is called only with at least two vars");
        Fun(vars)
    }

    /// Returns output type of the function (or self if it's not a function)
    pub fn last(&self) -> &Self {
        match self {
            Fun(vars) => vars.last().typ().last(),
            _ => self,
        }
    }

    pub fn substitute(&self, var: &VarRc, arg: &Exp) -> Self {
        debug_assert!(var.typ().alpha_eq(arg.typ()));
        match self {
            Top => Top,
            One(exp) => One(exp.substitute(var, arg)),
            Fun(vars) => Fun(substitute_fun_vars(vars, var, arg)),
        }
    }

    pub fn contains_var(&self, target: &VarRc) -> bool {
        match self {
            Top => false,
            One(exp) => exp.contains_var(target),
            Fun(vars) => vars.iter().any(|var| var.contains_var(target)),
        }
    }

    pub fn replace(&self, from: &VarRc, to: &VarRc) -> Self {
        match self {
            Top => Top,
            One(exp) => One(exp.replace(from, to)),
            Fun(vars) => Fun(map_fun_vars(vars, |var| replace_bound_var_rc(var, from, to))),
        }
    }

    pub fn after_apply(&self, arg: &Exp) -> Option<Self> {
        match self {
            Fun(vars) => Some(fun_after_apply(vars, arg)),
            _ => None,
        }
    }
}

fn substitute_fun_vars(vars: &DuoVec<VarRc>, var: &VarRc, arg: &Exp) -> DuoVec<VarRc> {
    let mut replacements = Vec::<(VarRc, VarRc)>::new();
    let mut shadowed = false;
    map_fun_vars(vars, |current| {
        let current_new = apply_bound_var_replacements(current, &replacements);
        let current_new = if shadowed || Rc::ptr_eq(current, var) {
            current_new
        } else {
            substitute_bound_var_rc(&current_new, var, arg)
        };
        if !Rc::ptr_eq(&current_new, current) {
            replacements.push((current.clone(), current_new.clone()));
        }
        if Rc::ptr_eq(current, var) {
            shadowed = true;
        }
        current_new
    })
}

fn fun_after_apply(vars: &DuoVec<VarRc>, arg: &Exp) -> Typ {
    let input = vars.first();
    let mut replacements = Vec::<(VarRc, VarRc)>::new();
    let remaining = vars
        .iter()
        .skip(1)
        .map(|current| {
            let current_new = apply_bound_var_replacements(current, &replacements);
            let current_new = substitute_bound_var_rc(&current_new, input, arg);
            if !Rc::ptr_eq(&current_new, current) {
                replacements.push((current.clone(), current_new.clone()));
            }
            current_new
        })
        .collect::<Vec<_>>();
    typ_from_fun_tail(remaining)
}

fn typ_from_fun_tail(mut vars: Vec<VarRc>) -> Typ {
    if vars.get(1).is_none() {
        return vars
            .pop()
            .expect("always succeeds because a one-item Vec contains one output var")
            .typ()
            .clone();
    }
    Fun(DuoVec::try_from(vars).expect("always succeeds because a function tail with more than one var has at least two vars"))
}

fn map_fun_vars(vars: &DuoVec<VarRc>, mapper: impl FnMut(&VarRc) -> VarRc) -> DuoVec<VarRc> {
    DuoVec::try_from(vars.iter().map(mapper).collect::<Vec<_>>()).expect("always succeeds because mapping preserves DuoVec length")
}

fn apply_bound_var_replacements(bound: &VarRc, replacements: &[(VarRc, VarRc)]) -> VarRc {
    replacements
        .iter()
        .fold(bound.clone(), |bound, (from, to)| replace_bound_var_rc(&bound, from, to))
}

fn substitute_bound_var_rc(bound: &VarRc, var: &VarRc, arg: &Exp) -> VarRc {
    if bound.contains_var(var) { Rc::new(bound.substitute(var, arg)) } else { bound.clone() }
}

fn replace_bound_var_rc(bound: &VarRc, from: &VarRc, to: &VarRc) -> VarRc {
    if Rc::ptr_eq(bound, from) {
        to.clone()
    } else if bound.contains_var(from) {
        Rc::new(bound.replace_var(from, to))
    } else {
        bound.clone()
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

impl<const N: usize> From<[&VarRc; N]> for Typ {
    fn from(vars: [&VarRc; N]) -> Self {
        Self::fun(vars.into_iter().cloned())
    }
}

impl AlphaEq for Typ {
    fn alpha_eq(&self, other: &Self) -> bool {
        alpha_eq_typ(self, other)
    }
}

/// This macro accepts either a plain type expression or a function type.
#[macro_export]
macro_rules! typ {
    () => {
        $crate::Typ::top()
    };
    ($exp: expr) => {
        $crate::Typ::from($exp)
    };
    ($first: expr $(=> $rest: expr)+) => {
        $crate::Typ::from([$first, $($rest),+])
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Equality, InvalidApplicationError, Of, exp, var};
    use pretty_assertions::assert_matches;

    #[test]
    fn must_compare_types_up_to_alpha_equivalence() {
        var!(x);
        var!(y);
        var!(z);
        var!(ox: typ!(&x));
        var!(oy: typ!(&y));
        var!(oz: typ!(&z));

        let left = typ!(&x => &ox);
        let right = typ!(&y => &oy);
        let different = typ!(&y => &oz);

        assert!(left.alpha_eq(&right));
        assert!(!left.alpha_eq(&different));
    }

    #[test]
    fn must_substitute_all_instances() {
        var!(t);
        var!(u);
        var!(a: typ!(&t));
        var!(b: typ!(&t));

        let (eq, refl) = Equality::default().into();

        // NOTE: `eq t t` and `refl t` are not even expressible in Rust, because `Typ::Top` is not a `Var`, so it can't be passed as a first argument to `eq` or `refl`

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
    fn must_not_capture_free_var_in_substituted_argument() {
        var!(x);
        var!(y);
        var!(oy: typ!(&y));

        let f_typ = typ!(&x => &oy);
        let actual = f_typ.substitute(&y, &Exp::sol(&x));

        var!(x_fresh);
        var!(o_fresh: typ!(&x));
        let expected = typ!(&x_fresh => &o_fresh);

        assert!(!f_typ.alpha_eq(&expected));
        assert!(actual.alpha_eq(&expected));
    }

    #[test]
    fn must_refresh_binder_identity_when_substitution_changes_it_to_equal_var() {
        // vars are still different even if they have the same name
        var!(y1: typ!(); "y");
        var!(y2: typ!(); "y");
        var!(z);

        var!(u1: typ!(&y1));
        var!(o1: typ!(&y1));
        let f_typ = typ!(&u1 => &o1);

        // this substitution must change the type
        let actual_1 = f_typ.substitute(&y1, &Exp::sol(&y2));

        // this substitution must not change the type (because y1 has already been substituted)
        let actual_2 = actual_1.substitute(&y1, &Exp::sol(&z));

        var!(v2: typ!(&y2));
        var!(o2: typ!(&y2));
        let expected = typ!(&v2 => &o2);

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
