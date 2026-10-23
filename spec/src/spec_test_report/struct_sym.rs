use aist::{Ctx, CtxAdtStructError};
use errgonomic::handle;
use facet::Facet;
use thiserror::Error;

#[derive(Facet, Debug)]
pub struct StructSym {
    pub fields: Result<StructSymFields, StructSymFieldsNewError>,
    pub methods: StructSymMethods,
}

impl StructSym {
    pub fn new(ctx: &Ctx<'_>) -> Result<Self, StructSymNewError> {
        use StructSymNewError::*;
        let sym = handle!(ctx.adt_struct("Sym"), AdtStructFailed);
        let fields = StructSymFields::new(sym);
        let methods = StructSymMethods::new(ctx, sym);
        Ok(Self {
            fields,
            methods,
        })
    }
}

#[derive(Error, Facet, Debug)]
#[repr(u8)]
pub enum StructSymNewError {
    #[error("failed to get `struct Sym`")]
    AdtStructFailed { source: StructSymAdtStructError },
}

#[derive(Error, Facet, Debug)]
#[repr(u8)]
pub enum StructSymAdtStructError {
    #[error("struct not found by symbol '{symbol}'")]
    StructNotFound { symbol: String },
    #[error("multiple structs found by symbol '{symbol}'")]
    StructInvalid { symbol: String },
}

impl From<CtxAdtStructError> for StructSymAdtStructError {
    fn from(value: CtxAdtStructError) -> Self {
        use CtxAdtStructError::*;
        match value {
            StructNotFound {
                symbol,
            } => Self::StructNotFound {
                symbol: symbol.to_string(),
            },
            StructInvalid {
                symbol,
            } => Self::StructInvalid {
                symbol: symbol.to_string(),
            },
        }
    }
}

mod struct_sym_fields;

pub use struct_sym_fields::*;

mod struct_sym_methods;

pub use struct_sym_methods::*;
