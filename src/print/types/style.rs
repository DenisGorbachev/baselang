use crate::PrintAll;
use strum::Display;
#[allow(dead_code)]
pub use Style::*;

#[derive(Display, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub enum Style {
    Plain,
    // Args on the left
    // Args in (), separated with commas
    // Args in {}, separated with commas
    // Space between var name and colon
    // Var names: CamelCase, snake_case, «2 + 2 equals 4» : (n : Nat) -> («2 + 2» : Sum 2 2 n) -> Eq n 4
    // Var names: shorthands vs full names (Equal vs Eq)
    // Var names: styles (Next vs Succ, List vs Array) -- this can be customized at the namespace level with "name pack" (which should be custom for user, btw)
    // Inline printing of binary relations (n `Eq` 4) / replace the relation name with relation symbol (n = 4) / replace with plain name (n equals 4)
    // Highlighting
}

impl Style {
    pub fn print(&self, printable: impl PrintAll) -> Vec<String> {
        // TODO: Apply the style
        printable.print_all()
    }
}
