use crate::IntoSymbol;
use derive_getters::Getters;
use derive_more::Deref;
use errgonomic::{handle, handle_bool};
use rustc_hash::FxHashMap;
use rustc_middle::ty::{AdtDef, TyCtxt};
use rustc_span::Symbol;
use rustc_span::def_id::LocalDefId;
use smallvec::SmallVec;
use thiserror::Error;

/// A wrapper around [`TyCtxt`] with a nicer API and caching
#[derive(Getters, Deref, Clone)]
pub struct Ctx<'a> {
    #[deref]
    tcx: TyCtxt<'a>,
    map: LocalSymbolMap,
}

impl<'a> Ctx<'a> {
    pub fn unique_local_def_id(&self, name: impl IntoSymbol) -> Result<LocalDefId, GetLocalDefIdError> {
        use GetLocalDefIdError::*;
        let symbol = name.into_symbol();
        match self.map.get(&symbol).map(SmallVec::as_slice) {
            Some([local_def_id]) => Ok(*local_def_id),
            Some([_, _, ..]) => Err(NotUnique {
                symbol,
            }),
            Some([]) | None => Err(NotFound {
                symbol,
            }),
        }
    }

    pub fn adt_def(&self, name: impl IntoSymbol) -> Result<AdtDef<'a>, GetLocalDefIdError> {
        self.unique_local_def_id(name)
            .map(|local_def_id| self.tcx.adt_def(local_def_id))
    }

    pub fn struct_def(&self, name: impl IntoSymbol) -> Result<AdtDef<'a>, StructDefError> {
        use StructDefError::*;
        let symbol = name.into_symbol();
        let local_def_id = handle!(self.unique_local_def_id(symbol), GetLocalDefIdFailed);
        let adt_def = self.tcx.adt_def(local_def_id);
        handle_bool!(!adt_def.is_struct(), NotStruct, local_def_id, symbol);
        Ok(adt_def)
    }
}

/// Assuming that most Symbols map to only 1 LocalDefId
pub type LocalDefIds = SmallVec<[LocalDefId; 1]>;

pub type LocalSymbolMap = FxHashMap<Symbol, LocalDefIds>;

impl<'a> From<TyCtxt<'a>> for Ctx<'a> {
    fn from(tcx: TyCtxt<'a>) -> Self {
        let map = tcx
            .iter_local_def_id()
            .fold(LocalSymbolMap::default(), |mut map, local_def_id| {
                let symbol = tcx.item_name(local_def_id);
                map.entry(symbol).or_default().push(local_def_id);
                map
            });
        Self {
            tcx,
            map,
        }
    }
}

#[derive(Error, Debug)]
pub enum GetLocalDefIdError {
    #[error("item not found by symbol '{symbol}'")]
    NotFound { symbol: Symbol },
    #[error("item not unique by symbol '{symbol}'")]
    NotUnique { symbol: Symbol },
}

#[derive(Error, Debug)]
pub enum StructDefError {
    #[error("failed to get a unique LocalDefId")]
    GetLocalDefIdFailed { source: GetLocalDefIdError },
    #[error("item '{symbol}' is not a struct")]
    NotStruct { symbol: Symbol, local_def_id: LocalDefId },
}
