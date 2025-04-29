use crate::Word;
use derive_getters::Getters;
use derive_more::{From, Into};
use derive_new::new;

#[derive(new, Getters, From, Into, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Debug)]
pub struct NymRu {
    singular: Word,
    plural: Word,
}

impl NymRu {}
mod nym_ru_cases;

pub use nym_ru_cases::*;
