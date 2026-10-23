use crate::{Exp, Scope, Substitution, Sym, SymRc};
use derive_getters::Getters;
use derive_new::new;
use std::collections::HashSet;
use std::ptr::NonNull;
use std::rc::Rc;

#[derive(new, Getters, Clone, Debug)]
pub struct Rewrite<P = Vec<Exp>, E = Exp> {
    pub patterns: P,
    pub body: E,
}

pub type RewriteRc<P = Vec<Exp>, E = Exp> = Rc<Rewrite<P, E>>;

impl Rewrite {
    pub fn variables<'a>(&'a self, scope: &'a Scope) -> impl Iterator<Item = &'a SymRc> {
        let mut seen = HashSet::new();
        self.patterns
            .iter()
            .flat_map(Exp::symbols)
            .filter(move |sym| !scope.is_registered(sym) && seen.insert(NonNull::from(sym.as_ref())))
    }

    pub fn freshened(&self, scope: &Scope) -> Self {
        let substitution = self
            .variables(scope)
            .fold(Substitution::new(), |mut substitution, sym| {
                let fresh = Sym::new_rc(sym.nym().clone(), sym.typ().instantiate(&substitution));
                substitution.insert(NonNull::from(sym.as_ref()), Exp::sol(&fresh));
                substitution
            });
        Self::new(
            self.patterns
                .iter()
                .map(|exp| exp.instantiate(&substitution))
                .collect(),
            self.body.instantiate(&substitution),
        )
    }
}
