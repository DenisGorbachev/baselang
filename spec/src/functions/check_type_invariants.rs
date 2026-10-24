use crate::{CheckRequirementError, check_requirement};
use aist_codex::{CodexReviewer, ReviewRequest, ReviewSnapshot};
use errgonomic::handle;
use thiserror::Error;
use tokio::sync::Semaphore;

pub async fn check_type_invariants(reviewer: &CodexReviewer, snapshot: &ReviewSnapshot, semaphore: &Semaphore, name: &str) -> Result<(), CheckTypeInvariantsError> {
    use CheckTypeInvariantsError::*;
    let request = ReviewRequest {
        id: format!("{name}.invariants"),
        question: format!("In package baselang, type {name} must explicitly state every semantic invariant as a predicate over its fields or specified method results. Every constructor, conversion, and deserializer returning {name} must establish all its invariants. Every public mutator must preserve all invariants on success and error. Public fields and mutable access are allowed only if every available assignment or mutation preserves all invariants. Inspect all relevant implementations and dependencies; explain any missing invariant declarations."),
    };
    handle!(check_requirement(reviewer, snapshot, semaphore, request).await, CheckRequirementFailed);
    Ok(())
}

#[derive(Error, Debug)]
pub enum CheckTypeInvariantsError {
    #[error("type invariant review failed")]
    CheckRequirementFailed { source: CheckRequirementError },
}
