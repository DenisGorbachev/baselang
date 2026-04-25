use crate::Typ;
use crate::{AlphaEq, Module, Nym, NymEn, NymLang, NymRu, NymRuCases, RefsTuple6, VarRc, exp, impl_vars_vec, typ, var};
use derive_more::Into;

#[derive(Into, Clone, Debug)]
pub struct Nats {
    pub nat: VarRc,
    pub zero: VarRc,
    /// Let's use `next` instead of `succ` because it's more understandable
    pub next: VarRc,
    pub add: VarRc,
    pub add_zero: VarRc,
    pub add_next: VarRc,
}

pub type NatsTuple = (VarRc, VarRc, VarRc, VarRc, VarRc, VarRc);

impl Nats {
    pub fn new() -> Self {
        // Nat : Top
        var!(nat: typ!(); Self::nat_nym());

        // `var!(nat: typ!());` expands to the following declaration:
        debug_assert!(nat.nym().as_ref() == Some(&Self::nat_nym()));
        debug_assert!(nat.typ().alpha_eq(&Typ::top()));
        // note that debug_assert_eq! will be removed in optimized builds, and any variables that are used only in debug_assert_eq! invocation should also be treated as dead code and removed by the compiler

        // Zero : Nat
        var!(zero: typ!(&nat));

        // Succ : (n : Nat) -> Nat
        var!(n: typ!(&nat));
        var!(o: typ!(&nat));
        var!(next: typ!(&n => &o));

        // Add : (a : Nat) -> (b : Nat) -> Nat
        var!(a: typ!(&nat));
        var!(b: typ!(&nat));
        var!(o: typ!(&nat));
        var!(add: typ!(&a => &b => &o));

        // Add.Zero : (b : Nat) -> (Add Zero b -> b)
        let add_zero_b_exp = exp!(&add, &zero, &b);
        var!(add_zero_b: typ!(add_zero_b_exp));
        var!(add_zero: typ!(&b => &add_zero_b => &b));

        // Add.Next : (a : Nat) -> (b : Nat) -> (Add (Next a) b -> Next (Add a b))
        let next_a = exp!(&next, &a);
        let add_next_a_b_exp = exp!(&add, next_a, &b);
        var!(add_next_a_b: typ!(add_next_a_b_exp));
        let add_a_b_exp = exp!(&add, &a, &b);
        let next_add_a_b_exp = exp!(&next, add_a_b_exp);
        var!(o: typ!(next_add_a_b_exp));
        var!(add_next: typ!(&a => &b => &add_next_a_b => &o));

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

impl_vars_vec!(Nats, nat, zero, next, add, add_zero, add_next);

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
