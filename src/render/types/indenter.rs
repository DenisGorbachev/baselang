use derive_getters::Getters;
use derive_more::{From, Into};
use derive_new::new;

#[derive(new, Getters, From, Into, Ord, PartialOrd, Eq, PartialEq, Default, Hash, Clone, Debug)]
pub struct Indenter {
    pub head_prefix: String,
    pub tail_prefix: String,
}

impl Indenter {
    pub fn new_simple(head_prefix: impl Into<String>) -> Self {
        let head_prefix = head_prefix.into();
        let tail_prefix = " ".repeat(head_prefix.len());
        Self {
            head_prefix,
            tail_prefix,
        }
    }

    pub fn indent_block(&self, iter: impl Iterator<Item = String>) -> impl Iterator<Item = String> {
        let Self {
            head_prefix,
            tail_prefix,
        } = self;
        iter.map(move |string| {
            string
                .split('\n')
                .enumerate()
                .map(move |(index, string)| if index == 0 { format!("{head_prefix}{string}") } else { format!("{tail_prefix}{string}") })
                .collect()
        })
    }

    pub fn indent_blocks(&self, iter: impl Iterator<Item = impl Iterator<Item = String>>) -> impl Iterator<Item = impl Iterator<Item = String>> {
        iter.map(move |block| self.indent_block(block))
    }
}

impl From<String> for Indenter {
    fn from(head_prefix: String) -> Self {
        Self::new_simple(head_prefix)
    }
}
