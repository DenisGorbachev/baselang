use crate::Theory;

pub trait Parse {
    type Error;

    fn parse(&mut self, input: &str) -> Result<Theory, Self::Error>;
}
