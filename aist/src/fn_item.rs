use rustc_hir::{BodyId, FnSig, Generics, Item};
use rustc_span::Ident;

#[derive(Copy, Clone)]
pub struct FnItem<'c> {
    pub sig: FnSig<'c>,
    pub ident: Ident,
    pub generics: &'c Generics<'c>,
    pub body: BodyId,
    pub has_body: bool,
}

impl<'c> TryFrom<Item<'c>> for FnItem<'c> {
    type Error = ();

    fn try_from(_item: Item<'c>) -> Result<Self, Self::Error> {
        todo!()
    }
}
