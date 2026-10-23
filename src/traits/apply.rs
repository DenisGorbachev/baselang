use std::error::Error;

pub trait Apply {
    type Arg;
    type Typ;
    type Err: Error;

    fn apply(&self, arg: &Self::Arg) -> Result<Self::Typ, Self::Err>;
}
