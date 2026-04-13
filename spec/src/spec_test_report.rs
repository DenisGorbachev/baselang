use aist::Ctx;
use errgonomic::handle;
use facet::Facet;
use std::io;
use thiserror::Error;
use tokio::runtime::Builder as RuntimeBuilder;

#[derive(Facet, Debug)]
pub struct SpecTestReport {
    /// `struct Var`
    pub struct_var: Result<StructVar, StructVarGatherError>,
}

impl SpecTestReport {
    pub fn new(ctx: Ctx<'_>) -> Result<Self, ReportGenerateError> {
        use ReportGenerateError::*;
        let runtime = handle!(RuntimeBuilder::new_current_thread().enable_all().build(), BuildFailed);
        Ok(runtime.block_on(Self::new_async(&ctx)))
    }

    pub async fn new_async(ctx: &Ctx<'_>) -> Self {
        tokio::task::yield_now().await;
        let struct_var = StructVar::new(ctx);
        Self {
            struct_var,
        }
    }
}

#[derive(Error, Debug)]
pub enum ReportGenerateError {
    #[error("failed to build the current-thread runtime")]
    BuildFailed { source: io::Error },
}

mod struct_var;

pub use struct_var::*;
