use crate::types::typ::Typ;
use crate::{Exp, InvalidApplicationError, Of};
use derive_getters::Getters;
use derive_more::{From, Into};
use std::rc::Rc;

/// This type should be wrapped in [`Rc`] because earlier variables need to be referenced by later variables
#[derive(Getters, From, Into, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Debug)]
pub struct Var {
    /// This field is needed for printing (we need to print the names of outer vars that are referenced by [`Var::typ`] to print them while the current var)
    /// TODO: inner vars names must not shadow outer vars
    /// TODO: The name "Top" is reserved
    // nym: NymRc,
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

    #[inline(always)]
    pub fn print(&self) -> String {
        let name = self.print_name();
        let typ = self.print_typ();
        format!("{name} : {typ}")
    }

    /// This function should be called when printing inner variables (they require parentheses)
    pub fn print_inner(&self, _is_top_level: bool, with_type: bool) -> String {
        let name = self.print_name();
        if with_type {
            let typ = self.print_typ();
            format!("({name} : {typ})")
        } else {
            name.to_string()
        }
    }

    #[inline(always)]
    pub fn print_name(&self) -> &str {
        &self.name
    }

    #[inline(always)]
    pub fn print_typ(&self) -> String {
        self.typ.print()
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
