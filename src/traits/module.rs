use crate::VarRc;

pub trait Module {
    fn vars(&self) -> Vec<VarRc>;
}
