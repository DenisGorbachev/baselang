use aist_core::{ContractReport, ContractReportIntoResultError};
use errgonomic::handle;
use thiserror::Error;

macro_rules! contract_test {
    ($(#[$attribute:meta])* $test:ident, $error:ident, $contract:ident) => {
        $(#[$attribute])*
        #[test]
        fn $test() -> Result<(), $error> {
            use $error::*;
            let mut report = ContractReport::default();
            report.run(stringify!($contract), $contract);
            handle!(report.into_result(), IntoResultFailed);
            Ok(())
        }

        #[derive(Error, Debug)]
        pub enum $error {
            #[error("behavioral contract failed")]
            IntoResultFailed { source: ContractReportIntoResultError },
        }
    };
}

contract_test!(must_preserve_application_contract, MustPreserveApplicationContractError, application);
contract_test!(must_preserve_registration_contract, MustPreserveRegistrationContractError, registration);
contract_test!(must_preserve_rewrites_contract, MustPreserveRewritesContractError, rewrites);
contract_test!(must_preserve_rollback_contract, MustPreserveRollbackContractError, rollback);
contract_test!(must_preserve_normalization_contract, MustPreserveNormalizationContractError, normalization);
contract_test!(must_preserve_constructors_contract, MustPreserveConstructorsContractError, constructors);
contract_test!(must_preserve_theory_contract, MustPreserveTheoryContractError, theory);
contract_test!(must_preserve_parser_contract, MustPreserveParserContractError, parser);
contract_test!(must_preserve_macros_contract, MustPreserveMacrosContractError, macros);
contract_test!(must_preserve_rendering_contract, MustPreserveRenderingContractError, rendering);
contract_test!(
    #[ignore = "Exp::of_smart is not implemented"]
    must_preserve_smart_application_contract,
    MustPreserveSmartApplicationContractError,
    smart_application
);

mod application;
pub use application::*;
mod registration;
pub use registration::*;
mod rewrites;
pub use rewrites::*;
mod rollback;
pub use rollback::*;
mod normalization;
pub use normalization::*;
mod constructors;
pub use constructors::*;
mod theory;
pub use theory::*;
mod parser;
pub use parser::*;
mod macros;
pub use macros::*;
mod rendering;
pub use rendering::*;
mod smart_application;
pub use smart_application::*;
mod scope_invariants;
pub use scope_invariants::*;
mod constants;
pub use constants::*;
mod probe_apply;
pub use probe_apply::*;
mod same_scope;
pub use same_scope::*;
mod same_exp;
pub use same_exp::*;
