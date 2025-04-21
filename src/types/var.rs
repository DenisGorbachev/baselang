use crate::types::typ::Typ;
use crate::{Exp, InvalidApplicationError, Of};
use derive_getters::Getters;
use derive_more::{From, Into};
use std::rc::Rc;

#[derive(Getters, From, Into, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Debug)]
pub struct Var {
    /// This field is needed for printing (we need to print the names of outer vars that are referenced by [`Var::typ`] to print them while the current var)
    name: String,
    /// Do we need to wrap the `typ` in [`Rc`]?
    /// * Yes: because multiple variables can have the same type (e.g. `n : Nat`, `Zero : Nat`)
    /// * No: because we can wrap the variables themselves in Rc and pass them into Exp
    typ: Typ,
}

pub type VarRc = Rc<Var>;

impl Var {
    pub fn new(name: impl Into<String>, typ: impl Into<Typ>) -> Self {
        Self {
            name: name.into(),
            typ: typ.into(),
        }
    }

    pub fn new_top(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            typ: Typ::Top,
        }
    }

    pub fn new_rc(name: impl Into<String>, typ: impl Into<Typ>) -> Rc<Self> {
        Rc::new(Self::new(name, typ))
    }

    pub fn new_top_rc(name: impl Into<String>) -> Rc<Self> {
        Rc::new(Self::new_top(name))
    }

    pub fn print(&self, _name: &str) -> String {
        todo!()
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
}
