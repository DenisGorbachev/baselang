use crate::types::typ::Typ;
use crate::{Exp, InvalidApplicationError, Nym, Of};
use derive_getters::Getters;
use std::rc::Rc;

/// This type should be wrapped in [`Rc`] because earlier variables need to be referenced by later variables
///
/// [`Var`] must not derive [`Eq`], [`PartialEq`], [`Hash`] because every variable is unique: it is equal only to itself. Compare variables through [`VarRc`] identity with [`Rc::ptr_eq`].
#[derive(Getters, Clone, Debug)]
pub struct Var {
    /// Names of this var.
    ///
    /// `Some(Nym)` is needed for printing named vars.
    /// `None` means that the var is anonymous.
    nym: Option<Nym>,

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
            nym: Some(nym.into()),
            typ: typ.into(),
        }
    }

    pub fn new_top(nym: impl Into<Nym>) -> Self {
        Self {
            nym: Some(nym.into()),
            typ: Typ::Top,
        }
    }

    pub fn new_rc(nym: impl Into<Nym>, typ: impl Into<Typ>) -> Rc<Self> {
        Rc::new(Self::new(nym, typ))
    }

    pub fn new_top_rc(nym: impl Into<Nym>) -> Rc<Self> {
        Rc::new(Self::new_top(nym))
    }

    pub fn new_anon_rc(typ: impl Into<Typ>) -> Rc<Self> {
        Rc::new(Self {
            nym: None,
            typ: typ.into(),
        })
    }

    pub fn set_nym(&mut self, nym: impl Into<Nym>) {
        self.nym = Some(nym.into())
    }

    pub fn typ_last(&self) -> &Typ {
        self.typ.last()
    }

    pub fn is_anon(&self) -> bool {
        self.nym.is_none()
    }

    pub fn substitute(&self, var: &VarRc, arg: &Exp) -> Self {
        let typ = self.typ.substitute(var, arg);
        Self {
            nym: self.nym.clone(),
            typ,
        }
    }

    pub fn replace_var(&self, from: &VarRc, to: &VarRc) -> Self {
        let typ = self.typ.replace(from, to);
        Self {
            nym: self.nym.clone(),
            typ,
        }
    }

    /// Returns `true` if this variable's type mentions `target` by identity.
    pub fn contains_var(&self, target: &VarRc) -> bool {
        self.typ.contains_var(target)
    }

    pub fn of_at(&self, _arg: &VarRc) -> Result<Exp, InvalidApplicationError> {
        todo!()
    }

    /// Returns `true` if the current var is a type family that is total in every arg except the last, and also unique in the last arg
    pub fn is_function(&self) -> bool {
        todo!()
    }
}

impl From<(Nym, Typ)> for Var {
    fn from((nym, typ): (Nym, Typ)) -> Self {
        Self::new(nym, typ)
    }
}

impl From<Var> for (Option<Nym>, Typ) {
    fn from(value: Var) -> Self {
        let Var {
            nym,
            typ,
        } = value;
        (nym, typ)
    }
}

impl Of<VarRc> for VarRc {
    fn of(&self, arg: VarRc) -> Result<Exp, InvalidApplicationError> {
        Exp::app(self.clone(), arg)
    }

    fn of_at(&self, _rhs: VarRc, _index: usize) -> Result<Exp, InvalidApplicationError> {
        todo!()
    }

    fn of_smart(&self, _rhs: VarRc) -> Option<Exp> {
        todo!()
    }
}

impl Of<&VarRc> for VarRc {
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

impl Of<Exp> for VarRc {
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
