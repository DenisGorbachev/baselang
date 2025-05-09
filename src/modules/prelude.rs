use crate::{Bool, Int, List, Module, Nat, Rat, VarRc, Wat, concat_with_extend};
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

macro_rules! prelude_vars {
    ($($name:ident),+) => {
        pub fn vars_refs(&self) -> Vec<&VarRc> {
            concat_with_extend(vec![
                $(self.$name.vars_refs()),+
            ])
        }
    };
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

    prelude_vars!(bool, nat, list, wat, int, rat);
}

impl Default for Prelude {
    fn default() -> Self {
        Self::new()
    }
}
