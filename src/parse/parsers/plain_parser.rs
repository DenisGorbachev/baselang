use derive_new::new;
use std::borrow::Cow;

#[derive(new, Eq, PartialEq, Hash, Clone, Debug)]
pub struct PlainParser {
    /// The name of the [`Typ::Top`]
    #[new(into)]
    pub top: Cow<'static, str>,
}

// macro_rules! stub_iter {
//     () => {
//         // stub_macro::stub!(impl dyn Iterator<Item = $item>)
//         //         todo!();
//         panic!("ast");
//         std::iter::empty()
//     };
// }
//
// impl Parse for PlainParser {
//     type Error = ();
//
//     fn parse(&mut self, input: &str) -> impl Iterator<Item = Result<VarRc, Self::Error>> {
//         stub_iter!()
//     }
// }

#[cfg(test)]
mod tests {
    #[test]
    fn must_parse() {}
}
