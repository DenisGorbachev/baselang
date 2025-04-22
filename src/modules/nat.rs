use crate::{exp, typ, var, Module, VarRc};
use derive_more::{From, Into};
use derive_new::new;

#[derive(new, From, Into, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Debug)]
pub struct Nat {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_prints;

    #[test]
    fn must_print() {
        assert_eq!(Nat::default().print(), parse_prints(include_str!("nat/prints/plain.base")))
    }
}
