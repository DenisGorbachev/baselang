use crate::{exp, module, top, typ, var, Nat, VarRc};

module!(
    /// The correct name for this module is ["quantity value"](https://jcgm.bipm.org/vim/en/1.19.html), but it's too long.
    pub struct Measure {
        measure,
        measure_new,
    }
);

impl Measure {
    pub fn new(nat: &Nat) -> Self {
        var!(value: typ!(exp!(nat.nat)));
        var!(power: typ!(exp!(nat.nat)));
        var!(unit: typ!());

        var!(measure: typ!(unit => top!()));

        let measure_of_unit = exp!(&measure, &unit);
        var!(measure_new: typ!(value => typ!(power => typ!(unit => typ!(measure_of_unit)))));

        Self {
            measure,
            measure_new,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{parse_prints, Module};
    use pretty_assertions::assert_eq;

    #[test]
    #[ignore]
    fn must_print() {
        let nat = Nat::new();
        assert_eq!(Measure::new(&nat).print(), parse_prints(include_str!("measure/prints/plain.base")))
    }
}
