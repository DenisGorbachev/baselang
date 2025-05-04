use crate::{module, typ, var, VarRc, Wat};

module!(
    pub struct Int {
        int,
        zero,
        pos,
        neg
    }
);

impl Int {
    pub fn new(wat: &Wat) -> Self {
        var!(w: typ!(&wat.wat));

        var!(int: typ!());
        var!(zero: typ!(&int));
        var!(pos: typ!(w => &int));
        var!(neg: typ!(w => &int));

        Self {
            int,
            zero,
            pos,
            neg,
        }
    }
}
