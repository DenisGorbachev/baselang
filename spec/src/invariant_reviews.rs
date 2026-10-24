use crate::{InvariantReviewsReport, check_requirement, check_type_invariants};
use aist_codex::{CargoReviewOptions, CargoReviewSnapshotError, CodexReviewer, ReviewRequest, cargo_review_snapshot};
use clap::Parser;
use errgonomic::handle;
use futures::join;
use std::num::{NonZeroU64, NonZeroUsize};
use std::path::{Path, PathBuf};
use std::time::Duration;
use thiserror::Error;
use tokio::sync::Semaphore;

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
    pub async fn run(self, root: &Path) -> Result<InvariantReviewsReport, InvariantReviewsRunError> {
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
        let semaphore = Semaphore::new(self.review_concurrency.get());
        let apply_request = ReviewRequest {
            id: "Apply.validity".into(),
            question: "Every implementation F of baselang::Apply must specify a validity predicate V_F(fun, arg, typ). F::apply must return Ok(typ) exactly when a typ satisfying V_F(self, arg, typ) exists, and its returned typ must satisfy that predicate; it must return Err when no valid typ exists. Inspect implementations, declared predicates, and dependent type checking logic.".into(),
        };
        let (app, sym, rewrite, scope, theory, typ, exp, duo_vec, apply) = join!(check_type_invariants(&reviewer, &snapshot, &semaphore, "App"), check_type_invariants(&reviewer, &snapshot, &semaphore, "Sym"), check_type_invariants(&reviewer, &snapshot, &semaphore, "Rewrite"), check_type_invariants(&reviewer, &snapshot, &semaphore, "Scope"), check_type_invariants(&reviewer, &snapshot, &semaphore, "Theory"), check_type_invariants(&reviewer, &snapshot, &semaphore, "Typ"), check_type_invariants(&reviewer, &snapshot, &semaphore, "Exp"), check_type_invariants(&reviewer, &snapshot, &semaphore, "DuoVec"), check_requirement(&reviewer, &snapshot, &semaphore, apply_request),);
        Ok(InvariantReviewsReport {
            app,
            sym,
            rewrite,
            scope,
            theory,
            typ,
            exp,
            duo_vec,
            apply,
        })
    }
}

#[derive(Error, Debug)]
pub enum InvariantReviewsRunError {
    #[error("failed to capture Baselang and its dependency context")]
    CargoReviewSnapshotFailed { source: CargoReviewSnapshotError },
}
