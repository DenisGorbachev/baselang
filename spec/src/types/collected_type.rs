use crate::TypeKind;
use core::fmt::{Display, Formatter};

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct CollectedType {
    pub kind: TypeKind,
    pub path: String,
}

impl Display for CollectedType {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{} {}", self.kind, self.path)
    }
}
