use derive_more::{From, Into};
use derive_new::new;

#[derive(new, From, Into, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Debug)]
pub struct NymRu {
    pub singular: NymRuCases,
    /// Russian language has elaborate rules for pluralization, so we have to force the translator to define it explicitly
    pub plural: NymRuCases,
}

impl NymRu {}

mod nym_ru_cases;

pub use nym_ru_cases::*;
