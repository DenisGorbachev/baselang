use crate::{module, typ, var, Int, VarRc, Wat};

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
        var!(den: typ!(&wat.wat));

        var!(rat: typ!());
        var!(new: typ!(num => typ!(den => typ!(&rat))));

        Self {
            rat,
            new,
        }
    }
}
