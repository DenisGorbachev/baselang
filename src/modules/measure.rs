use crate::{exp, module, top, typ, var, Rat, VarRc};

module!(
    /// The correct name for this module is ["quantity value"](https://jcgm.bipm.org/vim/en/1.19.html), but it's too long.
    pub struct Measure {
        measure,
        new,
    }
);

impl Measure {
    pub fn new(rat: &Rat) -> Self {
        var!(value: typ!(exp!(rat.rat)));
        var!(unit: typ!());

        var!(measure: typ!(unit => top!()));

        let measure_of_unit = exp!(&measure, &unit);
        var!(new: typ!(value => typ!(unit => typ!(measure_of_unit))));

        Self {
            measure,
            new,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{parse_prints, Module, Prelude};
    use pretty_assertions::assert_eq;

    #[test]
    fn must_print() {
        let prelude = Prelude::new();
        assert_eq!(Measure::new(&prelude.rat).print(), parse_prints(include_str!("measure/prints/plain.base")))
    }
}
