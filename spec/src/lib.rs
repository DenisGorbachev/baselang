use aist_core::check_workspace_dependencies;
use std::path::Path;

pub fn check(project_root: &Path) -> StructuralReport {
    StructuralReport {
        workspace_dependencies: check_workspace_dependencies(project_root, &["derive_getters"]),
        declarations: declarations(project_root),
    }
}

mod declarations;
pub use declarations::*;
mod command;
pub use command::*;
mod types;
pub use types::*;
mod traits;
pub use traits::*;
mod functions;
pub use functions::*;
#[cfg(feature = "aist-codex")]
mod invariant_reviews;
mod macros;
#[cfg(feature = "aist-codex")]
pub use invariant_reviews::*;
