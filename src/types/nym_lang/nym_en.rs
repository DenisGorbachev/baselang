use crate::{Phrase, impl_from_via};
use derive_more::{From, Into};
use derive_new::new;

#[derive(new, From, Into, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Debug)]
pub struct NymEn {
    pub singular: Phrase,
    /// If this field is equal to [`None`], then apply a regular pluralization rule to a singular word
    pub plural: Option<Phrase>,
}

impl_from_via!(&str, String, NymEn);

impl From<String> for NymEn {
    fn from(value: String) -> Self {
        let singular = Phrase::from(value);
        Self {
            singular,
            plural: None,
        }
    }
}

impl From<Phrase> for NymEn {
    fn from(singular: Phrase) -> Self {
        Self {
            singular,
            plural: None,
        }
    }
}

impl From<(Phrase, Phrase)> for NymEn {
    fn from((singular, plural): (Phrase, Phrase)) -> Self {
        Self {
            singular,
            plural: Some(plural),
        }
    }
}
