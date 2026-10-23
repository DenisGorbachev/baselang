use crate::{Bools, BoolsTuple, Lists, ListsTuple, Nats, NatsNewError, NatsTuple, impl_syms_vec_aggregate};
use derive_more::{From, Into};
use errgonomic::handle;
use thiserror::Error;

#[derive(From, Into, Clone, Debug)]
pub struct TestPrelude {
    pub bool: Bools,
    pub nat: Nats,
    pub list: Lists,
}

impl TestPrelude {
    pub fn new() -> Result<Self, TestPreludeNewError> {
        use TestPreludeNewError::*;
        let nat = handle!(Nats::new(), NatsNewFailed);
        Ok(Self {
            bool: Bools::new(),
            nat,
            list: Lists::new(),
        })
    }

    pub fn into() -> Result<(Bools, Nats, Lists), TestPreludeIntoError> {
        use TestPreludeIntoError::*;
        let prelude = handle!(Self::new(), NewFailed);
        Ok(prelude.into())
    }

    pub fn spread() -> Result<(BoolsTuple, NatsTuple, ListsTuple), TestPreludeSpreadError> {
        use TestPreludeSpreadError::*;
        let (bools, nats, lists) = handle!(Self::into(), IntoFailed);
        Ok((bools.into(), nats.into(), lists.into()))
    }
}

impl_syms_vec_aggregate!(TestPrelude, bool, nat, list);

#[derive(Error, Debug)]
pub enum TestPreludeNewError {
    #[error("failed to construct natural numbers")]
    NatsNewFailed { source: NatsNewError },
}

#[derive(Error, Debug)]
pub enum TestPreludeIntoError {
    #[error("failed to construct the test prelude")]
    NewFailed { source: TestPreludeNewError },
}

#[derive(Error, Debug)]
pub enum TestPreludeSpreadError {
    #[error("failed to construct the component modules")]
    IntoFailed { source: TestPreludeIntoError },
}
