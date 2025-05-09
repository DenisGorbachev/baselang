pub fn filter_renders(renders: impl Iterator<Item = impl Iterator<Item = Option<String>>>) -> impl Iterator<Item = impl Iterator<Item = String>> {
    renders.map(|blocks| blocks.flatten())
}
