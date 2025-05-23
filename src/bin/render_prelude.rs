use baselang::{Indenter, Prelude, Preset, Renderer, VarsVec, dedup_inner_iter, filter_renders, render_vars};
use itertools::Itertools;
use std::io;
use std::io::{Write, stdout};
use strum::IntoEnumIterator;

fn main() -> io::Result<()> {
    let renderers: Vec<Renderer> = Preset::iter().map(Renderer::from).collect_vec();
    let indenter = Indenter::new_simple("// ");
    let prelude = Prelude::new();
    let vars = prelude.vars_vec();
    let mut stdout = stdout().lock();
    let renders = render_vars(vars, &renderers);
    let renders = filter_renders(renders);
    let renders = dedup_inner_iter(renders);
    let renders = indenter.indent_blocks(renders);
    for blocks in renders {
        for block in blocks {
            writeln!(stdout, "{block}")?;
        }
        writeln!(stdout)?;
    }
    Ok(())
}
