use crate::{Module, RefsTuple3, VarRc, exp, typ, var};
use crate::{Typ, Var};
use derive_more::Into;

#[derive(Into, Eq, PartialEq, Hash, Clone, Debug)]
pub struct Nat {
    pub nat: VarRc,
    pub zero: VarRc,
    /// Let's use `next` instead of `succ` because it's more understandable
    pub next: VarRc,
}

impl Nat {
    pub fn new() -> Self {
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

impl Default for Nat {
    fn default() -> Self {
        Self::new()
    }
}

impl Module for Nat {
    type RefsTuple<'a> = RefsTuple3<'a>;

    fn vars_refs(&self) -> Vec<&VarRc> {
        vec![&self.nat, &self.zero, &self.next]
    }

    fn refs_tuple(&self) -> Self::RefsTuple<'_> {
        (&self.nat, &self.zero, &self.next)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{CueRenderer, PlainRenderer};

    #[test]
    fn test_print_plain() {
        let module = Nat::new();
        let renderer = PlainRenderer::new();
        let prints = include_str!("nat/prints/plain.base");
        crate::assert_eq_prints(&module, &renderer, prints);
    }

    #[test]
    fn test_print_cue() {
        let module = Nat::new();
        let renderer = CueRenderer::new();
        let prints = include_str!("nat/prints/cue.base");
        crate::assert_eq_prints(&module, &renderer, prints);
    }
}
