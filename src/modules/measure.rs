use crate::{Rats, exp, module, sym, typ};

module!(
    /// The correct name for this module is ["quantity value"](https://jcgm.bipm.org/vim/en/1.19.html), but it's too long.
    pub struct Measure {
        measure,
        new,
    }
);

impl Measure {
    pub fn new(rat: &Rats) -> Self {
        sym!(value: typ!(exp!(rat.rat)));
        sym!(unit: typ!());

        sym!(measure: typ!(unit => typ!()));

        let measure_of_unit = exp!(&measure, &unit);
        sym!(new: typ!(value => typ!(unit => typ!(measure_of_unit))));

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
    fn must_print() {
        let prelude = Prelude::new();
        let measure = Measure::new(&prelude.rat);
        let renderer = PlainRenderer::default();
        let prints = include_str!("measure/prints/plain.base");
        assert_eq_prints(&measure, &renderer, prints);
    }
}
