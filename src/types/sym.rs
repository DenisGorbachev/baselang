use crate::{Exp, InvalidApplicationError, Nym, Of, Typ};
use derive_getters::Getters;
use derive_new::new;
use std::rc::Rc;

/// This type should be wrapped in [`Rc`] because earlier symbols need to be referenced by later symbols
///
/// Compare symbols through [`SymRc`] identity with [`Rc::ptr_eq`].
#[derive(new, Getters, Clone, Debug)]
pub struct Sym<N = Nym, T = Typ> {
    /// Names of this sym.
    ///
    /// `Nym` is needed for printing (we need to print the names of outer syms that are referenced by [`Sym::typ`] while printing the current sym)
    #[new(into)]
    nym: N,

    /// Do we need to wrap the `typ` in [`Rc`]?
    /// * Yes: because multiple symbols can have the same type (e.g. `n : Nat`, `Zero : Nat`)
    /// * No: because we can wrap the symbols themselves in Rc and pass them into Exp
    ///   We can't make `typ` `pub` because that would allow mutating it after the [`Sym`] was used to construct an [`Exp::App`], which would break caching of [`Typ`] of the resulting expression
    #[new(into)]
    typ: T,
}

pub type SymRc<N = Nym, T = Typ> = Rc<Sym<N, T>>;

impl<N, T> Sym<N, T> {
    pub fn new_rc(nym: impl Into<N>, typ: impl Into<T>) -> Rc<Self> {
        Rc::new(Self::new(nym, typ))
    }

    pub fn set_nym(&mut self, nym: impl Into<N>) {
        self.nym = nym.into()
    }
}

impl<N> Sym<N, Typ> {
    pub fn new_top(nym: impl Into<N>) -> Self {
        Self::new(nym, Typ::Top)
    }

    pub fn new_top_rc(nym: impl Into<N>) -> Rc<Self> {
        Rc::new(Self::new_top(nym))
    }

    pub fn typ_last(&self) -> &Typ {
        self.typ.last()
    }
}

impl Sym {
    pub fn substitute(&self, sym: &SymRc, arg: &Exp) -> Self {
        let typ = self.typ.substitute(sym, arg);
        Self::new(self.nym.clone(), typ)
    }

    pub fn substitute_rc(self: &Rc<Self>, sym: &SymRc, arg: &Exp) -> Rc<Self> {
        if self.contains_sym(sym) { Rc::new(self.substitute(sym, arg)) } else { self.clone() }
    }

    pub fn replace_sym(&self, from: &SymRc, to: &SymRc) -> Self {
        let typ = self.typ.replace(from, to);
        Self::new(self.nym.clone(), typ)
    }

    pub fn replace_rc(self: &Rc<Self>, from: &SymRc, to: &SymRc) -> Rc<Self> {
        if Rc::ptr_eq(self, from) {
            to.clone()
        } else if self.contains_sym(from) {
            Rc::new(self.replace_sym(from, to))
        } else {
            self.clone()
        }
    }

    /// Returns `true` if this symbol's type mentions `target` by identity.
    pub fn contains_sym(&self, target: &SymRc) -> bool {
        self.typ.contains_sym(target)
    }

    pub fn of_at(&self, _arg: &SymRc) -> Result<Exp, InvalidApplicationError> {
        todo!()
    }

    /// Returns `true` if the current sym is a type family that is total in every arg except the last, and also unique in the last arg
    pub fn is_function(&self) -> bool {
        todo!()
    }
}

impl<N, T> From<(N, T)> for Sym<N, T> {
    fn from((nym, typ): (N, T)) -> Self {
        Self::new(nym, typ)
    }
}

impl<N, T> From<Sym<N, T>> for (N, T) {
    fn from(value: Sym<N, T>) -> Self {
        let Sym {
            nym,
            typ,
        } = value;
        (nym, typ)
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
}

#[macro_export]
macro_rules! sym {
    ($name:ident) => {
        let $name = $crate::Sym::<$crate::Nym, $crate::Typ>::new_top_rc(stringify!($name));
    };
    ($name: ident: $typ: expr) => {
        let $name = $crate::Sym::<$crate::Nym, $crate::Typ>::new_rc(stringify!($name), $typ);
    };
    ($name: ident: $typ: expr; $nym: expr) => {
        let $name = $crate::Sym::<$crate::Nym, $crate::Typ>::new_rc($nym, $typ);
    };
}
