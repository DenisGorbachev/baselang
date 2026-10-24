use crate::report;
use aist_core::{RustContractDeclarationError, RustContractMethodError};

report! {
    pub struct DuoVecChecks {
        pub declaration: Result<(), RustContractDeclarationError>,
        pub rc_alias: Result<(), RustContractDeclarationError>,
        pub first: Result<(), RustContractMethodError>,
        pub last: Result<(), RustContractMethodError>,
    }
}
