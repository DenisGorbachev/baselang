use std::error::Error;

#[derive(Debug)]
pub struct CheckOutcome<'a> {
    pub name: String,
    pub result: Result<(), &'a dyn Error>,
}

impl CheckOutcome<'_> {
    pub fn prepend_parent(&mut self, parent: &str) {
        self.name = if self.name.is_empty() { parent.to_owned() } else { format!("{parent}/{}", self.name) };
    }
}
