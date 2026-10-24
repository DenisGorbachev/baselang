use crate::report;
use aist_core::{RustContractDeclarationError, RustContractMethodError};

report! {
    pub struct ExpChecks {
        pub declaration: Result<(), RustContractDeclarationError>,
        pub rc_alias: Result<(), RustContractDeclarationError>,
        pub app: Result<(), RustContractMethodError>,
    }
}
