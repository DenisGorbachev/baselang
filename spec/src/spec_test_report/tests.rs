use facet::Facet;

#[derive(Facet, Debug)]
pub struct Tests {
    pub must_implement_smart_application: MustImplementSmartApplicationTest,
}

impl Default for Tests {
    fn default() -> Self {
        Self::new()
    }
}

impl Tests {
    pub fn new() -> Self {
        let must_implement_smart_application = MustImplementSmartApplicationTest::new();
        Self {
            must_implement_smart_application,
        }
    }
}

mod must_implement_smart_application_test;

pub use must_implement_smart_application_test::*;
