use crate::{Parse, VarRc};
use derive_new::new;
use std::borrow::Cow;
use stub_macro::stub_iter;
use thiserror::Error;

#[derive(new, Eq, PartialEq, Hash, Clone, Debug)]
pub struct PlainParser {
    /// The name of the [`Typ::Top`]
    #[new(into)]
    pub top: Cow<'static, str>,
}

impl Parse for PlainParser {
    type Error = PlainParserParseError;

    fn parse(&mut self, _input: &str) -> impl Iterator<Item = Result<VarRc, Self::Error>> {
        stub_iter!()
    }
}

#[derive(Error, Debug)]
pub enum PlainParserParseError {}

#[cfg(test)]
mod tests {
    use crate::{Parse, PlainParser};
    use itertools::Itertools;

    #[ignore]
    #[test]
    fn must_parse() {
        let mut parser = PlainParser::new("^_^");
        let iter = parser.parse(include_str!("../../../samples/vector.plain.base"));
        let _: Vec<_> = iter.try_collect().unwrap();
    }
}
