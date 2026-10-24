use aist_core::ContractReport;
use std::path::Path;

pub fn check(project_root: &Path) -> ContractReport {
    let mut report = ContractReport::default();
    report.run("structure", |report| declarations(report, project_root));
    report
}

mod declarations;
pub use declarations::*;
mod command;
pub use command::*;
#[cfg(feature = "aist-codex")]
mod invariant_reviews;
#[cfg(feature = "aist-codex")]
pub use invariant_reviews::*;
