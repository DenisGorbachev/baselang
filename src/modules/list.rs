use crate::{exp, typ, var, Top, VarRc};
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
