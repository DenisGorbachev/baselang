use crate::{Constructors, Exp, InvalidApplicationError, Nym, Of, Typ};
use derive_getters::Getters;
use std::rc::Rc;

/// This type should be wrapped in [`Rc`] because earlier symbols need to be referenced by later symbols
///
/// [`Sym`] must not derive [`Eq`], [`PartialEq`], [`Hash`] because every symbol is unique: it is equal only to itself. Compare symbols through [`SymRc`] identity with [`Rc::ptr_eq`].
#[derive(Getters, Clone, Debug)]
pub struct Sym {
    /// Names of this sym.
    ///
    /// `Nym` is needed for printing (we need to print the names of outer syms that are referenced by [`Sym::typ`] while printing the current sym)
    nym: Nym,

    /// Do we need to wrap the `typ` in [`Rc`]?
    /// * Yes: because multiple symbols can have the same type (e.g. `n : Nat`, `Zero : Nat`)
    /// * No: because we can wrap the symbols themselves in Rc and pass them into Exp
    ///   We can't make `typ` `pub` because that would allow mutating it after the [`Sym`] was used to construct an [`Exp::App`], which would break caching of [`Typ`] of the resulting expression
    typ: Typ,

    /// Constructors of this sym.
    ///
    /// `None` means that sym doesn't need constructors (we assume that it has a proof)
    /// `Some(vec![])` means that sym has no constructors (we know that it doesn't have a proof) (example: `Void` aka `False` type)
    constructors: Constructors,
}

pub type SymRc = Rc<Sym>;

impl Sym {
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

    pub fn typ_last(&self) -> &Typ {
        self.typ.last()
    }

    pub fn substitute(&self, sym: &SymRc, arg: &Exp) -> Self {
        let typ = self.typ.substitute(sym, arg);
        let constructors = self.constructors.as_ref().map(|constructors| {
            constructors
                .iter()
                .map(|constructor| constructor.substitute(sym, arg))
                .collect::<Vec<_>>()
        });
        Self::new(self.nym.clone(), typ, constructors)
    }

    pub fn replace_sym(&self, from: &SymRc, to: &SymRc) -> Self {
        let typ = self.typ.replace(from, to);
        let constructors = self.constructors.as_ref().map(|constructors| {
            constructors
                .iter()
                .map(|constructor| constructor.replace_sym(from, to))
                .collect::<Vec<_>>()
        });
        Self::new(self.nym.clone(), typ, constructors)
    }

    /// Returns `true` if this symbol's type or constructors mention `target` by identity.
    pub fn contains_sym(&self, target: &SymRc) -> bool {
        self.typ.contains_sym(target)
            || self.constructors.as_ref().is_some_and(|constructors| {
                constructors
                    .iter()
                    .any(|constructor| constructor.contains_sym(target))
            })
    }

    pub fn of_at(&self, _arg: &SymRc) -> Result<Exp, InvalidApplicationError> {
        todo!()
    }

    /// Returns `true` if the current sym is a type family that is total in every arg except the last, and also unique in the last arg
    pub fn is_function(&self) -> bool {
        todo!()
    }
}

impl From<(Nym, Typ)> for Sym {
    fn from((nym, typ): (Nym, Typ)) -> Self {
        Self::new(nym, typ, None)
    }
}

impl From<Sym> for (Nym, Typ, Constructors) {
    fn from(value: Sym) -> Self {
        let Sym {
            nym,
            typ,
            constructors,
        } = value;
        (nym, typ, constructors)
    }
}

pub trait ToSymRc {
    fn to_sym_rc(self) -> SymRc;
}

impl ToSymRc for SymRc {
    fn to_sym_rc(self) -> SymRc {
        self
    }
}

impl ToSymRc for &SymRc {
    fn to_sym_rc(self) -> SymRc {
        self.clone()
    }
}

impl<A: Into<Exp>> Of<A> for SymRc {
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

#[macro_export]
macro_rules! sym {
    ($name:ident) => {
        let $name = $crate::Sym::new_top_rc(stringify!($name), None);
    };
    ($name: ident: $typ: expr) => {
        let $name = $crate::Sym::new_rc(stringify!($name), $typ, None);
    };
    ($name: ident: $typ: expr; $nym: expr) => {
        let $name = $crate::Sym::new_rc($nym, $typ, None);
    };
}
