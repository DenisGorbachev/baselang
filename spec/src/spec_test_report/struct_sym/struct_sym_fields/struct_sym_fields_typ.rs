use crate::{StructSymFieldsFieldIsNotMutatedByMethodsError, field_is_not_mutated_by_methods};
use aist::Adt;
use facet::Facet;

#[derive(Facet, Debug)]
pub struct StructSymFieldsTyp {
    pub mutators: Result<(), StructSymFieldsFieldIsNotMutatedByMethodsError>,
}

impl StructSymFieldsTyp {
    pub fn new(sym: Adt) -> Self {
        let mutators = field_is_not_mutated_by_methods(sym, "typ");
        Self {
            mutators,
        }
    }
}
