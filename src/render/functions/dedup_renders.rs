use itertools::Itertools;

pub fn dedup_inner_iter<T: PartialEq>(outer: impl Iterator<Item = impl Iterator<Item = T>>) -> impl Iterator<Item = impl Iterator<Item = T>> {
    outer.map(|inner| inner.dedup())
}
