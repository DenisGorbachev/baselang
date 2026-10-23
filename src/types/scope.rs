use crate::{Binding, Constructors, Def, DefRc, Exp, Nym, Sym, SymRc, Typ};
use derive_getters::Getters;
use errgonomic::{handle, handle_bool};
use std::collections::HashMap;
use std::ptr::NonNull;
use std::rc::Rc;
use thiserror::Error;

#[derive(Default, Getters, Clone, Debug)]
pub struct Scope<B = HashMap<String, Binding>, C = Constructors> {
    bindings: B,
    constructors: C,
}

pub type ScopeRc<B = HashMap<String, Binding>, C = Constructors> = Rc<Scope<B, C>>;

impl Scope {
    pub fn new_sym(&mut self, nym: impl Into<Nym>, typ: impl Into<Typ>) -> Result<SymRc, ScopeNewSymError> {
        use ScopeNewSymError::*;
        let sym = Sym::new_rc(nym, typ);
        let sym = handle!(self.insert_sym(sym), InsertSymFailed);
        Ok(sym)
    }

    pub fn insert_sym(&mut self, sym: SymRc) -> Result<SymRc, ScopeInsertSymError> {
        use ScopeInsertSymError::*;
        handle_bool!(self.bindings.contains_key(sym.nym().identifier()), NymInvalid, sym);
        if let Some(head) = sym.typ().last().head() {
            self.constructors
                .entry(NonNull::from(head.as_ref()))
                .or_default()
                .push(sym.clone());
        }
        self.bindings
            .insert(sym.nym().identifier().to_owned(), Binding::Sym(sym.clone()));
        Ok(sym)
    }

    pub fn new_def(&mut self, nym: impl Into<Nym>, exp: impl Into<Exp>) -> Result<DefRc, ScopeNewDefError> {
        use ScopeNewDefError::*;
        let def = Rc::new(Def::new(nym, exp));
        let def = handle!(self.insert_def(def), InsertDefFailed);
        Ok(def)
    }

    pub fn insert_def(&mut self, def: DefRc) -> Result<DefRc, ScopeInsertDefError> {
        use ScopeInsertDefError::*;
        handle_bool!(self.bindings.contains_key(def.nym().identifier()), NymInvalid, def);
        self.bindings
            .insert(def.nym().identifier().to_owned(), Binding::Def(def.clone()));
        Ok(def)
    }

    pub fn sym(&self, name: &str) -> Option<&SymRc> {
        use Binding::*;
        match self.bindings.get(name) {
            Some(Sym(sym)) => Some(sym),
            _ => None,
        }
    }

    pub fn def(&self, name: &str) -> Option<&DefRc> {
        use Binding::*;
        match self.bindings.get(name) {
            Some(Def(def)) => Some(def),
            _ => None,
        }
    }

    pub fn constructors_of(&self, sym: &SymRc) -> &[SymRc] {
        self.constructors
            .get(&NonNull::from(sym.as_ref()))
            .map_or(&[], Vec::as_slice)
    }
}

#[derive(Error, Debug)]
pub enum ScopeNewSymError {
    #[error("failed to register the new symbol")]
    InsertSymFailed { source: ScopeInsertSymError },
}

#[derive(Error, Debug)]
pub enum ScopeInsertSymError {
    #[error("name '{name}' is already bound in this scope", name = sym.nym().identifier())]
    NymInvalid { sym: SymRc },
}

#[derive(Error, Debug)]
pub enum ScopeNewDefError {
    #[error("failed to register the new definition")]
    InsertDefFailed { source: ScopeInsertDefError },
}

#[derive(Error, Debug)]
pub enum ScopeInsertDefError {
    #[error("name '{name}' is already bound in this scope", name = def.nym().identifier())]
    NymInvalid { def: DefRc },
}
