use crate::{module, top, var};

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
