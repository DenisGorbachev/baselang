use crate::{SymRc, Typ, exp, module, sym, typ};
use Typ::*;

module!(
    pub struct Lists {
        list,
        nil,
        cons,
    }
);

pub type ListsTuple = (SymRc, SymRc, SymRc);

impl Lists {
    pub fn new() -> Self {
        // List : (t : Top) -> Top
        // List (t : Top) : Top
        sym!(t: Top);
        sym!(list: typ!(t => Top));

        // Nil : (t : Top) -> List t
        // Nil (t : Top) : List t
        let list_t = exp!(&list, &t);
        sym!(nil: typ!(t => typ!(list_t)));

        // Cons : (t : Top) -> (a : t) -> List t
        // Cons (t : Top) (a : t) : List t
        sym!(a: typ!(exp!(t)));
        let list_t = exp!(&list, &t);
        let cons_typ = typ!(t => typ!(a => typ!(list_t)));
        sym!(cons: cons_typ);

        Self {
            list,
            nil,
            cons,
        }
    }
}

impl Default for Lists {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{PlainRenderer, must_print};

    must_print!(Lists, PlainRenderer, "lists/prints/plain.base");
}
