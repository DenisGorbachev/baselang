use derive_getters::Getters;
use derive_more::{From, Into};
use derive_new::new;

/// `Nym` is a struct that stores different names of a [`Var`](crate::Var). The word "nym" is a Greek suffix that means "name".
#[derive(new, Getters, From, Into, Ord, PartialOrd, Eq, PartialEq, Default, Hash, Clone, Debug)]
pub struct Nym {
    en: (),
    ru: (),
}
impl Nym {}
