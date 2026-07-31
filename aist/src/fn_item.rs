use rustc_hir::ImplicitSelfKind::RefImm;
use rustc_hir::ItemKind::Fn;
use rustc_hir::{BodyId, FnSig, Generics, Item};
use rustc_span::Ident;
use thiserror::Error;

#[derive(Copy, Clone)]
pub struct FnItem<'c> {
    pub sig: FnSig<'c>,
    pub ident: Ident,
    pub generics: &'c Generics<'c>,
    pub body: BodyId,
    pub has_body: bool,
}

impl FnItem<'_> {
    /// Returns whether this function accepts only an immutable borrowed `self` receiver.
    pub fn is_getter(&self) -> bool {
        self.sig.decl.implicit_self() == RefImm && self.sig.decl.inputs.len() == 1
    }
}

impl<'c> TryFrom<Item<'c>> for FnItem<'c> {
    type Error = TryFromItemForFnItemError;

    fn try_from(item: Item<'c>) -> Result<Self, Self::Error> {
        use TryFromItemForFnItemError::*;
        let Item {
            kind,
            ..
        } = item;
        match kind {
            Fn {
                sig,
                ident,
                generics,
                body,
                has_body,
            } => Ok(Self {
                sig,
                ident,
                generics,
                body,
                has_body,
            }),
            _ => Err(ItemKindInvalid {}),
        }
    }
}

#[derive(Error, Debug)]
pub enum TryFromItemForFnItemError {
    #[error("item kind is not a function")]
    ItemKindInvalid {},
}
