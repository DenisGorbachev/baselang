use crate::{DefRc, Nym, SymRc};
use Binding::*;
use derive_more::From;
use std::rc::Rc;

#[derive(From, Clone, Debug)]
pub enum Binding {
    Sym(SymRc),
    Def(DefRc),
}

pub type BindingRc = Rc<Binding>;

impl Binding {
    pub fn nym(&self) -> &Nym {
        match self {
            Sym(sym) => sym.nym(),
            Def(def) => def.nym(),
        }
    }
}
