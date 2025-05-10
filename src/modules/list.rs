use crate::{Module, RefsTuple3, Top, VarRc, exp, typ, var};
use derive_more::Into;

#[derive(Into, Eq, PartialEq, Hash, Clone, Debug)]
pub struct List {
    pub list: VarRc,
    pub nil: VarRc,
    pub cons: VarRc,
}

impl List {
    pub fn new() -> Self {
        // List : (t : Top) -> Top
        // List (t : Top) : Top
        var!(t: Top);
        var!(list: typ!(t => Top));

        // Nil : (t : Top) -> List t
        // Nil (t : Top) : List t
        let list_t = exp!(&list, &t);
        var!(nil: typ!(t => typ!(list_t)));

        // Cons : (t : Top) -> (a : t) -> List t
        // Cons (t : Top) (a : t) : List t
        var!(a: typ!(exp!(t)));
        let list_t = exp!(&list, &t);
        let cons_typ = typ!(t => typ!(a => typ!(list_t)));
        var!(cons: cons_typ);

        Self {
            list,
            nil,
            cons,
        }
    }
}

impl Default for List {
    fn default() -> Self {
        Self::new()
    }
}

impl Module for List {
    type RefsTuple<'a> = RefsTuple3<'a>;

    fn vars_refs(&self) -> Vec<&VarRc> {
        vec![&self.list, &self.nil, &self.cons]
    }

    fn refs_tuple(&self) -> Self::RefsTuple<'_> {
        (&self.list, &self.nil, &self.cons)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{BaseRenderer, must_print};

    must_print!(List, BaseRenderer, "list/prints/plain.base");
}
