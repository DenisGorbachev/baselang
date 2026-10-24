use crate::Apply;
use std::cell::Cell;
use std::ptr::NonNull;
use std::rc::Rc;
use thiserror::Error;

#[derive(Debug)]
pub struct ProbeApply {
    pub calls: Rc<Cell<Option<NonNull<()>>>>,
    pub typ: Option<Rc<()>>,
}

impl Apply for ProbeApply {
    type Arg = Rc<()>;
    type Typ = Rc<()>;
    type Err = ProbeApplyApplyError;

    fn apply(&self, arg: &Self::Arg) -> Result<Self::Typ, Self::Err> {
        use ProbeApplyApplyError::*;
        assert!(
            self.calls
                .replace(Some(NonNull::from(arg.as_ref())))
                .is_none(),
            "apply must be called exactly once"
        );
        match &self.typ {
            Some(typ) => Ok(typ.clone()),
            None => Err(RejectedInvalid {
                arg: arg.clone(),
            }),
        }
    }
}

#[derive(Error, Debug)]
pub enum ProbeApplyApplyError {
    #[error("application rejected")]
    RejectedInvalid { arg: Rc<()> },
}
