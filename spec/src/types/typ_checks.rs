use crate::report;
use aist_core::{RustContractDeclarationError, RustContractEnumDeclarationError, RustContractMethodError};

report! {
    pub struct TypChecks {
        pub declaration: Result<(), RustContractEnumDeclarationError>,
        pub rc_alias: Result<(), RustContractDeclarationError>,
        pub last: Result<(), RustContractMethodError>,
    }
}
