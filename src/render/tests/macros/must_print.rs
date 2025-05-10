use crate::{Render, VarsVec, parse_prints, render_vars_vec};
use itertools::Itertools;
use std::iter::zip;

pub fn assert_eq_prints(module: &impl VarsVec, renderer: &impl Render, prints: &str) {
    let prints_actual = render_vars_vec(module, renderer).collect_vec();
    let prints_expected = parse_prints(prints);
    // Using a loop to see the error diffs more clearly
    for (actual, expected) in zip(&prints_actual, &prints_expected) {
        pretty_assertions::assert_eq!(actual, expected);
    }
    // this assertion is still needed because the lengths of vecs might be different
    pretty_assertions::assert_eq!(prints_actual, prints_expected);
}

#[macro_export]
macro_rules! must_print {
    ($(#[$meta:meta])* $module:ident, $renderer:ident, $prints_path:literal) => {
        #[test]
        $(#[$meta])*
        fn must_print() {
            let module = $module::default();
            let renderer = $renderer::default();
            let prints = include_str!($prints_path);
            $crate::assert_eq_prints(&module, &renderer, prints);
        }
    };
}
