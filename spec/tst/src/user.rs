use derive_getters::Getters;
use derive_more::{From, Into};
use derive_new::new;
use derive_setters::Setters;

#[derive(new, Setters, Getters, From, Into, Eq, PartialEq, Hash, Clone, Debug)]
#[setters(prefix = "with_")]
pub struct User {
    name: String,
}
