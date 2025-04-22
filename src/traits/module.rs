use crate::VarRc;

pub trait Module {
    type RefsTuple<'a>
    where
        Self: 'a;

    fn vars(&self) -> Vec<VarRc>;

    fn refs_tuple(&self) -> Self::RefsTuple<'_>;

    fn print(&self) -> Vec<String> {
        self.vars().into_iter().map(|var| var.print()).collect()
    }
}
