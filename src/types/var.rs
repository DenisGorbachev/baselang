use crate::types::typ::Typ;
use crate::{Exp, InvalidApplicationError, Nym, Of};
use derive_getters::Getters;
use derive_more::{From, Into};
use std::rc::Rc;

/// This type should be wrapped in [`Rc`] because earlier variables need to be referenced by later variables
#[derive(Getters, From, Into, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Debug)]
pub struct Var {
    /// This field is needed for printing (we need to print the names of outer vars that are referenced by [`Var::typ`] while printing the current var)
    nym: Nym,
    /// Do we need to wrap the `typ` in [`Rc`]?
    /// * Yes: because multiple variables can have the same type (e.g. `n : Nat`, `Zero : Nat`)
    /// * No: because we can wrap the variables themselves in Rc and pass them into Exp
    ///   We can't make `typ` `pub` because that would allow mutating it after the [`Var`] was used to construct an [`Exp::App`], which would break caching of [`Typ`] of the resulting expression
    typ: Typ,
}

pub type VarRc = Rc<Var>;

impl Var {
    pub fn new(nym: impl Into<Nym>, typ: impl Into<Typ>) -> Self {
        Self {
            nym: nym.into(),
            typ: typ.into(),
        }
    }

    pub fn new_top(nym: impl Into<Nym>) -> Self {
        Self {
            nym: nym.into(),
            typ: Typ::Top,
        }
    }

    pub fn new_rc(nym: impl Into<Nym>, typ: impl Into<Typ>) -> Rc<Self> {
        Rc::new(Self::new(nym, typ))
    }

    pub fn new_top_rc(nym: impl Into<Nym>) -> Rc<Self> {
        Rc::new(Self::new_top(nym))
    }

    pub fn set_nym(&mut self, nym: impl Into<Nym>) {
        self.nym = nym.into()
    }
}

pub trait ToVarRc {
    fn to_var_rc(self) -> VarRc;
}

impl ToVarRc for VarRc {
    fn to_var_rc(self) -> VarRc {
        self
    }
}

impl ToVarRc for &VarRc {
    fn to_var_rc(self) -> VarRc {
        self.clone()
    }
}

impl Of<VarRc> for VarRc {
    fn of(&self, arg: VarRc) -> Result<Exp, InvalidApplicationError> {
        Exp::app(self.clone(), arg)
    }
}

impl Of<&VarRc> for VarRc {
    fn of(&self, arg: &VarRc) -> Result<Exp, InvalidApplicationError> {
        Exp::app(self.clone(), arg.clone())
    }
}

impl Of<Exp> for VarRc {
    fn of(&self, arg: Exp) -> Result<Exp, InvalidApplicationError> {
        Exp::app(self.clone(), arg)
    }
}

#[macro_export]
macro_rules! vrc {
    ($name: expr) => {
        $crate::Var::new_top_rc($name)
    };
    ($name: expr, $typ: expr) => {
        $crate::Var::new_rc($name, $typ)
    };
}

#[macro_export]
macro_rules! var {
    ($name:ident) => {
        let $name = $crate::Var::new_top_rc(stringify!($name));
    };
    ($name: ident: $typ: expr) => {
        let $name = $crate::Var::new_rc(stringify!($name), $typ);
    };
    ($name: ident: $typ: expr; $nym: expr) => {
        let $name = $crate::Var::new_rc($nym, $typ);
    };
}
