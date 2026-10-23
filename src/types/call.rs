use crate::{Exp, SizeChange, SymRc};
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct Call {
    pub from: SymRc,
    pub to: SymRc,
    pub changes: Vec<Vec<SizeChange>>,
    pub width: usize,
}

impl Call {
    pub fn new(from: SymRc, to: SymRc, patterns: &[Exp], args: &[&Exp], width: usize) -> Self {
        use SizeChange::*;
        let changes = patterns
            .iter()
            .map(|pattern| {
                (0..width)
                    .map(|index| match args.get(index) {
                        Some(arg) if arg.same_term(pattern) => Equal,
                        Some(arg) if arg.strict_subterm(pattern) => Strict,
                        _ => Unknown,
                    })
                    .collect()
            })
            .collect();
        Self {
            from,
            to,
            changes,
            width,
        }
    }

    pub fn compose(&self, other: &Self) -> Option<Self> {
        use SizeChange::*;
        if !Rc::ptr_eq(&self.to, &other.from) {
            return None;
        }
        let changes = self
            .changes
            .iter()
            .map(|row| {
                (0..other.width)
                    .map(|column| {
                        row.iter()
                            .enumerate()
                            .map(|(index, left)| {
                                let right = other
                                    .changes
                                    .get(index)
                                    .and_then(|row| row.get(column))
                                    .copied()
                                    .unwrap_or(Unknown);
                                left.compose(right)
                            })
                            .max()
                            .unwrap_or(Unknown)
                    })
                    .collect()
            })
            .collect();
        Some(Self {
            from: self.from.clone(),
            to: other.to.clone(),
            changes,
            width: other.width,
        })
    }

    pub fn same(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.from, &other.from) && Rc::ptr_eq(&self.to, &other.to) && self.width == other.width && self.changes == other.changes
    }

    pub fn fails_descent(&self) -> bool {
        Rc::ptr_eq(&self.from, &self.to)
            && self
                .compose(self)
                .is_some_and(|composed| composed.same(self))
            && !self
                .changes
                .iter()
                .enumerate()
                .any(|(index, row)| row.get(index) == Some(&SizeChange::Strict))
    }
}
