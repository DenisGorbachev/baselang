use crate::{Bool, List, Nat};
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
}
