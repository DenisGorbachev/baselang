use std::error::Error;

pub trait Apply {
    type Arg;
    type Typ;
    type Error: Error + 'static;

    fn apply(&self, arg: &Self::Arg) -> Result<Self::Typ, Self::Error>;
}
