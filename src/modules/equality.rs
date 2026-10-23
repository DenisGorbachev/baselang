use crate::{SymRc, Typ, exp, module, sym, typ};
use Typ::*;

module!(
    /// The identity type.
    pub struct Equality {
        eq,
        refl,
    }
);

pub type EqualityTuple = (SymRc, SymRc);

impl Equality {
    pub fn new() -> Self {
        // Eq : (t : Top) -> (a : t) -> (b : t) -> Top
        sym!(t: Top);
        sym!(a: typ!(exp!(t)));
        sym!(b: typ!(exp!(t)));
        sym!(eq: typ!(t => typ!(a => typ!(b => Top))));

        // Refl : (t : Top) -> (a : t) -> Eq t a a
        // note that `t` and `a` are new syms created specifically for `refl`
        sym!(t: Top);
        sym!(a: typ!(exp!(t)));
        let eq_t_a_a = exp!(&eq, &t, &a, &a);
        sym!(refl: typ!(t => typ!(a => typ!(eq_t_a_a))));

        Self {
            eq,
            refl,
        }
    }
}

impl Default for Equality {
    fn default() -> Self {
        Self::new()
    }
}
