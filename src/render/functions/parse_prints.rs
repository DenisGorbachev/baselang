/// Parses a string containing expected print output into a vector of strings.
///
/// This function takes a string of expected output and splits it into lines,
/// filtering out empty lines and comment lines (lines that start with '//').
///
/// # Arguments
///
/// * `prints` - A string containing the expected output format
///
/// # Returns
///
/// A vector of strings, each representing a line of expected output
pub fn parse_prints(prints: &str) -> Vec<String> {
    // Split by newlines and filter
    let lines: Vec<&str> = prints
        .split('\n')
        .filter(|s| !s.is_empty() && !s.trim_start().starts_with("//"))
        .collect();

    // Convert to strings
    lines.iter().map(|s| s.to_string()).collect()
}
