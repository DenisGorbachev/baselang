/// A type is static if it has a fixed set of constructors (it's impossible to add a new constructor)
pub fn every_type_must_be_static() -> Option<bool> {
    match (every_function_must_be_total(), definitions_are_content_addressable()) {
        (true, true) => {
            // it is only possible to determine the totality of a function if all of its types are closed
            // if the definitions are content-addressable, then for any pair of `A : Type` and `B : Type` that have the same constructors, `A` and `B` refer to the same type
            // it is only possible to make definitions content-addressable if the definitions are static
            Some(true)
        }
        _ => None,
    }
}

/// Whether Baselang should support `public`, `private`, `protected`
pub fn should_support_visibilities() -> bool {
    false
}

pub fn every_function_must_be_total() -> bool {
    true
}

pub fn definitions_are_content_addressable() -> bool {
    true
}

pub fn var_struct_must_have_field_constructors_of_option_vec() -> Option<bool> {
    match (must_support_types_with_empty_set_of_constructors(), must_not_require_constructors_of_constructors(), every_type_must_be_static()) {
        (true, true, Some(true)) => Some(true),
        _ => None,
    }
}

/// Must support a logical `False` that has a fixed empty set of constructors
pub fn must_support_types_with_empty_set_of_constructors() -> bool {
    true
}

/// Otherwise it would be impossible to construct any element because it would require an infinite chain of constructors
///
/// Examples:
///
/// - `Zero : Nat` must not require a `ProofOfZero : Zero`
pub fn must_not_require_constructors_of_constructors() -> bool {
    true
}
