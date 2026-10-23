use crate::{Nats, NymEn, module, sym, typ};

module!(
    /// Wat is a "wrapped Nat" (interpreted as 1 + n)
    pub struct Wats {
        wat,
        wrap,
    }
);

impl Wats {
    pub fn new(nats: &Nats) -> Self {
        sym!(wat: typ!());

        sym!(n: typ!(&nats.nat));
        sym!(wrap: typ!(n => &wat); NymEn::from("wat.wrap"));

        Self {
            wat,
            wrap,
        }
    }
}
