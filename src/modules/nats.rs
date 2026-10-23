use crate::Typ;
use crate::{AlphaEq, Module, Nym, NymEn, NymLang, NymRu, NymRuCases, RefsTuple6, SymRc, exp, impl_syms_vec, sym, typ};
use derive_more::Into;

#[derive(Into, Clone, Debug)]
pub struct Nats {
    pub nat: SymRc,
    pub zero: SymRc,
    /// Let's use `next` instead of `succ` because it's more understandable
    pub next: SymRc,
    pub add: SymRc,
    pub add_zero: SymRc,
    pub add_next: SymRc,
}

pub type NatsTuple = (SymRc, SymRc, SymRc, SymRc, SymRc, SymRc);

impl Nats {
    pub fn new() -> Self {
        // Nat : Top
        sym!(nat: typ!(); Self::nat_nym());

        // `sym!(nat: typ!());` expands to the following declaration:
        debug_assert!(nat.nym() == &Self::nat_nym());
        debug_assert!(nat.typ().alpha_eq(&Typ::top()));
        // note that debug_assert_eq! will be removed in optimized builds, and any variables that are used only in debug_assert_eq! invocation should also be treated as dead code and removed by the compiler

        // Zero : Nat
        sym!(zero: typ!(exp!(nat)));

        // Succ : (n : Nat) -> Nat
        sym!(n: typ!(exp!(nat)));
        sym!(next: typ!(n => typ!(exp!(nat))));

        // Add : (a : Nat) -> (b : Nat) -> Nat
        sym!(a: typ!(exp!(nat)));
        sym!(b: typ!(exp!(nat)));
        sym!(add: typ!(a => typ!(b => typ!(exp!(nat)))));

        // Add.Zero : (b : Nat) -> (Add Zero b -> b)
        let add_zero_b_exp = exp!(&add, &zero, &b);
        sym!(add_zero_b: typ!(add_zero_b_exp));
        sym!(add_zero: typ!(b => typ!(add_zero_b => typ!(&b))));

        // Add.Next : (a : Nat) -> (b : Nat) -> (Add (Next a) b -> Next (Add a b))
        let next_a = exp!(&next, &a);
        let add_next_a_b_exp = exp!(&add, next_a, &b);
        sym!(add_next_a_b: typ!(add_next_a_b_exp));
        let add_a_b_exp = exp!(&add, &a, &b);
        let next_add_a_b_exp = exp!(&next, add_a_b_exp);
        sym!(add_next: typ!(a => typ!(b => typ!(add_next_a_b => typ!(next_add_a_b_exp)))));

        Self {
            nat,
            zero,
            next,
            add,
            add_zero,
            add_next,
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

impl Default for Nats {
    fn default() -> Self {
        Self::new()
    }
}

impl_syms_vec!(Nats, nat, zero, next, add, add_zero, add_next);

impl Module for Nats {
    type RefsTuple<'a> = RefsTuple6<'a>;

    fn refs_tuple(&self) -> Self::RefsTuple<'_> {
        (&self.nat, &self.zero, &self.next, &self.add, &self.add_zero, &self.add_next)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{PlainRenderer, must_print};

    must_print!(Nats, PlainRenderer, "nats/prints/plain.base");
}
