use crate::{Int, VarRc, Wat, module, typ, var};

module!(
    pub struct Rat {
        rat,
        new
    }
);

impl Rat {
    pub fn new(int: &Int, wat: &Wat) -> Self {
        // numerator must be either positive, negative or zero (so we can use an `int`)
        var!(num: typ!(&int.int));
        // denominator must be non-zero (so we must use a `wat`)
        // we could use a `nat` with implied meaning that `zero` is actually `1`, but it would be very confusing, so we use a `wat` to explicitly wrap it: `wat.new nat.zero`
        var!(den: typ!(&wat.wat));

        var!(rat: typ!());
        var!(new: typ!(num => typ!(den => typ!(&rat))));

        Self {
            rat,
            new,
        }
    }
}
