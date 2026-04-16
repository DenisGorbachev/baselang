use crate::{Fun, InvalidApplicationError, Of, One, Top, TypBox, Var};
use crate::{Typ, VarRc};
use std::rc::Rc;

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub enum Exp {
    /// [`Sol`] means `solo` (a single variable)
    /// Must wrap [`Var`] in [`Rc`] because a single var can be used in multiple exps (e.g. `Nat` can be used in multiple exps)
    /// Name comes from "solo" (needed a three-letter term that doesn't conflict with other terms)
    Sol(VarRc),
    /// [`App`] means `application` (of one expression on another expression)
    /// IMPORTANT: Never construct this variant directly, call [`Exp::app`] instead (which performs the type check)
    /// Must wrap [`Exp`] in [`Box`] to avoid "recursive type" error
    /// [`TypBox`] is a cached type of this expression (calculated in [`Exp::app`])
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
            (Top, arg_typ) => Err(InvalidApplicationError::new(Top, arg_typ)),
            (One(exp), arg_typ) => Err(InvalidApplicationError::new(One(exp), arg_typ)),
            (Fun(var, typ_old), arg_typ) => {
                // Structural equality is correct here because application checks type compatibility, not whether the types come from the same `Rc`
                if *var.typ() == arg_typ {
                    let typ_new = Box::new(typ_old.substitute(&var, &arg));
                    Ok(App(Box::new(fun), Box::new(arg), typ_new))
                } else {
                    Err(InvalidApplicationError::new(Fun(var, typ_old), arg_typ))
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

    /// In `self` expression, substitute variable `var` with expression `arg`
    ///
    /// The caller must ensure that `var` and `arg` have the same type.
    pub fn substitute(&self, var: &VarRc, arg: &Exp) -> Self {
        debug_assert_eq!(var.typ(), arg.typ());
        match self {
            Sol(var_inner) => {
                if Rc::ptr_eq(var_inner, var) {
                    arg.clone()
                } else {
                    self.clone()
                }
            }
            App(fun_inner, arg_inner, typ_inner) => {
                let new_fun_inner = fun_inner.substitute(var, arg);
                let new_arg_inner = arg_inner.substitute(var, arg);
                let new_typ_inner = typ_inner.substitute(var, arg);
                App(Box::new(new_fun_inner), Box::new(new_arg_inner), Box::new(new_typ_inner))
            }
        }
    }

    pub fn contains_var(&self, target: &VarRc) -> bool {
        match self {
            Sol(var) => Rc::ptr_eq(var, target),
            App(fun, arg, typ) => fun.contains_var(target) || arg.contains_var(target) || typ.contains_var(target),
        }
    }

    pub fn replace(&self, from: &VarRc, to: &VarRc) -> Self {
        match self {
            Sol(var_inner) => {
                if Rc::ptr_eq(var_inner, from) {
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

impl From<Var> for Exp {
    fn from(var: Var) -> Self {
        Sol(var.into())
    }
}

/// This impl should not clone the `var` because it is passed by value (the caller loses ownership anyway).
impl From<VarRc> for Exp {
    fn from(var: VarRc) -> Self {
        Sol(var)
    }
}

impl From<&VarRc> for Exp {
    fn from(var: &VarRc) -> Self {
        Self::sol(var)
    }
}

impl Of<&VarRc> for Exp {
    fn of(&self, arg: &VarRc) -> Result<Exp, InvalidApplicationError> {
        Exp::app(self.clone(), arg.clone())
    }

    fn of_at(&self, _rhs: &VarRc, _index: usize) -> Result<Exp, InvalidApplicationError> {
        todo!()
    }

    fn of_smart(&self, _rhs: &VarRc) -> Option<Exp> {
        todo!()
    }
}

impl Of<Exp> for Exp {
    fn of(&self, arg: Exp) -> Result<Exp, InvalidApplicationError> {
        Exp::app(self.clone(), arg)
    }

    fn of_at(&self, _rhs: Exp, _index: usize) -> Result<Exp, InvalidApplicationError> {
        todo!()
    }

    fn of_smart(&self, _rhs: Exp) -> Option<Exp> {
        todo!()
    }
}

impl Of<&Exp> for Exp {
    fn of(&self, arg: &Exp) -> Result<Exp, InvalidApplicationError> {
        Exp::app(self.clone(), arg.clone())
    }

    fn of_at(&self, _rhs: &Exp, _index: usize) -> Result<Exp, InvalidApplicationError> {
        todo!()
    }

    fn of_smart(&self, _rhs: &Exp) -> Option<Exp> {
        todo!()
    }
}

// impl TryFrom<(VarRc, VarRc)> for Exp {
//     type Error = InvalidApplicationError;
//
//     fn try_from((fun, arg): (VarRc, VarRc)) -> Result<Self, Self::Error> {
//         Self::app(fun, arg)
//     }
// }
//
// impl TryFrom<(VarRc, &VarRc)> for Exp {
//     type Error = InvalidApplicationError;
//
//     fn try_from((fun, arg): (VarRc, &VarRc)) -> Result<Self, Self::Error> {
//         Self::try_from((fun, arg.clone()))
//     }
// }
//
// impl TryFrom<(&VarRc, VarRc)> for Exp {
//     type Error = InvalidApplicationError;
//
//     fn try_from((fun, arg): (&VarRc, VarRc)) -> Result<Self, Self::Error> {
//         Self::try_from((fun.clone(), arg))
//     }
// }

/// Other `impl TryFrom` where the type argument contains a naked [`VarRc`] (without a reference) are commented out because they lead to hard-to-debug errors
/// It's quite rare anyway that a developer would like to pass a [`VarRc`] directly, and if they do, they can always use [`Exp::app`] instead of [`Exp::try_from`]
impl TryFrom<(&VarRc, &VarRc)> for Exp {
    type Error = InvalidApplicationError;

    fn try_from((fun, arg): (&VarRc, &VarRc)) -> Result<Self, Self::Error> {
        Self::app(fun.clone(), arg.clone())
    }
}

impl TryFrom<(Exp, &VarRc)> for Exp {
    type Error = InvalidApplicationError;

    fn try_from((fun, arg): (Exp, &VarRc)) -> Result<Self, Self::Error> {
        Self::app(fun, arg.clone())
    }
}

impl TryFrom<(&VarRc, Exp)> for Exp {
    type Error = InvalidApplicationError;

    fn try_from((fun, arg): (&VarRc, Exp)) -> Result<Self, Self::Error> {
        Self::app(fun.clone(), arg)
    }
}

impl TryFrom<(Exp, Exp)> for Exp {
    type Error = InvalidApplicationError;

    fn try_from((fun, arg): (Exp, Exp)) -> Result<Self, Self::Error> {
        Self::app(fun, arg)
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
        // we can't add references to $a or $b (e.g. `&$a, &$b`) because $a might be an `Exp`, which should be passed directly, not by reference
        $crate::Exp::try_from(($a, $b)).expect("should pass the type check")
    };
    ($a: expr, $b: expr, $($rest: expr),+) => {
        $crate::exp!($crate::exp!($a, $b), $($rest),+)
    };
}

#[cfg(test)]
mod tests {
    use crate::{Bools, Lists, Nats, Of, Vectors, typ, var};
    use pretty_assertions::assert_eq;

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

        var!(b: typ!(exp!(nat)));
        let next_two = exp!(&next, two.clone());
        let add_next_two_b_exp = exp!(&add, next_two, &b);
        var!(add_next_a_b: typ!(add_next_two_b_exp));
        let add_two_b_exp = exp!(&add, two.clone(), &b);
        let next_add_two_b_exp = exp!(&next, add_two_b_exp);
        var!(expected: typ!(b => typ!(add_next_a_b => typ!(next_add_two_b_exp))));

        assert_eq!(actual.typ(), expected.typ());
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
        var!(tail: typ!(vector_bool_one));
        var!(b: typ!(vector_bool_two));
        let next_one = exp!(&next, one.clone());
        let cons_bool_one_yes_tail = exp!(&cons, &bool, one.clone(), &yes, &tail);
        let append_bool_next_one_two_cons_b_exp = exp!(&append, &bool, next_one, two.clone(), cons_bool_one_yes_tail, &b);
        var!(append_t_next_len_a_len_b_cons_b: typ!(append_bool_next_one_two_cons_b_exp));
        let add_one_two = exp!(&add, one.clone(), two.clone());
        let append_bool_one_two_tail_b = exp!(&append, &bool, one, two, &tail, &b);
        let cons_bool_add_one_two_yes_append = exp!(&cons, &bool, add_one_two, &yes, append_bool_one_two_tail_b);
        var!(expected: typ!(tail => typ!(b => typ!(append_t_next_len_a_len_b_cons_b => typ!(cons_bool_add_one_two_yes_append)))));

        assert_eq!(actual.typ(), expected.typ());
    }
}
