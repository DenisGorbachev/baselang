use crate::{Nat, VarRc, exp, module, top, typ, var};

module!(
    pub struct Sum {
        sum,
        sum_base,
        sum_step,
    }
);

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

#[cfg(test)]
mod tests {
    // use super::*;
    // use crate::{PlainRenderer, must_print};
    // must_print!(Sum, PlainRenderer, "sum/prints/plain.base");
}
