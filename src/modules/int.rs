use crate::{Nat, module, typ, var};

module!(
    /// * `int.zero` is `0`
    /// * `int.pos nat.zero` is `+1`
    /// * `int.neg nat.zero` is `-1`
    pub struct Int {
        int,
        zero,
        pos,
        neg
    }
);

impl Int {
    pub fn new(nat: &Nat) -> Self {
        var!(n: typ!(&nat.nat));

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
