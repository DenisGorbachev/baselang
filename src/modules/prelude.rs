use crate::{Bools, Ints, Lists, Nats, Rats, Wats, impl_vars_vec_aggregate};
use derive_more::Into;

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
    pub fn new() -> Self {
        let bool = Bools::new();
        let nat = Nats::new();
        let list = Lists::new();
        let wat = Wats::new(&nat);
        let int = Ints::new(&nat);
        let rat = Rats::new(&int, &wat);
        Self {
            bool,
            nat,
            list,
            wat,
            int,
            rat,
        }
    }
}

impl_vars_vec_aggregate!(Prelude, bool, nat, list, wat, int, rat);

impl Default for Prelude {
    fn default() -> Self {
        Self::new()
    }
}
