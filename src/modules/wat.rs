use crate::{module, typ, var, Nat, VarRc};

module!(
    /// Wat is a "wrapped Nat" (interpreted as 1 + n)
    pub struct Wat {
        wat,
        wat_wrap,
    }
);

impl Wat {
    pub fn new(nat: &Nat) -> Self {
        var!(wat: typ!());

        var!(n: typ!(&nat.nat));
        var!(wat_wrap: typ!(n => &wat));

        Self {
            wat,
            wat_wrap,
        }
    }
}
