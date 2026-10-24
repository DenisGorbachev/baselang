use aist_core::{ContractReport, RustContract, WorkspaceInfo, check_workspace_dependencies, observe};
use std::path::Path;

pub fn declarations(report: &mut ContractReport) {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
    check_workspace_dependencies(report, &root, &["derive_getters"]);
    let workspace = observe!(report, WorkspaceInfo::try_from(root.as_path()));
    let contract = observe!(report, RustContract::new(&workspace, "baselang"));
    [
        "pub struct App<F, A, T> { fun: F, arg: A, typ: T }",
        "pub struct Sym<N = Nym, T = Typ> { nym: N, typ: T }",
        "pub struct Rewrite<P = Vec<Exp>, E = Exp> { pub patterns: P, pub body: E }",
        "pub struct Scope<B = HashMap<String, SymRc>, R = Rewrites, C = Constructors> { bindings: B, rewrites: R, constructors: C }",
        "pub struct Theory<S = Scope> { scope: S }",
    ]
    .into_iter()
    .for_each(|expected| contract.compiler_struct(report, expected));
    [
        "pub trait Apply { type Arg; type Typ; type Err: Error; fn apply(&self, arg: &Self::Arg) -> Result<Self::Typ, Self::Err>; }",
        "pub trait Parse { type Error; fn parse(&mut self, input: &str) -> Result<Theory, Self::Error>; }",
        "pub type Rewrites = HashMap<NonNull<Sym>, Vec<Rewrite>>;",
        "pub type Constructors = HashMap<NonNull<Sym>, Vec<SymRc>>;",
        "pub enum Typ { Top, One(Exp), Fun(DuoVec<SymRc>) }",
        "pub type TypRc = Rc<Typ>;",
        "pub enum Exp { Sol(SymRc), App(ExpBox, ExpBox, TypBox) }",
        "pub type ExpRc = Rc<Exp>;",
        "pub struct DuoVec<T>(Vec<T>);",
        "#[macro_export] macro_rules! sym { ($name:ident) => { let $name = $crate::Sym::<$crate::Nym, $crate::Typ>::new_top_rc(stringify!($name)); }; ($name:ident: $typ:expr) => { let $name = $crate::Sym::<$crate::Nym, $crate::Typ>::new_rc(stringify!($name), $typ); }; ($name:ident: $typ:expr; $nym:expr) => { let $name = $crate::Sym::<$crate::Nym, $crate::Typ>::new_rc($nym, $typ); }; }",
        "#[macro_export] macro_rules! typ { () => { $crate::Typ::top() }; ($exp:expr) => { $crate::Typ::one($exp) }; ($sym:ident => $typ:expr) => { $crate::Typ::fun(&$sym, $typ) }; }",
        "#[macro_export] macro_rules! top { () => { $crate::Typ::top() }; }",
        "#[macro_export] macro_rules! exp { ($sym:expr) => { $crate::Exp::from($sym.clone()) }; ($a:expr, $b:expr) => { $crate::Exp::app($a, $b).expect(\"should pass the type check\") }; ($a:expr, $b:expr, $($rest:expr),+$(,)?) => { $crate::exp!($crate::exp!($a, $b), $($rest),+) }; }",
    ]
    .into_iter()
    .for_each(|expected| contract.declaration(report, expected));
    contract.forbids_traits(report, "Sym", &["PartialEq", "Eq", "Hash"]);
    [
        ("App", "fn new(fun: F, arg: A) -> Result<Self, F::Err>;"),
        ("Scope", "fn new_sym(&mut self, nym: impl Into<Nym>, typ: impl Into<Typ>) -> Result<SymRc, ScopeNewSymError>;"),
        ("Scope", "fn insert_sym(&mut self, sym: SymRc) -> Result<SymRc, ScopeInsertSymError>;"),
        ("Scope", "fn insert_rewrite(&mut self, sym: SymRc, rewrite: Rewrite) -> Result<(), ScopeInsertRewriteError>;"),
        ("Scope", "fn define(&mut self, nym: impl Into<Nym>, body: impl Into<Exp>) -> Result<SymRc, ScopeDefineError>;"),
        ("Scope", "fn apply(&self, fun: impl Into<Exp>, arg: impl Into<Exp>) -> Result<Exp, ScopeApplyError>;"),
        ("Scope", "fn normalize(&self, exp: &Exp) -> Result<Exp, ScopeNormalizeError>;"),
        ("Scope", "fn rewrites_of(&self, sym: &SymRc) -> &[Rewrite];"),
        ("Scope", "fn constructors_of(&self, sym: &SymRc) -> &[SymRc];"),
        ("Theory", "fn new(scope: Scope) -> Result<Self, TheoryNewError>;"),
        ("Typ", "fn last(&self) -> &Self;"),
        ("Exp", "fn app(fun: impl Into<Exp>, arg: impl Into<Exp>) -> Result<Self, InvalidApplicationError>;"),
        ("DuoVec", "fn first(&self) -> &T;"),
        ("DuoVec", "fn last(&self) -> &T;"),
    ]
    .into_iter()
    .for_each(|(owner, expected)| contract.method(report, owner, expected));
}
