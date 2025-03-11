use crate::{Exp, Typ, Var, VarRc};
use derive_more::{From, Into};
use derive_new::new;

#[derive(new, From, Into, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Debug)]
pub struct Bool {
    pub bool: VarRc,
    /// This field is named `yes` instead of `true` because `true` is a reserved keyword in Rust
    pub yes: VarRc,
    /// This field is named `no` instead of `false` because `false` is a reserved keyword in Rust
    pub no: VarRc,
}

impl Default for Bool {
    fn default() -> Self {
        // Bool : Type
        let bool = Var::new_top_rc("Bool");

        // Yes : Bool
        // The following variable is named `yes` instead of `true` because `true` is a reserved keyword in Rust
        let yes = Var::new_rc("True", Typ::One(Exp::Sol(bool.clone())));

        // No : Bool
        // The following variable is named `no` instead of `false` because `false` is a reserved keyword in Rust
        let no = Var::new_rc("False", Typ::One(Exp::Sol(bool.clone())));

        Self {
            bool,
            yes,
            no,
        }
    }
}
