use rustc_errors::registry::Registry;
use rustc_middle::ty::TyCtxt;
use rustc_session::config::{Input, Options};
use rustc_span::FileName;
use std::sync::atomic::AtomicBool;

static USING_INTERNAL_FEATURES: AtomicBool = AtomicBool::new(false);

pub fn with_tcx<R>(code: &str, f: impl for<'tcx> FnOnce(TyCtxt<'tcx>) -> R + Send) -> R
where
    R: Send,
{
    let config = rustc_interface::Config {
        opts: Options::default(),
        crate_cfg: Vec::new(),
        crate_check_cfg: Vec::new(),
        input: Input::Str {
            name: FileName::Custom("lib.rs".into()),
            input: code.to_owned(),
        },
        output_dir: None,
        output_file: None,
        ice_file: None,
        file_loader: None,
        locale_resources: vec![],
        lint_caps: Default::default(),
        psess_created: None,
        hash_untracked_state: None,
        register_lints: None,
        override_queries: None,
        extra_symbols: Vec::new(),
        make_codegen_backend: None,
        registry: Registry::new(&[]),
        using_internal_features: &USING_INTERNAL_FEATURES,
    };

    rustc_interface::run_compiler(config, |compiler| {
        let krate = rustc_interface::passes::parse(&compiler.sess);
        rustc_interface::create_and_enter_global_ctxt(compiler, krate, |tcx| {
            tcx.ensure_ok().analysis(());
            f(tcx)
        })
    })
}
