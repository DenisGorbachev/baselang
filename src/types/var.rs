use crate::types::typ::Typ;
use crate::{Constructors, Exp, InvalidApplicationError, Nym, Of};
use derive_getters::Getters;
use std::rc::Rc;

/// This type should be wrapped in [`Rc`] because earlier variables need to be referenced by later variables
///
/// [`Var`] must not derive [`Eq`], [`PartialEq`], [`Hash`] because every variable is unique: it is equal only to itself. Compare variables through [`VarRc`] identity with [`Rc::ptr_eq`].
#[derive(Getters, Clone, Debug)]
pub struct Var {
    /// Names of this var.
    ///
    /// `Nym` is needed for printing (we need to print the names of outer vars that are referenced by [`Var::typ`] while printing the current var)
    nym: Nym,

    /// Do we need to wrap the `typ` in [`Rc`]?
    /// * Yes: because multiple variables can have the same type (e.g. `n : Nat`, `Zero : Nat`)
    /// * No: because we can wrap the variables themselves in Rc and pass them into Exp
    ///   We can't make `typ` `pub` because that would allow mutating it after the [`Var`] was used to construct an [`Exp::App`], which would break caching of [`Typ`] of the resulting expression
    typ: Typ,

    /// Constructors of this var.
    ///
    /// `None` means that var doesn't need constructors (we assume that it has a proof)
    /// `Some(vec![])` means that var has no constructors (we know that it doesn't have a proof) (example: `Void` aka `False` type)
    constructors: Constructors,
}

pub type VarRc = Rc<Var>;

impl Var {
    pub fn new(nym: impl Into<Nym>, typ: impl Into<Typ>, constructors: impl Into<Constructors>) -> Self {
        Self {
            nym: nym.into(),
            typ: typ.into(),
            constructors: constructors.into(),
        }
    }

    pub fn new_top(nym: impl Into<Nym>, constructors: impl Into<Constructors>) -> Self {
        Self {
            nym: nym.into(),
            typ: Typ::Top,
            constructors: constructors.into(),
        }
    }

    pub fn new_rc(nym: impl Into<Nym>, typ: impl Into<Typ>, constructors: impl Into<Constructors>) -> Rc<Self> {
        Rc::new(Self::new(nym, typ, constructors))
    }

    pub fn new_top_rc(nym: impl Into<Nym>, constructors: impl Into<Constructors>) -> Rc<Self> {
        Rc::new(Self::new_top(nym, constructors))
    }

    pub fn set_nym(&mut self, nym: impl Into<Nym>) {
        self.nym = nym.into()
    }

    pub fn set_constructors(&mut self, constructors: impl Into<Option<Vec<Self>>>) {
        self.constructors = constructors.into();
    }

    pub fn substitute(&self, var: &VarRc, arg: &Exp) -> Self {
        let typ = self.typ.substitute(var, arg);
        let constructors = self.constructors.as_ref().map(|constructors| {
            constructors
                .iter()
                .map(|constructor| constructor.substitute(var, arg))
                .collect::<Vec<_>>()
        });
        Self::new(self.nym.clone(), typ, constructors)
    }

    pub fn replace_var(&self, from: &VarRc, to: &VarRc) -> Self {
        let typ = self.typ.replace(from, to);
        let constructors = self.constructors.as_ref().map(|constructors| {
            constructors
                .iter()
                .map(|constructor| constructor.replace_var(from, to))
                .collect::<Vec<_>>()
        });
        Self::new(self.nym.clone(), typ, constructors)
    }

    /// Returns `true` if this variable's type or constructors mention `target` by identity.
    pub fn contains_var(&self, target: &VarRc) -> bool {
        self.typ.contains_var(target)
            || self.constructors.as_ref().is_some_and(|constructors| {
                constructors
                    .iter()
                    .any(|constructor| constructor.contains_var(target))
            })
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
        Self::new(nym, typ, None)
    }
}

impl From<Var> for (Nym, Typ, Constructors) {
    fn from(value: Var) -> Self {
        let Var {
            nym,
            typ,
            constructors,
        } = value;
        (nym, typ, constructors)
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
        let $name = $crate::Var::new_top_rc(stringify!($name), None);
    };
    ($name: ident: $typ: expr) => {
        let $name = $crate::Var::new_rc(stringify!($name), $typ, None);
    };
    ($name: ident: $typ: expr; $nym: expr) => {
        let $name = $crate::Var::new_rc($nym, $typ, None);
    };
}
