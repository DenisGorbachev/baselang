use crate::{Form, NymLang};
use derive_more::{From, Into};
use derive_new::new;

#[derive(new, From, Into, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Debug)]
pub struct Nym {
    /// Short form of the name (e.g. "nat")
    pub short: NymLang,
    /// Long form of the name (e.g. "natural number")
    pub long: Option<NymLang>,
}

impl Nym {
    pub fn get(&self, form: Form) -> Option<&NymLang> {
        match form {
            Form::Short => Some(&self.short),
            Form::Long => self.long.as_ref(),
        }
    }
}

impl From<&str> for Nym {
    fn from(value: &str) -> Self {
        Self::from(String::from(value))
    }
}

impl From<String> for Nym {
    fn from(value: String) -> Self {
        Self::from(NymLang::from(value))
    }
}

impl From<NymLang> for Nym {
    fn from(value: NymLang) -> Self {
        Self {
            short: value,
            long: Default::default(),
        }
    }
}
