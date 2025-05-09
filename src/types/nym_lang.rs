use derive_more::{From, Into};
use derive_new::new;
use std::rc::Rc;

/// `Nym` is a struct that stores different names of a [`Var`](crate::Var). The word "nym" is a Greek suffix that means "name".
#[derive(new, From, Into, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Debug)]
pub struct NymLang {
    pub en: NymEn,
    pub ru: Option<NymRu>,
}

pub type NymLangRc = Rc<NymLang>;

impl NymLang {}

impl From<&str> for NymLang {
    fn from(value: &str) -> Self {
        Self::from(String::from(value))
    }
}

impl From<String> for NymLang {
    fn from(value: String) -> Self {
        Self::from(NymEn::from(value))
    }
}

impl From<NymEn> for NymLang {
    fn from(value: NymEn) -> Self {
        Self {
            en: value,
            ru: Default::default(),
        }
    }
}

mod nym_en;
mod nym_ru;
mod phrase;

pub use nym_en::*;
pub use nym_ru::*;
pub use phrase::*;
