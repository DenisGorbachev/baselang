use crate::{Bools, Ints, Lists, Nats, NatsNewError, Rats, Wats, impl_syms_vec_aggregate};
use derive_more::Into;
use errgonomic::handle;
use thiserror::Error;

#[derive(Into, Clone, Debug)]
pub struct Prelude {
    pub bool: Bools,
    pub nat: Nats,
    pub list: Lists,
    pub wat: Wats,
    pub int: Ints,
    pub rat: Rats,
}

impl Prelude {
    pub fn new() -> Result<Self, PreludeNewError> {
        use PreludeNewError::*;
        let bool = Bools::new();
        let nat = handle!(Nats::new(), NatsNewFailed);
        let list = Lists::new();
        let wat = Wats::new(&nat);
        let int = Ints::new(&nat);
        let rat = Rats::new(&int, &wat);
        Ok(Self {
            bool,
            nat,
            list,
            wat,
            int,
            rat,
        })
    }
}

impl_syms_vec_aggregate!(Prelude, bool, nat, list, wat, int, rat);

#[derive(Error, Debug)]
pub enum PreludeNewError {
    #[error("failed to construct natural numbers")]
    NatsNewFailed { source: NatsNewError },
}
