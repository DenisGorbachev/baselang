use clap::ValueEnum;

#[derive(ValueEnum, Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Debug)]
#[value(rename_all = "kebab-case")]
pub enum SizeChange {
    Unknown,
    Equal,
    Strict,
}

impl SizeChange {
    pub fn compose(self, other: Self) -> Self {
        use SizeChange::*;
        match (self, other) {
            (Unknown, _) | (_, Unknown) => Unknown,
            (Equal, Equal) => Equal,
            _ => Strict,
        }
    }
}
