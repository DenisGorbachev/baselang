use baselang::{Indenter, PlainRenderer, Prelude, Render, dedup_renders, render_vars};
use std::io;
use std::io::{Write, stdout};

fn main() -> io::Result<()> {
    let renderers: &[Box<dyn Render>] = &[Box::new(PlainRenderer::new())];
    let indenter = Indenter::new_simple("// ");
    let prelude = Prelude::new();
    let vars = prelude.vars_refs();
    let mut stdout = stdout().lock();
    let renders = render_vars(vars, renderers);
    let renders = dedup_renders(renders);
    let renders = indenter.indent_blocks(renders);
    for blocks in renders {
        for block in blocks {
            writeln!(stdout, "{block}")?;
        }
        writeln!(stdout)?;
    }
    Ok(())
}
