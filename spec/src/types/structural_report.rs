use crate::{Declarations, DeclarationsError, report};
use aist_core::CheckWorkspaceDependenciesError;

report! {
    pub struct StructuralReport {
        pub workspace_dependencies: Result<(), CheckWorkspaceDependenciesError>,
        pub declarations: Result<Declarations, DeclarationsError>,
    }
}
