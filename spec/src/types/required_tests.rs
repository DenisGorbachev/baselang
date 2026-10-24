use crate::report;
use aist_core::RustContractTestFnError;

report! {
    pub struct RequiredTests {
        pub application: Result<(), RustContractTestFnError>,
        pub registration: Result<(), RustContractTestFnError>,
        pub rewrites: Result<(), RustContractTestFnError>,
        pub rollback: Result<(), RustContractTestFnError>,
        pub normalization: Result<(), RustContractTestFnError>,
        pub constructors: Result<(), RustContractTestFnError>,
        pub theory: Result<(), RustContractTestFnError>,
        pub parser: Result<(), RustContractTestFnError>,
        pub macros: Result<(), RustContractTestFnError>,
        pub rendering: Result<(), RustContractTestFnError>,
    }
}
