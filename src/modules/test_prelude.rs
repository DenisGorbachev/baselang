use crate::{Bool, BoolTuple, List, ListTuple, Nat, NatTuple, impl_vars_vec_aggregate};
use derive_more::{From, Into};

#[derive(From, Into, Eq, PartialEq, Default, Hash, Clone, Debug)]
pub struct TestPrelude {
    pub bool: Bool,
    pub nat: Nat,
    pub list: List,
}

impl TestPrelude {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn into() -> (Bool, Nat, List) {
        Self::default().into()
    }

    pub fn spread() -> (BoolTuple, NatTuple, ListTuple) {
        let (bool, nat, list) = Self::into();
        (bool.into(), nat.into(), list.into())
    }
}

impl_vars_vec_aggregate!(TestPrelude, bool, nat, list);
