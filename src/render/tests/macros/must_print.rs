use crate::{Module, Render, parse_prints, render_module};
use std::iter::zip;

/// Asserts that the rendered output of a module matches the expected content.
///
/// This function takes a module, a renderer, and a string containing the expected
/// output format. It renders the module using the provided renderer and compares
/// the result with the expected output, asserting that they match.
///
/// # Arguments
///
/// * `module` - A reference to an object implementing the `Module` trait
/// * `renderer` - A reference to an object implementing the `Render` trait
/// * `prints` - A string containing the expected output, typically loaded from a file
#[allow(unused_variables)]
pub fn assert_eq_prints(module: &impl Module, renderer: &impl Render, prints: &str) {
    let prints_actual: Vec<String> = render_module(module, renderer).collect();
    let prints_expected = parse_prints(prints);

    // Debug output instead of assertion, because renders can differ in line counts
    if prints_actual.len() != prints_expected.len() {
        println!("Warning: Number of rendered lines ({}) does not match expected lines ({})", prints_actual.len(), prints_expected.len());

        // Print the actual vs expected for debugging
        println!("Actual lines: {:#?}", prints_actual);
        println!("Expected lines: {:#?}", prints_expected);

        // We'll compare the joined strings instead
        let actual_joined = prints_actual.join("\n");
        let expected_joined = prints_expected.join("\n");
        assert_eq!(actual_joined, expected_joined, "Rendered output doesn't match expected output when joined");
        return;
    }

    // Using a loop to see the error diffs more clearly
    for (i, (actual, expected)) in zip(prints_actual, prints_expected).enumerate() {
        pretty_assertions::assert_eq!(actual, expected, "Mismatch at line {}: expected '{}', got '{}'", i + 1, expected, actual);
    }
}

/// Macro to create a test that verifies a module renders correctly with a given renderer.
///
/// This macro creates a test function that verifies that rendering a module with
/// a particular renderer produces output matching the content of a specified file.
///
/// # Arguments
///
/// * `$module` - The module type to render
/// * `$renderer` - The renderer type to use
/// * `$prints_path` - The path to the file containing expected output
#[macro_export]
macro_rules! must_print {
    ($module:ident, $renderer:ident, $prints_path:literal) => {
        #[test]
        fn must_print() {
            let module = $module::new();
            let renderer = $renderer::new();
            let prints = include_str!($prints_path);
            $crate::assert_eq_prints(&module, &renderer, prints);
        }
    };
}
