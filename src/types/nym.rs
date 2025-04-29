use derive_getters::Getters;
use derive_more::{From, Into};
use derive_new::new;
use std::rc::Rc;

/// `Nym` is a struct that stores different names of a [`Var`](crate::Var). The word "nym" is a Greek suffix that means "name".
#[derive(new, Getters, From, Into, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Debug)]
pub struct Nym {
    en: NymEn,
    ru: Option<NymRu>,
}

pub type NymRc = Rc<Nym>;

impl Nym {}

mod nym_en;
mod nym_ru;
mod word;

pub use nym_en::*;
pub use nym_ru::*;
pub use word::*;
