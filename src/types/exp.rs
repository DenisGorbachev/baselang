use crate::{AlphaEq, Binding, Def, DefRc, InvalidApplicationError, Of, Sym, TypBox, alpha_eq_exp};
use crate::{SymRc, Typ};
use Exp::*;
use Typ::*;
use std::rc::Rc;

/// [`Exp`] may be compared semantically via [`AlphaEq`].
#[derive(Clone, Debug)]
pub enum Exp {
    /// [`Sol`] means `solo` (a single symbol)
    /// Must wrap [`Sym`] in [`Rc`] because a single sym can be used in multiple exps (e.g. `Nat` can be used in multiple exps)
    /// Name comes from "solo" (needed a three-letter term that doesn't conflict with other terms)
    Sol(/* sym */ SymRc),
    /// [`App`] means `application` (of one expression on another expression)
    /// IMPORTANT: Never construct this variant directly, call [`Exp::app`] instead (which performs the type check)
    /// Must wrap [`Exp`] in [`Box`] to avoid "recursive type" error
    /// [`TypBox`] is a cached type of this expression (calculated in [`Exp::app`])
    App(/* fun */ ExpBox, /* arg */ ExpBox, /* typ */ TypBox),
}

pub type ExpRc = Rc<Exp>;
pub type ExpBox = Box<Exp>;

impl Exp {
    pub fn head(&self) -> &SymRc {
        let mut exp = self;
        loop {
            match exp {
                Sol(sym) => return sym,
                App(fun, _, _) => exp = fun,
            }
        }
    }

    /// This function accepts `sym: &SymRc` to avoid `sym.clone()` in the caller (which simplifies its code)
    /// If you want to create an [`Exp`] that takes ownership of the [`SymRc`], use [`Sol`] directly
    #[inline(always)]
    pub fn sol(sym: &SymRc) -> Self {
        Sol(sym.clone())
    }

    pub fn app(fun: impl Into<Exp>, arg: impl Into<Exp>) -> Result<Self, InvalidApplicationError> {
        let fun = fun.into();
        let arg = arg.into();
        match (fun.typ().clone(), arg.typ().clone()) {
            (Top, arg_typ) => Err(InvalidApplicationError::new(Top, arg_typ)),
            (One(exp), arg_typ) => Err(InvalidApplicationError::new(One(exp), arg_typ)),
            (Fun(param, typ_old), arg_typ) => {
                if param.typ().alpha_eq(&arg_typ) {
                    let typ_new = Box::new(typ_old.substitute(&param, &arg));
                    Ok(App(Box::new(fun), Box::new(arg), typ_new))
                } else {
                    Err(InvalidApplicationError::new(Fun(param, typ_old), arg_typ))
                }
            }
        }
    }

    pub fn typ(&self) -> &Typ {
        match self {
            Sol(sym) => sym.typ(),
            App(_, _, typ) => typ,
        }
    }

    /// In `self` expression, substitute symbol `sym` with expression `arg`
    ///
    /// The caller must ensure that `sym` and `arg` have the same type.
    pub fn substitute(&self, sym: &SymRc, arg: &Exp) -> Self {
        debug_assert!(sym.typ().alpha_eq(arg.typ()));
        match self {
            Sol(sym_inner) => {
                if Rc::ptr_eq(sym_inner, sym) {
                    arg.clone()
                } else {
                    self.clone()
                }
            }
            App(fun_inner, arg_inner, typ_inner) => {
                let new_fun_inner = fun_inner.substitute(sym, arg);
                let new_arg_inner = arg_inner.substitute(sym, arg);
                let new_typ_inner = typ_inner.substitute(sym, arg);
                App(Box::new(new_fun_inner), Box::new(new_arg_inner), Box::new(new_typ_inner))
            }
        }
    }

    pub fn contains_sym(&self, target: &SymRc) -> bool {
        match self {
            Sol(sym) => Rc::ptr_eq(sym, target),
            App(fun, arg, typ) => fun.contains_sym(target) || arg.contains_sym(target) || typ.contains_sym(target),
        }
    }

    pub fn replace(&self, from: &SymRc, to: &SymRc) -> Self {
        match self {
            Sol(sym_inner) => {
                if Rc::ptr_eq(sym_inner, from) {
                    Exp::sol(to)
                } else {
                    self.clone()
                }
            }
            App(fun_inner, arg_inner, typ_inner) => {
                let new_fun_inner = fun_inner.replace(from, to);
                let new_arg_inner = arg_inner.replace(from, to);
                let new_typ_inner = typ_inner.replace(from, to);
                App(Box::new(new_fun_inner), Box::new(new_arg_inner), Box::new(new_typ_inner))
            }
        }
    }
}

impl AlphaEq for Exp {
    fn alpha_eq(&self, other: &Self) -> bool {
        alpha_eq_exp(self, other)
    }
}

impl From<Sym> for Exp {
    fn from(sym: Sym) -> Self {
        Sol(sym.into())
    }
}

/// This impl should not clone the `sym` because it is passed by value (the caller loses ownership anyway).
impl From<SymRc> for Exp {
    fn from(sym: SymRc) -> Self {
        Sol(sym)
    }
}

impl From<&SymRc> for Exp {
    fn from(sym: &SymRc) -> Self {
        Self::sol(sym)
    }
}

impl<N> From<Def<N>> for Exp {
    fn from(def: Def<N>) -> Self {
        let (_, exp) = def.into();
        exp
    }
}

impl<N> From<&Def<N>> for Exp {
    fn from(def: &Def<N>) -> Self {
        def.exp().clone()
    }
}

impl<N> From<DefRc<N>> for Exp {
    fn from(def: DefRc<N>) -> Self {
        match Rc::try_unwrap(def) {
            Ok(def) => def.into(),
            Err(def) => def.as_ref().into(),
        }
    }
}

impl<N> From<&DefRc<N>> for Exp {
    fn from(def: &DefRc<N>) -> Self {
        def.as_ref().into()
    }
}

impl From<&Exp> for Exp {
    fn from(exp: &Exp) -> Self {
        exp.clone()
    }
}

impl From<&Binding> for Exp {
    fn from(binding: &Binding) -> Self {
        use Binding::*;
        match binding {
            Sym(sym) => sym.into(),
            Def(def) => def.into(),
        }
    }
}

impl<A: Into<Exp>> Of<A> for Exp {
    fn of(&self, arg: A) -> Result<Exp, InvalidApplicationError> {
        Exp::app(self.clone(), arg)
    }

    fn of_at(&self, _rhs: A, _index: usize) -> Result<Exp, InvalidApplicationError> {
        todo!()
    }

    fn of_smart(&self, _rhs: A) -> Option<Exp> {
        todo!()
    }
}

// impl TryFrom<(SymRc, SymRc)> for Exp {
//     type Error = InvalidApplicationError;
//
//     fn try_from((fun, arg): (SymRc, SymRc)) -> Result<Self, Self::Error> {
//         Self::app(fun, arg)
//     }
// }
//
// impl TryFrom<(SymRc, &SymRc)> for Exp {
//     type Error = InvalidApplicationError;
//
//     fn try_from((fun, arg): (SymRc, &SymRc)) -> Result<Self, Self::Error> {
//         Self::try_from((fun, arg.clone()))
//     }
// }
//
// impl TryFrom<(&SymRc, SymRc)> for Exp {
//     type Error = InvalidApplicationError;
//
//     fn try_from((fun, arg): (&SymRc, SymRc)) -> Result<Self, Self::Error> {
//         Self::try_from((fun.clone(), arg))
//     }
// }

/// Other `impl TryFrom` where the type argument contains a naked [`SymRc`] (without a reference) are commented out because they lead to hard-to-debug errors
/// It's quite rare anyway that a developer would like to pass a [`SymRc`] directly, and if they do, they can always use [`Exp::app`] instead of [`Exp::try_from`]
impl TryFrom<(&SymRc, &SymRc)> for Exp {
    type Error = InvalidApplicationError;

    fn try_from((fun, arg): (&SymRc, &SymRc)) -> Result<Self, Self::Error> {
        Self::app(fun.clone(), arg.clone())
    }
}

impl TryFrom<(Exp, &SymRc)> for Exp {
    type Error = InvalidApplicationError;

    fn try_from((fun, arg): (Exp, &SymRc)) -> Result<Self, Self::Error> {
        Self::app(fun, arg.clone())
    }
}

impl TryFrom<(&Exp, &SymRc)> for Exp {
    type Error = InvalidApplicationError;

    fn try_from((fun, arg): (&Exp, &SymRc)) -> Result<Self, Self::Error> {
        Self::app(fun.clone(), arg.clone())
    }
}

impl TryFrom<(&SymRc, Exp)> for Exp {
    type Error = InvalidApplicationError;

    fn try_from((fun, arg): (&SymRc, Exp)) -> Result<Self, Self::Error> {
        Self::app(fun.clone(), arg)
    }
}

impl TryFrom<(Exp, Exp)> for Exp {
    type Error = InvalidApplicationError;

    fn try_from((fun, arg): (Exp, Exp)) -> Result<Self, Self::Error> {
        Self::app(fun, arg)
    }
}

/// This macro uses `$sym.clone()` to avoid the "&" before symbols.
/// `$sym` should have a [`SymRc`] type.
/// `$sym.clone()` is also used in [`typ`](crate::typ) macro.
#[macro_export]
macro_rules! exp {
    ($sym: expr) => {
        $crate::Exp::from($sym.clone())
    };
    ($a: expr, $b: expr) => {
        // we can't add references to $a or $b (e.g. `&$a, &$b`) because $a might be an `Exp`, which should be passed directly, not by reference
        $crate::Exp::app($a, $b).expect("should pass the type check")
    };
    ($a: expr, $b: expr, $($rest: expr),+$(,)?) => {
        $crate::exp!($crate::exp!($a, $b), $($rest),+)
    };
}

#[cfg(test)]
mod tests {
    use crate::{AlphaEq, Bools, Lists, Nats, Of, Vectors, sym, typ};

    #[test]
    #[allow(unused_variables)]
    fn must_error_for_wrong_types() {
        let Bools {
            bool,
            yes,
            no,
        } = Bools::new();
        let Lists {
            list,
            nil,
            cons,
        } = Lists::new();
        let Nats {
            nat,
            zero,
            next,
            ..
        } = Nats::new();
        let list_bool = list.of(&bool).unwrap();
        let nil_bool = nil.of(&bool).unwrap();
        let cons_nat = cons.of(&nat).unwrap();
        // let cons_nat_zero = cons_nat.of(&zero).unwrap();
        // assert!(cons_nat_zero.of(&nil_bool).is_err())
    }

    #[test]
    fn must_partially_apply_add_next_to_two() {
        let Nats {
            nat,
            zero,
            next,
            add,
            add_next,
            ..
        } = Nats::new();

        let one = exp!(&next, &zero);
        let two = exp!(&next, one.clone());

        let actual = add_next.of(two.clone()).unwrap();

        sym!(b: typ!(exp!(nat)));
        let next_two = exp!(&next, two.clone());
        let add_next_two_b_exp = exp!(&add, next_two, &b);
        sym!(add_next_a_b: typ!(add_next_two_b_exp));
        let add_two_b_exp = exp!(&add, two.clone(), &b);
        let next_add_two_b_exp = exp!(&next, add_two_b_exp);
        sym!(expected: typ!(b => typ!(add_next_a_b => typ!(next_add_two_b_exp))));

        assert!(actual.typ().alpha_eq(expected.typ()));
    }

    #[test]
    fn must_partially_apply_append_cons_to_bool_one_two_yes() {
        let Bools {
            bool,
            yes,
            ..
        } = Bools::new();
        let nats = Nats::new();
        let Nats {
            zero,
            next,
            add,
            ..
        } = nats.clone();
        let Vectors {
            vector,
            cons,
            append,
            append_cons,
            ..
        } = Vectors::new(&nats);

        let one = exp!(&next, &zero);
        let two = exp!(&next, one.clone());

        let actual = append_cons
            .of(&bool)
            .unwrap()
            .of(one.clone())
            .unwrap()
            .of(two.clone())
            .unwrap()
            .of(&yes)
            .unwrap();

        let vector_bool = exp!(&vector, &bool);
        let vector_bool_one = exp!(vector_bool.clone(), one.clone());
        let vector_bool_two = exp!(vector_bool.clone(), two.clone());
        sym!(tail: typ!(vector_bool_one));
        sym!(b: typ!(vector_bool_two));
        let next_one = exp!(&next, one.clone());
        let cons_bool_one_yes_tail = exp!(&cons, &bool, one.clone(), &yes, &tail);
        let append_bool_next_one_two_cons_b_exp = exp!(&append, &bool, next_one, two.clone(), cons_bool_one_yes_tail, &b);
        sym!(append_t_next_len_a_len_b_cons_b: typ!(append_bool_next_one_two_cons_b_exp));
        let add_one_two = exp!(&add, one.clone(), two.clone());
        let append_bool_one_two_tail_b = exp!(&append, &bool, one, two, &tail, &b);
        let cons_bool_add_one_two_yes_append = exp!(&cons, &bool, add_one_two, &yes, append_bool_one_two_tail_b);
        sym!(expected: typ!(tail => typ!(b => typ!(append_t_next_len_a_len_b_cons_b => typ!(cons_bool_add_one_two_yes_append)))));

        assert!(actual.typ().alpha_eq(expected.typ()));
    }
}
