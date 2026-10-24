use aist_codex::{CodexReviewer, CodexReviewerCheckError, ReviewRequest, ReviewSnapshot, ReviewVerdictIntoResultError};
use errgonomic::handle;
use thiserror::Error;
use tokio::sync::{AcquireError, Semaphore};

pub async fn check_requirement(reviewer: &CodexReviewer, snapshot: &ReviewSnapshot, semaphore: &Semaphore, request: ReviewRequest) -> Result<(), CheckRequirementError> {
    use CheckRequirementError::*;
    let _permit = handle!(semaphore.acquire().await, AcquireFailed, request);
    let review = handle!(reviewer.check(snapshot, &request).await, CheckFailed, request);
    handle!(review.verdict.into_result(), IntoResultFailed, request);
    Ok(())
}

#[derive(Error, Debug)]
pub enum CheckRequirementError {
    #[error("failed to acquire review slot for '{id}'", id = request.id)]
    AcquireFailed { source: AcquireError, request: ReviewRequest },
    #[error("review failed for '{id}'", id = request.id)]
    CheckFailed { source: Box<CodexReviewerCheckError>, request: ReviewRequest },
    #[error("requirement was not satisfied for '{id}'", id = request.id)]
    IntoResultFailed { source: ReviewVerdictIntoResultError, request: ReviewRequest },
}
