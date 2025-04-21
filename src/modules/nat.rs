use crate::{exp, typ, var, Module, VarRc};
use derive_more::{From, Into};
use derive_new::new;

#[derive(new, From, Into, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Debug)]
pub struct Nat {
    /// The name of the root export, after camel-case transformation, must be equal to the name of the module
    pub nat: VarRc,
    pub zero: VarRc,
    /// Let's use `next` instead of `succ` because it's more understandable
    pub next: VarRc,
}

impl Default for Nat {
    fn default() -> Self {
        // Nat : Top
        var!(nat: typ!());

        // Zero : Nat
        var!(zero: typ!(exp!(nat)));

        // Succ : (n : Nat) -> Nat
        var!(n: typ!(exp!(nat)));
        var!(next: typ!(n => typ!(exp!(nat))));

        Self {
            nat,
            zero,
            next,
        }
    }
}

impl Module for Nat {
    fn vars(&self) -> Vec<VarRc> {
        vec![self.nat.clone(), self.zero.clone(), self.next.clone()]
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::Nat;
//
//     #[test]
//     fn must_print() {
//         let nat = Nat::default();
//         assert_eq!(nat.nat.print(), "Nat : Top");
//         assert_eq!(nat.zero.print(), "Zero : Nat");
//         assert_eq!(nat.next.print(), "Next : (n : Nat) -> Nat");
//     }
// }
