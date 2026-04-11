use crate::{Nats, module, typ, var};

module!(
    /// * `int.zero` is `0`
    /// * `int.pos nat.zero` is `+1`
    /// * `int.neg nat.zero` is `-1`
    pub struct Ints {
        int,
        zero,
        pos,
        neg
    }
);

impl Ints {
    pub fn new(nats: &Nats) -> Self {
        var!(n: typ!(&nats.nat));

        var!(int: typ!());
        var!(zero: typ!(&int));
        var!(pos: typ!(n => &int));
        var!(neg: typ!(n => &int));

        Self {
            int,
            zero,
            pos,
            neg,
        }
    }
}
