use crate::report;
use aist_core::RustContractDeclarationError;

report! {
    pub struct MacroChecks {
        pub sym: Result<(), RustContractDeclarationError>,
        pub typ: Result<(), RustContractDeclarationError>,
        pub top: Result<(), RustContractDeclarationError>,
        pub exp: Result<(), RustContractDeclarationError>,
    }
}
