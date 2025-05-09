use crate::Phrase;
use derive_more::{From, Into};
use derive_new::new;

#[derive(new, From, Into, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Debug)]
pub struct NymRuCases {
    #[new(into)]
    pub nominative: Phrase,
    #[new(into)]
    pub genitive: Phrase,
    #[new(into)]
    pub dative: Phrase,
    #[new(into)]
    pub accusative: Phrase,
    #[new(into)]
    pub instrumental: Phrase,
    #[new(into)]
    pub prepositional: Phrase,
}

impl NymRuCases {}
