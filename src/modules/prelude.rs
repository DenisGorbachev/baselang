use crate::{Bool, Int, List, Nat, Rat, Wat, impl_vars_vec_aggregate};
use derive_more::Into;

#[derive(Into, Eq, PartialEq, Hash, Clone, Debug)]
pub struct Prelude {
    pub bool: Bool,
    pub nat: Nat,
    pub list: List,
    pub wat: Wat,
    pub int: Int,
    pub rat: Rat,
}

impl Prelude {
    pub fn new() -> Self {
        let bool = Bool::new();
        let nat = Nat::new();
        let list = List::new();
        let wat = Wat::new(&nat);
        let int = Int::new(&nat);
        let rat = Rat::new(&int, &wat);
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
