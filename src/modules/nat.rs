use crate::{Module, Nym, NymEn, NymLang, NymRu, NymRuCases, RefsTuple3, VarRc, exp, impl_vars_vec, typ, var};
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
        var!(nat: typ!(); Self::nat_nym());

        // `var!(nat: typ!());` expands to the following declaration:
        debug_assert_eq!(nat, Var::new_rc(Self::nat_nym(), Typ::top()));
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

    pub fn nat_nym() -> Nym {
        Nym {
            short: NymLang {
                en: NymEn::from("nat"),
                ru: NymRu {
                    singular: NymRuCases::new("нат", "ната", "нату", "нат", "натом", "нате"),
                    plural: NymRuCases::new("наты", "наты", "натам", "натов", "натами", "натах"),
                }
                .into(),
            },
            long: NymLang {
                en: NymEn::from("natural number"),
                ru: NymRu {
                    singular: NymRuCases::new("натуральное число", "натурального числа", "натуральному числу", "натуральное число", "натуральным числом", "натуральном числе"),
                    plural: NymRuCases::new("натуральные числа", "натуральных чисел", "натуральным числам", "натуральные числа", "натуральными числами", "натуральных числах"),
                }
                .into(),
            }
            .into(),
        }
    }
}

impl Default for Nat {
    fn default() -> Self {
        Self::new()
    }
}

impl_vars_vec!(Nat, nat, zero, next);

impl Module for Nat {
    type RefsTuple<'a> = RefsTuple3<'a>;

    fn refs_tuple(&self) -> Self::RefsTuple<'_> {
        (&self.nat, &self.zero, &self.next)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{PlainRenderer, must_print};

    must_print!(Nat, PlainRenderer, "nat/prints/plain.base");
}
