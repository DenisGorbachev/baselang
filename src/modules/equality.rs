use crate::{Top, VarRc, exp, module, typ, var};

module!(
    /// The identity type.
    pub struct Equality {
        eq,
        refl,
    }
);

pub type EqualityTuple = (VarRc, VarRc);

impl Equality {
    pub fn new() -> Self {
        // Eq : (t : Top) -> (a : t) -> (b : t) -> Top
        var!(t: Top);
        var!(a: typ!(exp!(t)));
        var!(b: typ!(exp!(t)));
        var!(eq: typ!(t => typ!(a => typ!(b => Top))));

        // Refl : (t : Top) -> (a : t) -> Eq t a a
        // note that `t` and `a` are new vars created specifically for `refl`
        var!(t: Top);
        var!(a: typ!(exp!(t)));
        let eq_t_a_a = exp!(&eq, &t, &a, &a);
        var!(refl: typ!(t => typ!(a => typ!(eq_t_a_a))));

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
