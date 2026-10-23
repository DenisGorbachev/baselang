use crate::{Nats, module, sym, typ};

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
        sym!(n: typ!(&nats.nat));

        sym!(int: typ!());
        sym!(zero: typ!(&int));
        sym!(pos: typ!(n => &int));
        sym!(neg: typ!(n => &int));

        Self {
            int,
            zero,
            pos,
            neg,
        }
    }
}
