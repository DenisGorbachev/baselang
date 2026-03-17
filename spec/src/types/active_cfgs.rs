use std::collections::BTreeSet;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ActiveCfgs {
    pub names: BTreeSet<String>,
    pub pairs: BTreeSet<(String, String)>,
}
