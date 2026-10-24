use crate::{SymRc, Typ};
use derive_new::new;

#[derive(new, Clone, Debug)]
pub struct ConstructorDeclaration {
    pub family: SymRc,
    pub constructor: SymRc,
    pub typ: Typ,
}
