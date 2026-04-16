use crate::{Indenter, Prelude, Preset, Renderer, VarsVec, dedup_inner_iter, filter_renders, render_vars};
use clap::Parser;
use errgonomic::handle;
use itertools::Itertools;
use std::io;
use std::io::{Write, stdout};
use std::process::ExitCode;
use strum::IntoEnumIterator;
use thiserror::Error;

#[derive(Parser, Clone, Debug)]
pub struct RenderPreludeCommand {}

impl RenderPreludeCommand {
    pub async fn run(self) -> Result<ExitCode, RenderPreludeCommandRunError> {
        #[allow(unused_imports)]
        use RenderPreludeCommandRunError::*;
        let Self {} = self;
        let renderers: Vec<Renderer> = Preset::iter().map(Renderer::from).collect_vec();
        let indenter = Indenter::new_simple("// ");
        let prelude = Prelude::new();
        let vars = prelude.vars_vec();
        let renders = render_vars(vars, &renderers);
        let renders = filter_renders(renders);
        let renders = dedup_inner_iter(renders);
        let renders = indenter.indent_blocks(renders);
        let mut stdout = stdout().lock();
        handle!(write_blocks(&mut stdout, renders), WriteBlocksFailed);
        Ok(ExitCode::SUCCESS)
    }
}

pub fn write_blocks<BlockIter, LineIter>(writer: &mut impl Write, blocks: BlockIter) -> Result<(), WriteBlocksError>
where
    BlockIter: IntoIterator<Item = LineIter>,
    LineIter: IntoIterator<Item = String>,
{
    #[allow(unused_imports)]
    use WriteBlocksError::*;
    blocks.into_iter().try_for_each(|block| {
        handle!(write_block(writer, block), WriteBlockFailed);
        handle!(writeln!(writer), WriteBlockSeparatorFailed);
        Ok(())
    })
}

pub fn write_block(writer: &mut impl Write, block: impl IntoIterator<Item = String>) -> Result<(), WriteBlockError> {
    #[allow(unused_imports)]
    use WriteBlockError::*;
    block.into_iter().try_for_each(|line| {
        handle!(writeln!(writer, "{line}"), WriteLineFailed);
        Ok(())
    })
}

#[derive(Error, Debug)]
pub enum RenderPreludeCommandRunError {
    #[error("failed to render prelude")]
    WriteBlocksFailed { source: WriteBlocksError },
}

#[derive(Error, Debug)]
pub enum WriteBlocksError {
    #[error("failed to write a rendered block")]
    WriteBlockFailed { source: WriteBlockError },
    #[error("failed to write a rendered block separator")]
    WriteBlockSeparatorFailed { source: io::Error },
}

#[derive(Error, Debug)]
pub enum WriteBlockError {
    #[error("failed to write a rendered line")]
    WriteLineFailed { source: io::Error },
}
