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

/// Whether vars that have the same type and the same set of constructors must be equal
///
/// ## Examples
///
/// ### Ex 1
///
/// ```baselang
/// A : Type
/// B : Type
/// ```
///
/// Are `A` and `B` equal?
///
/// ### Ex 2
///
/// ```baselang
/// A : Type
/// X : A
///
/// B : Type
/// Y : B
/// ```
///
/// Are `A` and `B` equal?
///
/// ## Notes
///
/// - Structural equality breaks "semantic typing" (`Meters : Type; Meters.Mk : Nat -> Type; Seconds : Type; Seconds.Mk : Nat -> Type`)
pub fn structurally_equal_vars_must_be_equal() -> bool {
    false
}

/// Whether Baselang should support `public`, `private`, `protected`
pub fn should_support_visibilities() -> bool {
    false
}

/// This is true by the definition of the function, but see [`totality_check_can_be_deferred_to_use_time`]
pub fn every_function_must_be_total() -> bool {
    true
}

/// Whether it's possible to check the totality right before use
///
/// ## Pros/Cons
///
/// - But `Vec Bool (Add 2 3)` must be type-checked at compile time, so `Add` must be evaluated at compile time
///   - But such instances are rare
///   - But we can treat it as "use" and freeze only the definition of `Add` without freezing other definitions
///   - But the actual usage is more abstract (see `samples/vector.plain.base`)
pub fn totality_check_can_be_deferred_to_use_time() -> bool {
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

/// It's better to avoid `Var::set_constructors`
pub fn should_not_have_mutators_for_constructors_on_var_struct() -> bool {
    true
}

/// Whether users should be able to add constructors to existing types
///
/// - And it's better to allow it, because:
///   - It would allow users to add variants to non-exhaustive enums
///   - It would allow users to hot-patch the existing types / functions
///     - This requires a `@delete` compiler built-in for completeness
/// - But it would break the totality of the functions
///   - But the users would be able to add constructors / rewriters to restore totality
///     - This requires that the totality checker is run after the definitions are processed
pub fn constructors_should_be_forward_mutable() -> bool {
    todo!()
}

/// Whether users should be able to write `One = Succ Zero` and use `One` in the subsequent declarations
///
/// Implementation options:
/// - Extend `Var` struct with `value: Value` field (`enum Value { Known { exp: Exp }, Unknown { typ: Typ } }`)
///   - Maybe switch from tuple-like `Typ::Fun` to vector-like `Typ::Fun`
///     - Think about how to refactor `Exp::App` to a vector-like variant
/// - Introduce `Exp::Let { var: VarRc, val: Exp, exp: Exp}` variant
/// - Introduce `Typ::Let { var: VarRc, val: Exp, typ: Typ }` variant
///
/// Implementation imperatives:
/// - Must support let bindings at all levels (including the top level)
///   - This requirement invalidates the `Exp::Let` implementation because expressions are not allowed at the top level
pub fn should_support_let_bindings() -> bool {
    true
}

/// Prioritize code readability
pub fn enum_variant_format() -> EnumVariantFormat {
    if struct_enum_variant_fields_are_rendered_on_separate_lines_by_rustfmt() {
        EnumVariantFormat::Tuple
    } else {
        EnumVariantFormat::Struct
    }
}

/// Whether `rustfmt` renders struct enum variants in match arms or destructuring assignments on separate lines, like this:
///
/// ```rust,ignore
/// App {
///   fun,
///   arg,
///   typ,
/// } = ...
/// ```
///
/// This is harder to read.
pub fn struct_enum_variant_fields_are_rendered_on_separate_lines_by_rustfmt() -> bool {
    true
}

#[derive(Clone, Copy, Debug)]
pub enum EnumVariantFormat {
    Struct,
    Tuple,
}

impl EnumVariantFormat {
    /// Prioritize code readability
    pub fn choose() -> Self {
        if struct_enum_variant_fields_are_rendered_on_separate_lines_by_rustfmt() {
            EnumVariantFormat::Tuple
        } else {
            EnumVariantFormat::Struct
        }
    }
}
