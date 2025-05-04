use crate::{top, var, Module, RefsTuple7, VarRc};
use derive_more::{From, Into};

/// This module is based on International System of Units (SI).
/// There's only one difference: it uses `gram` instead of `kilogram`.
#[derive(From, Into, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Debug)]
pub struct Units {
    pub meter: VarRc,
    pub gram: VarRc,
    pub second: VarRc,
    pub ampere: VarRc,
    pub kelvin: VarRc,
    pub mole: VarRc,
    pub candela: VarRc,
}

impl Units {
    pub fn new() -> Self {
        var!(meter: top!());
        var!(gram: top!());
        var!(second: top!());
        var!(ampere: top!());
        var!(kelvin: top!());
        var!(mole: top!());
        var!(candela: top!());

        Self {
            meter,
            gram,
            second,
            ampere,
            kelvin,
            mole,
            candela,
        }
    }
}

impl Default for Units {
    fn default() -> Self {
        Self::new()
    }
}

impl Module for Units {
    type RefsTuple<'a> = RefsTuple7<'a>;

    fn vars(&self) -> Vec<VarRc> {
        vec![
            self.meter.clone(),
            self.gram.clone(),
            self.second.clone(),
            self.ampere.clone(),
            self.kelvin.clone(),
            self.mole.clone(),
            self.candela.clone(),
        ]
    }

    fn refs_tuple(&self) -> Self::RefsTuple<'_> {
        (&self.meter, &self.gram, &self.second, &self.ampere, &self.kelvin, &self.mole, &self.candela)
    }
}
