use derive_more::{From, Into};
use derive_new::new;
use std::rc::Rc;

/// `Nym` is a struct that stores different names of a [`Var`](crate::Var). The word "nym" is a Greek suffix that means "name".
#[derive(new, From, Into, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Debug)]
pub struct Nym {
    pub en: NymEn,
    pub ru: Option<NymRu>,
}

pub type NymRc = Rc<Nym>;

impl Nym {}

impl From<&str> for Nym {
    fn from(value: &str) -> Self {
        Self::from(value.to_string())
    }
}

impl From<String> for Nym {
    fn from(value: String) -> Self {
        Self {
            en: value.into(),
            ru: Default::default(),
        }
    }
}

mod nym_en;
mod nym_ru;
mod word;

pub use nym_en::*;
pub use nym_ru::*;
pub use word::*;
