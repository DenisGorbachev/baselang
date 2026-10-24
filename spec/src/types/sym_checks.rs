use crate::report;
use aist_core::{RustContractDeclarationError, RustContractForbidsTraitsError, RustContractRequireDeriveError};

report! {
    pub struct SymChecks {
        pub declaration: Result<(), RustContractDeclarationError>,
        pub rc_alias: Result<(), RustContractDeclarationError>,
        pub getters: Result<(), RustContractRequireDeriveError>,
        pub partial_eq: Result<(), RustContractForbidsTraitsError>,
        pub eq: Result<(), RustContractForbidsTraitsError>,
        pub hash: Result<(), RustContractForbidsTraitsError>,
    }
}
