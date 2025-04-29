use derive_more::From;

#[derive(From, Ord, PartialOrd, Eq, PartialEq, Default, Hash, Clone, Debug)]
pub enum Capitalized {
    /// Return the result of applying a regular capitalization rule to the canonical word
    #[default]
    FromCanonical,
    /// Return a canonical form directly
    AsCanonical,
    /// Return a custom string
    Custom(String),
}

impl Capitalized {
    pub fn from_canonical(str: &str) -> String {
        let mut chars = str.chars();
        match chars.next() {
            None => String::new(),
            Some(char) => char.to_uppercase().collect::<String>() + chars.as_str(),
        }
    }
}
