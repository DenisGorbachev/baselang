use crate::{Bool, List, Module, Nat, VarRc};
use derive_getters::Getters;
use derive_more::{From, Into};
use derive_new::new;

#[derive(new, Getters, From, Into, Ord, PartialOrd, Eq, PartialEq, Default, Hash, Clone, Debug)]
pub struct TestPrelude {
    pub bool: Bool,
    pub nat: Nat,
    pub list: List,
}

impl TestPrelude {
    pub fn bool_list(&self) -> ((&VarRc, &VarRc, &VarRc), (&VarRc, &VarRc, &VarRc)) {
        (self.bool.refs_tuple(), self.list.refs_tuple())
    }
}
