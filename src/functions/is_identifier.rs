pub fn is_identifier(input: &str) -> bool {
    input.split('.').all(|part| {
        let mut chars = part.chars();
        chars
            .next()
            .is_some_and(|ch| ch.is_alphabetic() || ch == '_')
            && chars.all(|ch| ch.is_alphanumeric() || ch == '_')
    })
}
