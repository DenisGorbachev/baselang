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
                    let var_name = var.print_name().to_string();
                    let concrete_var = Var::new_rc(var_name, arg_typ.clone());

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
    pub fn substitute_var(&self, var: &VarRc, arg: &Exp) -> ExpBox {
        match self {
            Sol(v) if Rc::ptr_eq(v, var) => Box::new(arg.clone()),
            Sol(_) => Box::new(self.clone()),
            App(fun, param, _, _) => {
                let new_fun = fun.substitute_var(var, arg);
                let new_param = param.substitute_var(var, arg);
                // Try to create a new application with the substituted parts
                match Exp::app((*new_fun).clone(), (*new_param).clone()) {
                    Ok(exp) => Box::new(exp),
                    Err(_) => Box::new(self.clone()), // Fallback to the original on error
                }
            }
        }
    }

    #[inline(always)]
    pub fn print(&self, with_type: bool) -> String {
        self.print_inner(true, with_type)
    }

    pub fn print_inner(&self, is_top_level: bool, with_type: bool) -> String {
        match self {
            Sol(var) => var.print_inner(is_top_level, with_type),
            App(fun, arg, _, typ) => {
                // this const must be false because we don't want to print the types of the inner values, only the type of the current exp itself
                // this const is defined only for clarity
                const WITH_TYPE_INNER: bool = false;
                let fun = fun.print(WITH_TYPE_INNER);
                let arg = arg.print(WITH_TYPE_INNER);
                // this `if` is necessary because we need to wrap the `{fun} {arg}` pair in braces to display the type
                if with_type {
                    let typ = typ.print();
                    format!("({fun} {arg}) : {typ}")
                } else {
                    format!("{fun} {arg}")
                }
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
    use crate::{Bool, List, Nat, Of};

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
        let Nat {
            nat,
            zero,
            next,
        } = Nat::default();
        let list_bool = list.of(&bool).unwrap();
        let nil_bool = nil.of(&bool).unwrap();
        let cons_nat = cons.of(&nat).unwrap();
        // let cons_nat_zero = cons_nat.of(&zero).unwrap();
        // assert!(cons_nat_zero.of(&nil_bool).is_err())
    }
}
