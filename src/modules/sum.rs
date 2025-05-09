use crate::{Module, Nat, RefsTuple3, VarRc, exp, top, typ, var};
use derive_more::{From, Into};

#[derive(From, Into, Eq, PartialEq, Hash, Clone, Debug)]
pub struct Sum {
    pub sum: VarRc,
    pub sum_base: VarRc,
    pub sum_step: VarRc,
}

impl Sum {
    pub fn new(nat: &Nat) -> Self {
        let Nat {
            nat,
            zero,
            next,
        } = nat;

        var!(a: typ!(exp!(nat)));
        var!(b: typ!(exp!(nat)));
        var!(c: typ!(exp!(nat)));

        var!(sum: typ!(a => typ!(b => typ!(c => top!()))));

        let sum_zero_c_c_exp = exp!(&sum, zero, &c, &c);
        var!(sum_base: typ!(c => sum_zero_c_c_exp));

        let next_a = exp!(next, &a);
        let next_c = exp!(next, &c);
        let sum_a_b_c_exp = exp!(&sum, &a, &b, &c);
        var!(sum_a_b_c: typ!(sum_a_b_c_exp));
        let sum_next_a_b_next_c_exp = exp!(&sum, next_a, &b, next_c);
        var!(sum_step: typ!(a => typ!(b => typ!(c => typ!(sum_a_b_c => typ!(sum_next_a_b_next_c_exp))))));

        Self {
            sum,
            sum_base,
            sum_step,
        }
    }
}

impl Module for Sum {
    type RefsTuple<'a> = RefsTuple3<'a>;

    fn vars(&self) -> Vec<VarRc> {
        vec![
            self.sum.clone(),
            self.sum_base.clone(),
            self.sum_step.clone(),
        ]
    }

    fn refs_tuple(&self) -> Self::RefsTuple<'_> {
        (&self.sum, &self.sum_base, &self.sum_step)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_prints;
    use pretty_assertions::assert_eq;

    #[test]
    #[ignore]
    fn must_print() {
        let nat = Nat::new();
        assert_eq!(Sum::new(&nat).print(), parse_prints(include_str!("sum/prints/plain.base")))
    }
}
