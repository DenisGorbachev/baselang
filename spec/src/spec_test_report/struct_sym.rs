use aist::{Ctx, GetLocalDefIdError, StructDefError};
use errgonomic::handle;
use facet::Facet;
use thiserror::Error;

#[derive(Facet, Debug)]
pub struct StructSym {
    pub fields: StructSymFields,
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
    #[error("failed to get a unique `Sym` item")]
    GetLocalDefIdFailed { source: StructSymGetLocalDefIdError },
    #[error("item '{symbol}' is not a struct")]
    NotStruct { symbol: String },
}

#[derive(Error, Facet, Debug)]
#[repr(u8)]
pub enum StructSymGetLocalDefIdError {
    #[error("item not found by symbol '{symbol}'")]
    NotFound { symbol: String },
    #[error("item not unique by symbol '{symbol}'")]
    NotUnique { symbol: String },
}

impl From<StructDefError> for StructSymAdtStructError {
    fn from(value: StructDefError) -> Self {
        use StructDefError::*;
        match value {
            GetLocalDefIdFailed {
                source,
            } => Self::GetLocalDefIdFailed {
                source: source.into(),
            },
            NotStruct {
                symbol,
            } => Self::NotStruct {
                symbol: symbol.to_string(),
            },
        }
    }
}

impl From<GetLocalDefIdError> for StructSymGetLocalDefIdError {
    fn from(value: GetLocalDefIdError) -> Self {
        use GetLocalDefIdError::*;
        match value {
            NotFound {
                symbol,
            } => Self::NotFound {
                symbol: symbol.to_string(),
            },
            NotUnique {
                symbol,
            } => Self::NotUnique {
                symbol: symbol.to_string(),
            },
        }
    }
}

mod struct_sym_fields;

pub use struct_sym_fields::*;

mod struct_sym_methods;

pub use struct_sym_methods::*;
