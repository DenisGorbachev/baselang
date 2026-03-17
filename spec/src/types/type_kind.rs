use TypeKind::*;
use core::fmt::{Display, Formatter};

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum TypeKind {
    Enum,
    Struct,
    Type,
    Union,
}

impl Display for TypeKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Enum => f.write_str("enum"),
            Struct => f.write_str("struct"),
            Type => f.write_str("type"),
            Union => f.write_str("union"),
        }
    }
}
