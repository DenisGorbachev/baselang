use crate::{Sym, SymRc};
use derive_new::new;
use std::collections::{HashMap, HashSet};
use std::ptr::NonNull;

#[derive(new, Clone, Debug)]
pub struct PositivityEdge {
    pub from: SymRc,
    pub to: SymRc,
    pub constructor: SymRc,
    pub strict: bool,
}

impl PositivityEdge {
    pub fn cycle(&self, outgoing: &HashMap<NonNull<Sym>, Vec<&Self>>) -> Option<Vec<Self>> {
        let start = NonNull::from(self.to.as_ref());
        let goal = NonNull::from(self.from.as_ref());
        let mut pending = vec![start];
        let mut visited = HashSet::from([start]);
        let mut parents = HashMap::<NonNull<Sym>, &Self>::new();
        while let Some(current) = pending.pop() {
            if current == goal {
                let mut cycle = Vec::new();
                let mut current = goal;
                while let Some(parent) = parents.get(&current) {
                    cycle.push((*parent).clone());
                    current = NonNull::from(parent.from.as_ref());
                }
                cycle.push(self.clone());
                cycle.reverse();
                return Some(cycle);
            }
            outgoing
                .get(&current)
                .into_iter()
                .flatten()
                .for_each(|next| {
                    let target = NonNull::from(next.to.as_ref());
                    if visited.insert(target) {
                        parents.insert(target, next);
                        pending.push(target);
                    }
                });
        }
        None
    }
}
