use crate::{Exp, Typ, Var};

/// The Render trait defines methods for rendering variables, types, and expressions
/// into string representations.
///
/// Implementors of this trait can provide different formatting styles or
/// output formats for the same underlying data structures.
pub trait Render: Send + Sync {
    /// Renders a variable into a string representation.
    ///
    /// # Arguments
    ///
    /// * `var` - A reference to the variable to render
    ///
    /// # Returns
    ///
    /// A string representation of the variable
    fn render_var(&self, var: &Var) -> String;

    /// Renders a type into a string representation.
    ///
    /// # Arguments
    ///
    /// * `typ` - A reference to the type to render
    ///
    /// # Returns
    ///
    /// A string representation of the type
    fn render_typ(&self, typ: &Typ) -> String;

    /// Renders an expression into a string representation.
    ///
    /// # Arguments
    ///
    /// * `exp` - A reference to the expression to render
    ///
    /// # Returns
    ///
    /// A string representation of the expression
    fn render_exp(&self, exp: &Exp) -> String;

    /// Returns the name of the renderer.
    ///
    /// This method provides a way to identify the renderer type.
    ///
    /// # Returns
    ///
    /// A string containing the name of the renderer
    fn name(&self) -> &'static str {
        "Unknown"
    }
}
