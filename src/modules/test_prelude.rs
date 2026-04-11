use crate::{Bools, BoolsTuple, Lists, ListsTuple, Nats, NatsTuple, impl_vars_vec_aggregate};
use derive_more::{From, Into};

#[derive(From, Into, Eq, PartialEq, Default, Hash, Clone, Debug)]
pub struct TestPrelude {
    pub bool: Bools,
    pub nat: Nats,
    pub list: Lists,
}

impl TestPrelude {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn into() -> (Bools, Nats, Lists) {
        Self::default().into()
    }

    pub fn spread() -> (BoolsTuple, NatsTuple, ListsTuple) {
        let (bools, nats, lists) = Self::into();
        (bools.into(), nats.into(), lists.into())
    }
}

impl_vars_vec_aggregate!(TestPrelude, bool, nat, list);
