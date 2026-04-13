use aist::{Ctx, GetLocalDefIdError, StructDefError};
use errgonomic::handle;
use facet::Facet;
use thiserror::Error;

#[derive(Facet, Debug)]
pub struct StructVar {
    pub fields: StructVarFields,
}

impl StructVar {
    pub fn new(ctx: &Ctx<'_>) -> Result<Self, StructVarGatherError> {
        use StructVarGatherError::*;
        let var = handle!(ctx.adt_struct("Var"), AdtStructFailed);
        let fields = StructVarFields::gather(var);
        Ok(Self {
            fields,
        })
    }
}

#[derive(Error, Facet, Debug)]
#[repr(u8)]
pub enum StructVarGatherError {
    #[error("failed to get `struct Var`")]
    AdtStructFailed { source: StructVarAdtStructError },
}

#[derive(Error, Facet, Debug)]
#[repr(u8)]
pub enum StructVarAdtStructError {
    #[error("failed to get a unique `Var` item")]
    GetLocalDefIdFailed { source: StructVarGetLocalDefIdError },
    #[error("item '{symbol}' is not a struct")]
    NotStruct { symbol: String },
}

#[derive(Error, Facet, Debug)]
#[repr(u8)]
pub enum StructVarGetLocalDefIdError {
    #[error("item not found by symbol '{symbol}'")]
    NotFound { symbol: String },
    #[error("item not unique by symbol '{symbol}'")]
    NotUnique { symbol: String },
}

impl From<StructDefError> for StructVarAdtStructError {
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

impl From<GetLocalDefIdError> for StructVarGetLocalDefIdError {
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

mod struct_var_fields;

pub use struct_var_fields::*;
