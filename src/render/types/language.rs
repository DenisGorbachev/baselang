#[allow(dead_code)]
pub use Language::*;
use strum::Display;

#[derive(Display, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub enum Language {
    TypeScript,
    JavaScript,
    Rust,
    C,
    #[strum(serialize = "C++")]
    CPP,
    Java,
    Kotlin,
    Python,
    Baselang,
    Prolog,
    Lean,
    Haskell,
    Zig,
    Go,
    D,
    Nim,
    V,
    Ruby,
    R,
    Julia,
    Lua,
    Perl,
    Erlang,
    Elixir,
    Swift,
    Dart,
}

impl Language {}
