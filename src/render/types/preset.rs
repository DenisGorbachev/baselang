use strum::EnumIter;

#[derive(EnumIter, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Debug)]
pub enum Preset {
    EnglishShort,
    EnglishLong,
    BaseDefault,
    BaseIdea,
}
