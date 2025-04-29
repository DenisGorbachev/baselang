use crate::Capitalized;
use derive_more::{From, Into};
use derive_new::new;
use std::borrow::Cow;
use std::fmt::{Display, Formatter};

/// This struct holds the canonical and capitalized names of a [`crate::Var`]
///
/// ```rust
/// # use baselang::{Capitalized, Word};
/// let list = Word::new("list", Capitalized::FromCanonical);
/// let usa = Word::new("USA", Capitalized::AsCanonical);
/// let usa = Word::new("nat", Capitalized::Custom("Nat".to_string()));
/// ```
#[derive(new, From, Into, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Debug)]
pub struct Word {
    /// A canonical form of the word (e.g. `list`, `nat`, `USA`)
    #[new(into)]
    pub canonical: String,
    /// A capitalized form of the word (e.g. `List`, `Nat`, `USA`)
    #[new(into)]
    pub capitalized: Capitalized,
}

impl Word {
    pub fn to_capitalized(&self) -> Cow<str> {
        use Capitalized::*;
        use Cow::*;
        match &self.capitalized {
            FromCanonical => Owned(Capitalized::from_canonical(&self.canonical)),
            AsCanonical => Borrowed(&self.canonical),
            Custom(string) => Borrowed(string),
        }
    }
}

impl From<String> for Word {
    fn from(canonical: String) -> Self {
        Self {
            canonical,
            capitalized: Capitalized::default(),
        }
    }
}

impl Display for Word {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.canonical)
    }
}
