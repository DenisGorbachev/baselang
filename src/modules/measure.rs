use crate::{Rat, VarRc, exp, module, top, typ, var};

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
    use crate::{PlainRenderer, Prelude, assert_eq_prints};

    #[test]
    #[ignore]
    fn must_print() {
        let prelude = Prelude::new();
        let measure = Measure::new(&prelude.rat);
        let renderer = PlainRenderer::new();
        let prints = include_str!("measure/prints/plain.base");
        assert_eq_prints(&measure, &renderer, prints);
    }
}
