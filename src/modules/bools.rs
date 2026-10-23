/*!
```rust assert_eq(yes.name, "arst"); ```
*/

use crate::{Exp, Module, RefsTuple3, Sym, SymRc, Typ, impl_syms_vec};
use derive_more::Into;

#[derive(Into, Clone, Debug)]
pub struct Bools {
    pub bool: SymRc,
    /// This field is named `yes` instead of `true` because `true` is a reserved keyword in Rust
    /// Note that the printed name of this symbol is "true" (matching the user expectations)
    pub yes: SymRc,
    /// This field is named `no` instead of `false` because `false` is a reserved keyword in Rust
    /// Note that the printed name of this symbol is "false" (matching the user expectations)
    pub no: SymRc,
}

pub type BoolsTuple = (SymRc, SymRc, SymRc);

impl Bools {
    pub fn new() -> Self {
        // Bool : Top
        let bool = Sym::new_rc("bool", Typ::top(), None);

        // Yes : Bool
        let yes = Sym::new_rc("true", Typ::one(Exp::sol(&bool)), None);

        // No : Bool
        let no = Sym::new_rc("false", Typ::one(Exp::sol(&bool)), None);

        Self {
            bool,
            yes,
            no,
        }
    }

    pub fn into() -> BoolsTuple {
        Self::default().into()
    }
}

impl Default for Bools {
    fn default() -> Self {
        Self::new()
    }
}

impl_syms_vec!(Bools, bool, yes, no);

impl Module for Bools {
    type RefsTuple<'a> = RefsTuple3<'a>;

    fn refs_tuple(&self) -> Self::RefsTuple<'_> {
        (&self.bool, &self.yes, &self.no)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{PlainRenderer, must_print};

    must_print!(Bools, PlainRenderer, "bools/prints/plain.base");
}
