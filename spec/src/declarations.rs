use crate::{AppChecks, Declarations, DuoVecChecks, ExpChecks, MacroChecks, RequiredTests, RewriteChecks, ScopeChecks, SymChecks, TheoryChecks, TypChecks};
use aist_core::{RustContract, RustContractNewError, TryFromPathForWorkspaceInfoError, WorkspaceInfo};
use errgonomic::handle;
use std::path::{Path, PathBuf};
use thiserror::Error;

pub fn declarations(root: &Path) -> Result<Declarations, DeclarationsError> {
    use DeclarationsError::*;
    let workspace = handle!(WorkspaceInfo::try_from(root), TryFromFailed);
    let contract = handle!(RustContract::new(&workspace, "baselang"), NewFailed, root: root.to_path_buf());
    Ok(Declarations {
        app: AppChecks {
            declaration: contract.declaration("pub struct App<F, A, T> { fun: F, arg: A, typ: T }"),
            rc_alias: contract.declaration("pub type AppRc<F, A, T> = Rc<App<F, A, T>>;"),
            getters: contract.require_derive("App", "Getters"),
            constructor: contract.method("App", "pub fn new(fun: F, arg: A) -> Result<Self, F::Err>;"),
        },
        sym: SymChecks {
            declaration: contract.declaration("pub struct Sym<N = Nym, T = Typ> { nym: N, typ: T }"),
            rc_alias: contract.declaration("pub type SymRc<N = Nym, T = Typ> = Rc<Sym<N, T>>;"),
            getters: contract.require_derive("Sym", "Getters"),
            partial_eq: contract.forbids_traits("Sym", &["PartialEq"]),
            eq: contract.forbids_traits("Sym", &["Eq"]),
            hash: contract.forbids_traits("Sym", &["Hash"]),
        },
        rewrite: RewriteChecks {
            declaration: contract.declaration("pub struct Rewrite<P = Vec<Exp>, E = Exp> { pub patterns: P, pub body: E }"),
            rc_alias: contract.declaration("pub type RewriteRc<P = Vec<Exp>, E = Exp> = Rc<Rewrite<P, E>>;"),
            getters: contract.require_derive("Rewrite", "Getters"),
        },
        scope: ScopeChecks {
            declaration: contract.declaration("pub struct Scope<B = HashMap<String, SymRc>, R = Rewrites, C = Constructors> { bindings: B, rewrites: R, constructors: C }"),
            rc_alias: contract.declaration("pub type ScopeRc<B = HashMap<String, SymRc>, R = Rewrites, C = Constructors> = Rc<Scope<B, R, C>>;"),
            getters: contract.require_derive("Scope", "Getters"),
            new_sym: contract.method("Scope", "pub fn new_sym(&mut self, nym: impl Into<Nym>, typ: impl Into<Typ>) -> Result<SymRc, ScopeNewSymError>;"),
            insert_sym: contract.method("Scope", "pub fn insert_sym(&mut self, sym: SymRc) -> Result<SymRc, ScopeInsertSymError>;"),
            insert_rewrite: contract.method("Scope", "pub fn insert_rewrite(&mut self, sym: SymRc, rewrite: Rewrite) -> Result<(), ScopeInsertRewriteError>;"),
            define: contract.method("Scope", "pub fn define(&mut self, nym: impl Into<Nym>, body: impl Into<Exp>) -> Result<SymRc, ScopeDefineError>;"),
            apply: contract.method("Scope", "pub fn apply(&self, fun: impl Into<Exp>, arg: impl Into<Exp>) -> Result<Exp, ScopeApplyError>;"),
            normalize: contract.method("Scope", "pub fn normalize(&self, exp: &Exp) -> Result<Exp, ScopeNormalizeError>;"),
            rewrites_of: contract.method("Scope", "pub fn rewrites_of(&self, sym: &SymRc) -> &[Rewrite];"),
            constructors_of: contract.method("Scope", "pub fn constructors_of(&self, sym: &SymRc) -> &[SymRc];"),
        },
        theory: TheoryChecks {
            declaration: contract.declaration("pub struct Theory<S = Scope> { scope: S }"),
            rc_alias: contract.declaration("pub type TheoryRc<S = Scope> = Rc<Theory<S>>;"),
            getters: contract.require_derive("Theory", "Getters"),
            constructor: contract.method("Theory", "pub fn new(scope: Scope) -> Result<Self, TheoryNewError>;"),
        },
        typ: TypChecks {
            declaration: contract.enum_declaration("pub enum Typ { Top, One(Exp), Fun }", &["Fun"]),
            rc_alias: contract.declaration("pub type TypRc = Rc<Typ>;"),
            last: contract.method("Typ", "pub fn last(&self) -> &Self;"),
        },
        exp: ExpChecks {
            declaration: contract.declaration("pub enum Exp { Sol(SymRc), App(ExpBox, ExpBox, TypBox) }"),
            rc_alias: contract.declaration("pub type ExpRc = Rc<Exp>;"),
            app: contract.method("Exp", "pub fn app(fun: impl Into<Exp>, arg: impl Into<Exp>) -> Result<Self, InvalidApplicationError>;"),
        },
        duo_vec: DuoVecChecks {
            declaration: contract.declaration("pub struct DuoVec<T> { prefix: [T; 2], tail: Vec<T> }"),
            rc_alias: contract.declaration("pub type DuoVecRc<T> = Rc<DuoVec<T>>;"),
            first: contract.method("DuoVec", "pub fn first(&self) -> &T;"),
            last: contract.method("DuoVec", "pub fn last(&self) -> &T;"),
        },
        apply_trait: contract.declaration("pub trait Apply { type Arg; type Typ; type Err: Error; fn apply(&self, arg: &Self::Arg) -> Result<Self::Typ, Self::Err>; }"),
        parse_trait: contract.declaration("pub trait Parse { type Error; fn parse(&mut self, input: &str) -> Result<Theory, Self::Error>; }"),
        rewrites: contract.declaration("pub type Rewrites = HashMap<NonNull<Sym>, Vec<Rewrite>>;"),
        constructors: contract.declaration("pub type Constructors = HashMap<NonNull<Sym>, Vec<SymRc>>;"),
        macros: MacroChecks {
            sym: contract.declaration("#[macro_export] macro_rules! sym { ($name:ident) => { let $name = $crate::Sym::<$crate::Nym, $crate::Typ>::new_top_rc(stringify!($name)); }; ($name:ident: $typ:expr) => { let $name = $crate::Sym::<$crate::Nym, $crate::Typ>::new_rc(stringify!($name), $typ); }; ($name:ident: $typ:expr; $nym:expr) => { let $name = $crate::Sym::<$crate::Nym, $crate::Typ>::new_rc($nym, $typ); }; }"),
            typ: contract.declaration("#[macro_export] macro_rules! typ { () => { $crate::Typ::top() }; ($exp:expr) => { $crate::Typ::one($exp) }; ($sym:ident => $typ:expr) => { $crate::Typ::fun(&$sym, $typ) }; }"),
            top: contract.declaration("#[macro_export] macro_rules! top { () => { $crate::Typ::top() }; }"),
            exp: contract.declaration("#[macro_export] macro_rules! exp { ($sym:expr) => { $crate::Exp::from($sym.clone()) }; ($a:expr, $b:expr) => { $crate::Exp::app($a, $b).expect(\"should pass the type check\") }; ($a:expr, $b:expr, $($rest:expr),+$(,)?) => { $crate::exp!($crate::exp!($a, $b), $($rest),+) }; }"),
        },
        tests: RequiredTests {
            application: contract.test_fn("must_preserve_application_contract"),
            registration: contract.test_fn("must_preserve_registration_contract"),
            rewrites: contract.test_fn("must_preserve_rewrites_contract"),
            rollback: contract.test_fn("must_preserve_rollback_contract"),
            normalization: contract.test_fn("must_preserve_normalization_contract"),
            constructors: contract.test_fn("must_preserve_constructors_contract"),
            theory: contract.test_fn("must_preserve_theory_contract"),
            parser: contract.test_fn("must_preserve_parser_contract"),
            macros: contract.test_fn("must_preserve_macros_contract"),
            rendering: contract.test_fn("must_preserve_rendering_contract"),
        },
    })
}

#[derive(Error, Debug)]
pub enum DeclarationsError {
    #[error("failed to load the inspected workspace")]
    TryFromFailed { source: TryFromPathForWorkspaceInfoError },
    #[error("failed to inspect Baselang at '{root}'", root = root.display())]
    NewFailed { source: RustContractNewError, root: PathBuf },
}
