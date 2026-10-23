use crate::{module, sym, top};

module!(
    /// This module is based on International System of Units (SI).
    /// There's only one difference: it uses `gram` instead of `kilogram`.
    pub struct Units {
        meter,
        gram,
        second,
        ampere,
        kelvin,
        mole,
        candela,
    }
);

impl Units {
    pub fn new() -> Self {
        sym!(meter: top!());
        sym!(gram: top!());
        sym!(second: top!());
        sym!(ampere: top!());
        sym!(kelvin: top!());
        sym!(mole: top!());
        sym!(candela: top!());

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
