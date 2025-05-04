use crate::VarRc;

pub type RefsTuple1<'a> = &'a VarRc;
pub type RefsTuple2<'a> = (&'a VarRc, &'a VarRc);
pub type RefsTuple3<'a> = (&'a VarRc, &'a VarRc, &'a VarRc);
pub type RefsTuple4<'a> = (&'a VarRc, &'a VarRc, &'a VarRc, &'a VarRc);
pub type RefsTuple5<'a> = (&'a VarRc, &'a VarRc, &'a VarRc, &'a VarRc, &'a VarRc);
pub type RefsTuple6<'a> = (&'a VarRc, &'a VarRc, &'a VarRc, &'a VarRc, &'a VarRc, &'a VarRc);
pub type RefsTuple7<'a> = (&'a VarRc, &'a VarRc, &'a VarRc, &'a VarRc, &'a VarRc, &'a VarRc, &'a VarRc);
pub type RefsTuple8<'a> = (&'a VarRc, &'a VarRc, &'a VarRc, &'a VarRc, &'a VarRc, &'a VarRc, &'a VarRc, &'a VarRc);
pub type RefsTuple9<'a> = (&'a VarRc, &'a VarRc, &'a VarRc, &'a VarRc, &'a VarRc, &'a VarRc, &'a VarRc, &'a VarRc, &'a VarRc);
pub type RefsTuple10<'a> = (&'a VarRc, &'a VarRc, &'a VarRc, &'a VarRc, &'a VarRc, &'a VarRc, &'a VarRc, &'a VarRc, &'a VarRc, &'a VarRc);
pub type RefsTuple11<'a> = (&'a VarRc, &'a VarRc, &'a VarRc, &'a VarRc, &'a VarRc, &'a VarRc, &'a VarRc, &'a VarRc, &'a VarRc, &'a VarRc, &'a VarRc);
pub type RefsTuple12<'a> = (&'a VarRc, &'a VarRc, &'a VarRc, &'a VarRc, &'a VarRc, &'a VarRc, &'a VarRc, &'a VarRc, &'a VarRc, &'a VarRc, &'a VarRc, &'a VarRc);

// TODO: continue up to RefsTuple12
#[macro_export]
macro_rules! refs_tuple_type {
    ($a:ident) => {
        $crate::RefsTuple1<'a>
    };
    ($a:ident, $b:ident) => {
        $crate::RefsTuple2<'a>
    };
    ($a:ident, $b:ident, $c:ident) => {
        $crate::RefsTuple3<'a>
    };
    ($a:ident, $b:ident, $c:ident, $d:ident) => {
        $crate::RefsTuple4<'a>
    };
}
