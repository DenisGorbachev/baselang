use crate::{Module, Render, parse_prints, render_module};
use std::iter::zip;

#[allow(unused_variables)]
pub fn assert_eq_prints(module: &impl Module, renderer: &impl Render, prints: &str) {
    let prints_actual = render_module(module, renderer);
    let prints_expected = parse_prints(prints);
    // Using a loop to see the error diffs more clearly
    for (actual, expected) in zip(prints_actual, prints_expected) {
        pretty_assertions::assert_eq!(actual, expected);
    }
}

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
