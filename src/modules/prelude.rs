use crate::{Bool, List, Module, Nat};
use derive_getters::Getters;
use derive_more::{From, Into};
use derive_new::new;

/// The [`Default`] implementation for an aggregate module is already good enough
#[derive(new, Getters, From, Into, Ord, PartialOrd, Eq, PartialEq, Default, Hash, Clone, Debug)]
pub struct Prelude {
    pub bool: Bool,
    pub nat: Nat,
    pub list: List,
}

impl Prelude {
    pub fn print(&self) -> Vec<String> {
        let mut vec = vec![];
        vec.extend(self.bool.print());
        vec.extend(self.nat.print());
        vec.extend(self.list.print());
        vec
    }
}
