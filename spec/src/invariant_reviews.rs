use aist_codex::{CargoReviewOptions, CargoReviewSnapshotError, CodexReviewer, ReviewRequest, cargo_review_snapshot};
use aist_core::ContractReport;
use clap::Parser;
use errgonomic::handle;
use futures::{StreamExt, pin_mut};
use std::num::{NonZeroU64, NonZeroUsize};
use std::path::{Path, PathBuf};
use std::time::Duration;
use thiserror::Error;

#[derive(Parser, Debug)]
#[command(flatten_help = true)]
pub struct InvariantReviews {
    #[arg(long)]
    pub model: String,
    #[arg(long, default_value = "codex")]
    pub codex: PathBuf,
    #[arg(long, default_value = "high")]
    pub reasoning_effort: String,
    #[arg(long)]
    pub cache_directory: PathBuf,
    #[arg(long, default_value = "1")]
    pub cache_epoch: String,
    #[arg(long)]
    pub cache_max_age_seconds: u64,
    #[arg(long)]
    pub review_timeout_seconds: NonZeroU64,
    #[arg(long, default_value = "1")]
    pub review_concurrency: NonZeroUsize,
}

impl InvariantReviews {
    pub async fn run(self, root: &Path, report: &mut ContractReport) -> Result<(), InvariantReviewsRunError> {
        use InvariantReviewsRunError::*;
        let reviewer = CodexReviewer {
            executable: self.codex,
            model: self.model,
            reasoning_effort: self.reasoning_effort,
            cache_directory: self.cache_directory,
            cache_epoch: self.cache_epoch,
            max_age: Duration::from_secs(self.cache_max_age_seconds),
            timeout: Duration::from_secs(self.review_timeout_seconds.get()),
        };
        let options = CargoReviewOptions {
            all_features: true,
            ..CargoReviewOptions::default()
        };
        let snapshot = handle!(cargo_review_snapshot(root, "baselang", &options, reviewer.timeout).await, CargoReviewSnapshotFailed);
        let requests = [
            "App", "Sym", "Rewrite", "Scope", "Theory", "Typ", "Exp", "DuoVec",
        ]
        .into_iter()
        .map(|name| ReviewRequest {
            id: format!("{name}.invariants"),
            question: format!("In package baselang, type {name} must explicitly state every semantic invariant as a predicate over its fields or specified method results. Every constructor, conversion, and deserializer returning {name} must establish all its invariants. Every public mutator must preserve all invariants on success and error. Public fields and mutable access are allowed only if every available assignment or mutation preserves all invariants. Inspect all relevant implementations and dependencies; explain any missing invariant declarations."),
        })
        .chain([ReviewRequest {
            id: "Apply.validity".into(),
            question: "Every implementation F of baselang::Apply must specify a validity predicate V_F(fun, arg, typ). F::apply must return Ok(typ) exactly when a typ satisfying V_F(self, arg, typ) exists, and its returned typ must satisfy that predicate; it must return Err when no valid typ exists. Inspect implementations, declared predicates, and dependent type checking logic.".into(),
        }]);
        let reviews = reviewer.check_many(&snapshot, requests, self.review_concurrency);
        pin_mut!(reviews);
        while let Some((request, result)) = reviews.next().await {
            match result {
                Ok(review) => match review.verdict.into_result() {
                    Ok(()) => report.require(format!("LLM.{}", request.id), true),
                    Err(error) => report.error(format!("LLM.{}", request.id), error),
                },
                Err(error) => report.error(format!("LLM.{}", request.id), error),
            }
        }
        Ok(())
    }
}

#[derive(Error, Debug)]
pub enum InvariantReviewsRunError {
    #[error("failed to capture Baselang and its dependency context")]
    CargoReviewSnapshotFailed { source: CargoReviewSnapshotError },
}
