#[allow(dead_code)]
pub use Form::*;
use strum::Display;

/// The variants of [`Form`] must be isomorphic to the fields of [`Nym`](crate::Nym)
#[derive(Display, Ord, PartialOrd, Eq, PartialEq, Default, Hash, Clone, Copy, Debug)]
pub enum Form {
    #[default]
    Short,
    Long,
}
impl Form {}
