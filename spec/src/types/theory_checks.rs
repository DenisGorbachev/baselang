use crate::report;
use aist_core::{RustContractDeclarationError, RustContractMethodError, RustContractRequireDeriveError};

report! {
    pub struct TheoryChecks {
        pub declaration: Result<(), RustContractDeclarationError>,
        pub rc_alias: Result<(), RustContractDeclarationError>,
        pub getters: Result<(), RustContractRequireDeriveError>,
        pub constructor: Result<(), RustContractMethodError>,
    }
}
