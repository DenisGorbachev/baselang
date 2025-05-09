use crate::{CueRenderer, Module, Nat, Render, render_module};

/// Utility function to debug renderer output.
///
/// This function accepts a module and a renderer, renders the module using the
/// given renderer, and prints the output to standard output for debugging purposes.
pub fn debug_renderer_output(module: &impl Module, renderer: &impl Render) {
    println!("DEBUG: Renderer '{}' output:", renderer.name());
    let renders: Vec<String> = render_module(module, renderer).collect();
    println!("Number of lines: {}", renders.len());
    for (i, line) in renders.iter().enumerate() {
        println!("Line {}: '{}'", i + 1, line);
    }
}
