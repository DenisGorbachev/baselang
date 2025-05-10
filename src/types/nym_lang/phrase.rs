use crate::{Capitalized, impl_from_str_as_from_string};
use Cow::*;
use derive_more::{From, Into};
use derive_new::new;
use std::borrow::Cow;
use std::fmt::{Display, Formatter};

/// This struct holds the canonical and capitalized names of a [`crate::Var`]
///
/// ```rust
/// # use baselang::{Capitalized, Phrase};
/// let list = Phrase::new("list", Capitalized::FromCanonical);
/// let natural_number = Phrase::new("natural number", Capitalized::FromCanonical);
/// let usa = Phrase::new("USA", Capitalized::AsCanonical);
/// ```
#[derive(new, From, Into, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Debug)]
pub struct Phrase {
    /// A canonical form of the word (e.g. `list`, `natural number`, `USA`)
    #[new(into)]
    pub canonical: String,
    /// A capitalized form of the word (e.g. `List`, `Natural number`, `USA`)
    #[new(into)]
    pub capitalized: Capitalized,
}

impl Phrase {
    pub fn to_canonical(&self) -> Cow<str> {
        Borrowed(&self.canonical)
    }

    pub fn to_capitalized(&self) -> Cow<str> {
        use Capitalized::*;
        match &self.capitalized {
            FromCanonical => Owned(Capitalized::from_canonical(&self.canonical)),
            AsCanonical => Borrowed(&self.canonical),
            Custom(string) => Borrowed(string),
        }
    }
}

impl_from_str_as_from_string!(Phrase);

impl From<String> for Phrase {
    fn from(canonical: String) -> Self {
        Self {
            canonical,
            capitalized: Capitalized::default(),
        }
    }
}

impl Display for Phrase {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.canonical)
    }
}

impl<'a> From<&'a Phrase> for Cow<'a, str> {
    fn from(value: &'a Phrase) -> Self {
        Borrowed(value.canonical.as_str())
    }
}
