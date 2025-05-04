use crate::{exp, typ, var, Module, RefsTuple3, VarRc};
use crate::{Typ, Var};
use derive_more::Into;

#[derive(Into, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Debug)]
pub struct Nat {
    pub nat: VarRc,
    pub zero: VarRc,
    /// Let's use `next` instead of `succ` because it's more understandable
    pub next: VarRc,
}

impl Nat {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for Nat {
    fn default() -> Self {
        // Nat : Top
        var!(nat: typ!());

        // `var!(nat: typ!());` expands to the following declaration:
        debug_assert_eq!(nat, Var::new_rc("nat", Typ::top()));
        // note that debug_assert_eq! will be removed in optimized builds, and any variables that are used only in debug_assert_eq! invocation should also be treated as dead code and removed by the compiler

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
    type RefsTuple<'a> = RefsTuple3<'a>;

    fn vars(&self) -> Vec<VarRc> {
        vec![self.nat.clone(), self.zero.clone(), self.next.clone()]
    }

    fn refs_tuple(&self) -> Self::RefsTuple<'_> {
        (&self.nat, &self.zero, &self.next)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_prints;

    #[test]
    fn must_print() {
        assert_eq!(Nat::new().print(), parse_prints(include_str!("nat/prints/plain.base")))
    }
}
