#[cfg(feature = "aist-codex")]
use crate::{InvariantReviewsReport, InvariantReviewsRunError};
use crate::{StructuralReport, report};

report! {
    pub struct SpecReport {
        pub structure: StructuralReport,
        #[cfg(feature = "aist-codex")]
        pub invariants: Result<InvariantReviewsReport, InvariantReviewsRunError>,
    }
}
