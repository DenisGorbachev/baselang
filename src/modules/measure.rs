use crate::{Rats, exp, module, typ, var};

module!(
    /// The correct name for this module is ["quantity value"](https://jcgm.bipm.org/vim/en/1.19.html), but it's too long.
    pub struct Measure {
        measure,
        new,
    }
);

impl Measure {
    pub fn new(rat: &Rats) -> Self {
        var!(value: typ!(&rat.rat));
        var!(unit: typ!());

        var!(o: typ!());
        var!(measure: typ!(&unit => &o));

        let measure_of_unit = exp!(&measure, &unit);
        var!(o: typ!(measure_of_unit));
        var!(new: typ!(&value => &unit => &o));

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
