use crate::{Ints, Wats, module, typ, var};

module!(
    pub struct Rats {
        rat,
        new
    }
);

impl Rats {
    pub fn new(ints: &Ints, wats: &Wats) -> Self {
        // numerator must be either positive, negative or zero (so we can use an `int`)
        var!(num: typ!(&ints.int));
        // denominator must be non-zero (so we must use a `wat`)
        // we could use a `nat` with implied meaning that `zero` is actually `1`, but it would be very confusing, so we use a `wat` to explicitly wrap it: `wat.new nat.zero`
        var!(den: typ!(&wats.wat));

        var!(rat: typ!());
        var!(new: typ!(num => typ!(den => typ!(&rat))); "rat.new");

        Self {
            rat,
            new,
        }
    }
}
