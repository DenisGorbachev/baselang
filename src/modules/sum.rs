use crate::{Nats, exp, module, top, typ, var};

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

        var!(a: typ!(nat));
        var!(b: typ!(nat));
        var!(c: typ!(nat));

        var!(o: top!());
        var!(sum: typ!(&a => &b => &c => &o));

        let sum_zero_c_c_exp = exp!(&sum, zero, &c, &c);
        var!(o: typ!(sum_zero_c_c_exp));
        var!(sum_base: typ!(&c => &o));

        let next_a = exp!(next, &a);
        let next_c = exp!(next, &c);
        let sum_a_b_c_exp = exp!(&sum, &a, &b, &c);
        var!(sum_a_b_c: typ!(sum_a_b_c_exp));
        let sum_next_a_b_next_c_exp = exp!(&sum, next_a, &b, next_c);
        var!(o: typ!(sum_next_a_b_next_c_exp));
        var!(sum_step: typ!(&a => &b => &c => &sum_a_b_c => &o));

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
