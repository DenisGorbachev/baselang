/*!
```rust assert_eq(yes.name, "arst"); ```
*/

use crate::{Exp, Module, RefsTuple3, Typ, Var, VarRc};
use derive_more::Into;

#[derive(Into, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Debug)]
pub struct Bool {
    pub bool: VarRc,
    /// This field is named `yes` instead of `true` because `true` is a reserved keyword in Rust
    /// Note that the printed name of this variable is "true" (in line with expectations of the users)
    pub yes: VarRc,
    /// This field is named `no` instead of `false` because `false` is a reserved keyword in Rust
    /// /// Note that the printed name of this variable is "false" (in line with expectations of the users)
    pub no: VarRc,
}

impl Bool {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for Bool {
    fn default() -> Self {
        // Bool : Top
        let bool = Var::new_rc("bool", Typ::top());

        // Yes : Bool
        let yes = Var::new_rc("true", Typ::one(Exp::sol(&bool)));

        // No : Bool
        let no = Var::new_rc("false", Typ::one(Exp::sol(&bool)));

        Self {
            bool,
            yes,
            no,
        }
    }
}

impl Module for Bool {
    type RefsTuple<'a> = RefsTuple3<'a>;

    fn vars(&self) -> Vec<VarRc> {
        vec![self.bool.clone(), self.yes.clone(), self.no.clone()]
    }

    fn refs_tuple(&self) -> Self::RefsTuple<'_> {
        (&self.bool, &self.yes, &self.no)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_prints;

    #[test]
    fn must_print() {
        assert_eq!(Bool::new().print(), parse_prints(include_str!("bool/prints/plain.base")))
    }
}
