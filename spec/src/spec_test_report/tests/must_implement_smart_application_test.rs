use facet::Facet;

#[derive(Facet, Debug)]
pub struct MustImplementSmartApplicationTest {}

impl Default for MustImplementSmartApplicationTest {
    fn default() -> Self {
        Self::new()
    }
}

impl MustImplementSmartApplicationTest {
    pub fn new() -> Self {
        Self {}
    }
}
