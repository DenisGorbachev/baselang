use crate::Word;
use derive_more::{From, Into};
use derive_new::new;

#[derive(new, From, Into, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Debug)]
pub struct NymEn {
    pub singular: Word,
    /// If this field is equal to [`None`], then apply a regular pluralization rule to a singular word
    pub plural: Option<Word>,
}

impl From<&str> for NymEn {
    fn from(value: &str) -> Self {
        Self::from(value.to_string())
    }
}

impl From<String> for NymEn {
    fn from(value: String) -> Self {
        let singular = Word::from(value);
        Self {
            singular,
            plural: None,
        }
    }
}
