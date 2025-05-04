use crate::Var;

pub fn parse_prints(prints: &str) -> Vec<String> {
    prints
        .split('\n')
        .filter(|s| !s.is_empty() && !s.trim_start().starts_with("//"))
        .map(str::to_string)
        .collect()
}

pub fn print_vars<'a>(vars: impl IntoIterator<Item = &'a Var>) -> Vec<String> {
    vars.into_iter().map(Var::print).collect()
}
