use crate::{Nats, exp, module, sym, top, typ};

// TODO: Move it to `Nats`
module!(
    pub struct Sum {
        sum,
        sum_base,
        sum_step,
    }
);

impl Sum {
    pub fn new(nat: &Nats) -> Self {
        let Nats {
            nat,
            zero,
            next,
            ..
        } = nat;

        sym!(a: typ!(exp!(nat)));
        sym!(b: typ!(exp!(nat)));
        sym!(c: typ!(exp!(nat)));

        sym!(sum: typ!(a => typ!(b => typ!(c => top!()))));

        let sum_zero_c_c_exp = exp!(&sum, zero, &c, &c);
        sym!(sum_base: typ!(c => sum_zero_c_c_exp));

        let next_a = exp!(next, &a);
        let next_c = exp!(next, &c);
        let sum_a_b_c_exp = exp!(&sum, &a, &b, &c);
        sym!(sum_a_b_c: typ!(sum_a_b_c_exp));
        let sum_next_a_b_next_c_exp = exp!(&sum, next_a, &b, next_c);
        sym!(sum_step: typ!(a => typ!(b => typ!(c => typ!(sum_a_b_c => typ!(sum_next_a_b_next_c_exp))))));

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
