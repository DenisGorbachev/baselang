use crate::{Nat, NymEn, VarRc, module, typ, var};

module!(
    /// Wat is a "wrapped Nat" (interpreted as 1 + n)
    pub struct Wat {
        wat,
        wrap,
    }
);

impl Wat {
    pub fn new(nat: &Nat) -> Self {
        var!(wat: typ!());

        var!(n: typ!(&nat.nat));
        var!(wrap: typ!(n => &wat); NymEn::from("wat.wrap"));

        Self {
            wat,
            wrap,
        }
    }
}
