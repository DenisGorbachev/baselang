use crate::VarRc;

pub trait Parse {
    type Error;

    fn parse(&mut self, input: &str) -> impl Iterator<Item = Result<VarRc, Self::Error>>;
}
