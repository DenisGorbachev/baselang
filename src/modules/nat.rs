use crate::{exp, typ, var, VarRc};
use derive_more::{From, Into};
use derive_new::new;

#[derive(new, From, Into, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Debug)]
pub struct Nat {
    pub nat: VarRc,
    pub zero: VarRc,
    pub succ: VarRc,
}

impl Default for Nat {
    fn default() -> Self {
        // Nat : Top
        var!(nat: typ!());

        // Zero : Nat
        var!(zero: typ!(exp!(nat)));

        // Succ : (n : Nat) -> Nat
        var!(n: typ!(exp!(nat)));
        var!(succ: typ!(n => typ!(exp!(nat))));

        Self {
            nat,
            zero,
            succ,
        }
    }
}
