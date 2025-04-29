use crate::Word;
use derive_getters::Getters;
use derive_more::{From, Into};
use derive_new::new;

#[derive(new, Getters, From, Into, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Debug)]
pub struct NymRuCases {
    nominative: Word,
    genitive: Word,
    dative: Word,
    accusative: Word,
    instrumental: Word,
    prepositional: Word,
}

impl NymRuCases {}
