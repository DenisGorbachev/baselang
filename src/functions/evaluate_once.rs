use std::sync::atomic::{AtomicBool, Ordering::Relaxed};

pub fn evaluate_once<T>(evaluated: &AtomicBool, expression: impl FnOnce() -> T) -> T {
    assert!(!evaluated.swap(true, Relaxed), "operand must be evaluated at most once");
    expression()
}
