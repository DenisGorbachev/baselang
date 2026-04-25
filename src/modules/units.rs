use crate::{module, typ, var};

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
        var!(meter: typ!());
        var!(gram: typ!());
        var!(second: typ!());
        var!(ampere: typ!());
        var!(kelvin: typ!());
        var!(mole: typ!());
        var!(candela: typ!());

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
