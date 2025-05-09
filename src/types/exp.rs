use crate::{Fun, InvalidApplicationError, Of, One, Top, TypBox, Var};
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
    /// [`VarRc`] is a cached var of the fun of this expression (calculated in [`Exp::app`]) (might be unnecessary; let's see)
    /// [`TypBox`] is a cached type of this expression (calculated in [`Exp::app`])
    /// TODO: Remove [`VarRc`] if it turns out to be unnecessary for printing
    App(ExpBox, ExpBox, VarRc, TypBox),
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
                if *var.typ() == arg_typ {
                    // Create a new variable with the concrete type for later printing
                    // This ensures that when we print the type, the parameter uses the concrete type
                    // TODO: Optimize this code
                    let var_name = var.name().to_string();
                    let concrete_var = Var::new_rc(var_name, var.typ().clone());

                    // Substitute the var with arg in the type
                    let typ_new = Box::new(typ_old.substitute(&var, &arg));
                    Ok(App(Box::new(fun), Box::new(arg), concrete_var, typ_new))
                } else {
                    Err(InvalidApplicationError::new(Fun(var, typ_old), arg_typ))
                }
            }
        }
    }

    pub fn typ(&self) -> &Typ {
        match self {
            Sol(var) => var.typ(),
            App(_, _, _, typ) => typ,
        }
    }

    /// Substitute variable `var` with expression `arg` in this expression
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
            App(fun_inner, arg_inner, var_inner, typ_inner) => {
                let new_fun_inner = fun_inner.substitute(var, arg);
                let new_arg_inner = arg_inner.substitute(var, arg);
                debug_assert_eq!(new_fun_inner.typ(), fun_inner.typ());
                debug_assert_eq!(new_arg_inner.typ(), arg_inner.typ());
                App(Box::new(new_fun_inner), Box::new(new_arg_inner), var_inner.clone(), typ_inner.clone())
            }
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

impl From<&VarRc> for Exp {
    fn from(var: &VarRc) -> Self {
        Self::sol(var)
    }
}

impl Of<&VarRc> for Exp {
    fn of(&self, arg: &VarRc) -> Result<Exp, InvalidApplicationError> {
        Exp::app(self.clone(), arg.clone())
    }
}

impl Of<Exp> for Exp {
    fn of(&self, arg: Exp) -> Result<Exp, InvalidApplicationError> {
        Exp::app(self.clone(), arg)
    }
}

impl Of<&Exp> for Exp {
    fn of(&self, arg: &Exp) -> Result<Exp, InvalidApplicationError> {
        Exp::app(self.clone(), arg.clone())
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
    use crate::{Bool, List, Nat, Of};

    #[test]
    #[allow(unused_variables)]
    fn must_error_for_wrong_types() {
        let Bool {
            bool,
            yes,
            no,
        } = Bool::new();
        let List {
            list,
            nil,
            cons,
        } = List::new();
        let Nat {
            nat,
            zero,
            next,
        } = Nat::new();
        let list_bool = list.of(&bool).unwrap();
        let nil_bool = nil.of(&bool).unwrap();
        let cons_nat = cons.of(&nat).unwrap();
        // let cons_nat_zero = cons_nat.of(&zero).unwrap();
        // assert!(cons_nat_zero.of(&nil_bool).is_err())
    }
}
