use crate::{Module, Render};

/// Renders all variables in a module using the specified renderer.
///
/// This function takes a module implementing the `Module` trait and a renderer implementing
/// the `Render` trait, and returns an iterator over strings representing each variable
/// in the module rendered according to the renderer's implementation.
///
/// # Arguments
///
/// * `module` - A reference to an object implementing the `Module` trait
/// * `renderer` - A reference to an object implementing the `Render` trait
///
/// # Returns
///
/// An iterator over strings, each representing a rendered variable from the module.
pub fn render_module(module: &impl Module, renderer: &impl Render) -> impl Iterator<Item = String> {
    module
        .vars_refs()
        .into_iter()
        .map(|x| renderer.render_var(x.as_ref()))
}
