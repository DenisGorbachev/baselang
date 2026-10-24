use crate::{AppChecks, DuoVecChecks, ExpChecks, MacroChecks, RequiredTests, RewriteChecks, ScopeChecks, SymChecks, TheoryChecks, TypChecks, report};
use aist_core::RustContractDeclarationError;

report! {
    pub struct Declarations {
        pub app: AppChecks,
        pub sym: SymChecks,
        pub rewrite: RewriteChecks,
        pub scope: ScopeChecks,
        pub theory: TheoryChecks,
        pub typ: TypChecks,
        pub exp: ExpChecks,
        pub duo_vec: DuoVecChecks,
        pub apply_trait: Result<(), RustContractDeclarationError>,
        pub parse_trait: Result<(), RustContractDeclarationError>,
        pub rewrites: Result<(), RustContractDeclarationError>,
        pub constructors: Result<(), RustContractDeclarationError>,
        pub macros: MacroChecks,
        pub tests: RequiredTests,
    }
}
