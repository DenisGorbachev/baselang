use crate::VarRc;

pub trait Module {
    fn vars(&self) -> Vec<VarRc>;

    fn print(&self) -> Vec<String> {
        self.vars().into_iter().map(|var| var.print()).collect()
    }
}
