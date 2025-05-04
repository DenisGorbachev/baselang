use crate::{exp, top, typ, var, Module, Nat, VarRc};
use derive_more::{From, Into};

/// The correct name for this module is ["quantity value"](https://jcgm.bipm.org/vim/en/1.19.html), but it's too long.
#[derive(From, Into, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Debug)]
pub struct Measure {
    pub measure: VarRc,
    pub measure_new: VarRc,
}

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

impl Module for Measure {
    type RefsTuple<'a> = (&'a VarRc, &'a VarRc);

    fn vars(&self) -> Vec<VarRc> {
        vec![self.measure.clone(), self.measure_new.clone()]
    }

    fn refs_tuple(&self) -> Self::RefsTuple<'_> {
        (&self.measure, &self.measure_new)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_prints;
    use pretty_assertions::assert_eq;

    #[test]
    #[ignore]
    fn must_print() {
        let nat = Nat::new();
        assert_eq!(Measure::new(&nat).print(), parse_prints(include_str!("measure/prints/plain.base")))
    }
}
