use crate::{CueRenderer, PlainRenderer, Render};
use std::sync::Arc;

/// Enum defining the available renderer types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RendererType {
    /// Plain text renderer, suitable for simple output
    Plain,
    /// Cue-based renderer, with more structured formatting
    Cue,
}

/// Returns a renderer of the specified type.
///
/// This factory function provides a centralized way to obtain renderers,
/// making it easier to extend with new renderer types in the future.
///
/// # Arguments
///
/// * `renderer_type` - The type of renderer to create
///
/// # Returns
///
/// A boxed object implementing the `Render` trait
pub fn get_renderer(renderer_type: RendererType) -> Arc<dyn Render> {
    match renderer_type {
        RendererType::Plain => Arc::new(PlainRenderer::new()),
        RendererType::Cue => Arc::new(CueRenderer::new()),
    }
}

/// Returns all available renderers.
///
/// This function returns a vector of all supported renderers, which can be
/// useful for batch processing with multiple output formats.
///
/// # Returns
///
/// A vector of tuples containing the renderer type and a boxed renderer
pub fn get_all_renderers() -> Vec<(RendererType, Arc<dyn Render>)> {
    vec![
        (RendererType::Plain, get_renderer(RendererType::Plain)),
        (RendererType::Cue, get_renderer(RendererType::Cue)),
    ]
}
