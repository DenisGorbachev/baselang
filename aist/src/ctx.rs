use crate::{Adt, IntoSymbol};
use derive_getters::Getters;
use derive_more::Deref;
use errgonomic::{handle, handle_bool};
use rustc_hash::FxHashMap;
use rustc_middle::ty::TyCtxt;
use rustc_span::Symbol;
use rustc_span::def_id::LocalDefId;
use smallvec::SmallVec;
use thiserror::Error;

/// A wrapper around [`TyCtxt`] with a nicer API and caching.
///
/// This wrapper should contain only those methods that are reusable across multiple specs
#[derive(Getters, Deref, Clone)]
pub struct Ctx<'c> {
    #[deref]
    tcx: TyCtxt<'c>,
    map: LocalSymbolMap,
}

impl<'c> Ctx<'c> {
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

    pub fn adt(&self, name: impl IntoSymbol) -> Result<Adt<'c>, GetLocalDefIdError> {
        self.unique_local_def_id(name)
            .map(|local_def_id| Adt::new(self.tcx.adt_def(local_def_id), self.tcx))
    }

    pub fn struct_def(&self, name: impl IntoSymbol) -> Result<Adt<'c>, StructDefError> {
        use StructDefError::*;
        let symbol = name.into_symbol();
        let adt = handle!(self.adt(symbol), GetLocalDefIdFailed);
        handle_bool!(!adt.is_struct(), NotStruct, symbol);
        Ok(adt)
    }
}

/// Assuming that most Symbols map to only 1 LocalDefId
pub type LocalDefIds = SmallVec<[LocalDefId; 1]>;

pub type LocalSymbolMap = FxHashMap<Symbol, LocalDefIds>;

impl<'a> From<TyCtxt<'a>> for Ctx<'a> {
    fn from(tcx: TyCtxt<'a>) -> Self {
        let map = tcx
            .iter_local_def_id()
            .filter_map(|local_def_id| {
                tcx.opt_item_name(local_def_id)
                    .map(|symbol| (symbol, local_def_id))
            })
            .fold(LocalSymbolMap::default(), |mut map, (symbol, local_def_id)| {
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
    NotStruct { symbol: Symbol },
}
