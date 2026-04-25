use crate::{Top, VarRc, exp, module, typ, var};

module!(
    pub struct Lists {
        list,
        nil,
        cons,
    }
);

pub type ListsTuple = (VarRc, VarRc, VarRc);

impl Lists {
    pub fn new() -> Self {
        // List : (t : Top) -> Top
        // List (t : Top) : Top
        var!(t: Top);
        var!(o: typ!());
        var!(list: typ!(&t => &o));

        // Nil : (t : Top) -> List t
        // Nil (t : Top) : List t
        let list_t = exp!(&list, &t);
        var!(o: typ!(list_t));
        var!(nil: typ!(&t => &o));

        // Cons : (t : Top) -> (a : t) -> List t
        // Cons (t : Top) (a : t) : List t
        var!(a: typ!(&t));
        let list_t = exp!(&list, &t);
        var!(o: typ!(list_t));
        let cons_typ = typ!(&t => &a => &o);
        var!(cons: cons_typ);

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
