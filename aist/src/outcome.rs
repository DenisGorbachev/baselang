#[allow(unused_imports)]
use Outcome::*;
use facet::Facet;

#[derive(Facet, Default, Eq, PartialEq, Hash, Clone, Copy, Debug)]
#[repr(u8)]
pub enum Outcome {
    Pass,
    #[default]
    Fail,
}

impl Outcome {}
