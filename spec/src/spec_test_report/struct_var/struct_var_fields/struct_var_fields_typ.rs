use super::StructVarFieldsFieldIsNotMutatedByMethodsError;
use aist::Adt;
use facet::Facet;

#[derive(Facet, Debug)]
pub struct StructVarFieldsTyp {
    pub mutators: Result<(), StructVarFieldsFieldIsNotMutatedByMethodsError>,
}

impl StructVarFieldsTyp {
    pub fn new(var: Adt) -> Self {
        let mutators = super::field_is_not_mutated_by_methods(var, "typ");
        Self {
            mutators,
        }
    }
}
