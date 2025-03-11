use crate::{exp, typ, var, VarRc};
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
        // Bool : Top
        var!(bool: typ!());

        // Yes : Bool
        var!(yes: typ!(exp!(bool)));

        // No : Bool
        var!(no: typ!(exp!(bool)));

        Self {
            bool,
            yes,
            no,
        }
    }
}
