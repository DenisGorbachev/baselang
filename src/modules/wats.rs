use crate::{Nats, NymEn, module, typ, var};

module!(
    /// Wat is a "wrapped Nat" (interpreted as 1 + n)
    pub struct Wats {
        wat,
        wrap,
    }
);

impl Wats {
    pub fn new(nats: &Nats) -> Self {
        var!(wat: typ!());

        var!(n: typ!(&nats.nat));
        var!(o: typ!(&wat));
        var!(wrap: typ!(&n => &o); NymEn::from("wat.wrap"));

        Self {
            wat,
            wrap,
        }
    }
}
