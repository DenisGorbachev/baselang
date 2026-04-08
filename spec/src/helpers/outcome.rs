#[allow(unused_imports)]
use Outcome::*;

#[derive(Default, Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub enum Outcome {
    Pass,
    #[default]
    Fail,
}

impl Outcome {}
