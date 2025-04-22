use crate::{exp, typ, var, Module, Top, VarRc};
use derive_more::{From, Into};
use derive_new::new;

#[derive(new, From, Into, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Debug)]
pub struct List {
    pub list: VarRc,
    pub nil: VarRc,
    pub cons: VarRc,
}

impl Default for List {
    fn default() -> Self {
        // List : (t : Top) -> Top
        // List (t : Top) : Top
        var!(t: Top);
        var!(list: typ!(t => Top));

        // Nil : (t : Top) -> List t
        // Nil (t : Top) : List t
        let list_t = exp!(list, t);
        var!(nil: typ!(t => typ!(list_t)));

        // Cons : (t : Top) -> (a : t) -> List t
        // Cons (t : Top) (a : t) : List t
        var!(a: typ!(exp!(t)));
        let list_t = exp!(list, t);
        let cons_typ = typ!(t => typ!(a => typ!(list_t)));
        var!(cons: cons_typ);

        Self {
            list,
            nil,
            cons,
        }
    }
}

impl Module for List {
    type RefsTuple<'a> = (&'a VarRc, &'a VarRc, &'a VarRc);

    fn vars(&self) -> Vec<VarRc> {
        vec![self.list.clone(), self.nil.clone(), self.cons.clone()]
    }

    fn refs_tuple(&self) -> Self::RefsTuple<'_> {
        (&self.list, &self.nil, &self.cons)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_prints;
    use pretty_assertions::assert_eq;
    use std::iter::zip;

    /// Using a loop to see the error diffs more clearly
    #[test]
    fn must_print() {
        let prints_actual = List::default().print();
        let prints_expected = parse_prints(include_str!("list/prints/plain.base"));
        for (actual, expected) in zip(prints_actual, prints_expected) {
            assert_eq!(actual, expected);
        }
    }
}
