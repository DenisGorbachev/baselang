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
    use crate::{PlainRenderer, Prelude, PreludeNewError, assert_eq_prints};
    use errgonomic::handle;
    use thiserror::Error;

    #[test]
    fn must_print() -> Result<(), MustPrintError> {
        use MustPrintError::*;
        let prelude = handle!(Prelude::new(), NewFailed);
        let measure = Measure::new(&prelude.rat);
        let renderer = PlainRenderer::default();
        let prints = include_str!("measure/prints/plain.base");
        assert_eq_prints(&measure, &renderer, prints);
        Ok(())
    }
    #[derive(Error, Debug)]
    pub enum MustPrintError {
        #[error("failed to construct the prelude")]
        NewFailed { source: PreludeNewError },
    }
}
