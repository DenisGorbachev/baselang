use itertools::Itertools;

pub fn dedup_renders(renders: impl Iterator<Item = impl Iterator<Item = String>>) -> impl Iterator<Item = impl Iterator<Item = String>> {
    renders.map(|blocks| blocks.dedup())
}
