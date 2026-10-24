use crate::report;
use aist_core::{RustContractDeclarationError, RustContractRequireDeriveError};

report! {
    pub struct RewriteChecks {
        pub declaration: Result<(), RustContractDeclarationError>,
        pub rc_alias: Result<(), RustContractDeclarationError>,
        pub getters: Result<(), RustContractRequireDeriveError>,
    }
}
