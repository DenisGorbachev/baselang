use crate::SymRc;

pub trait Parse {
    type Error;

    fn parse(&mut self, input: &str) -> impl Iterator<Item = Result<SymRc, Self::Error>>;
}
