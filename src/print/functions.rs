pub fn parse_prints(prints: &str) -> Vec<String> {
    prints
        .split('\n')
        .filter(|s| !s.is_empty() && !s.trim_start().starts_with("//"))
        .map(str::to_string)
        .collect()
}
