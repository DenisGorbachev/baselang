use crate::report;
use aist_core::{RustContractDeclarationError, RustContractMethodError, RustContractRequireDeriveError};

report! {
    pub struct ScopeChecks {
        pub declaration: Result<(), RustContractDeclarationError>,
        pub rc_alias: Result<(), RustContractDeclarationError>,
        pub getters: Result<(), RustContractRequireDeriveError>,
        pub new_sym: Result<(), RustContractMethodError>,
        pub insert_sym: Result<(), RustContractMethodError>,
        pub insert_rewrite: Result<(), RustContractMethodError>,
        pub define: Result<(), RustContractMethodError>,
        pub apply: Result<(), RustContractMethodError>,
        pub normalize: Result<(), RustContractMethodError>,
        pub rewrites_of: Result<(), RustContractMethodError>,
        pub constructors_of: Result<(), RustContractMethodError>,
    }
}
