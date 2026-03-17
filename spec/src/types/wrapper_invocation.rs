use std::ffi::OsString;
use std::path::PathBuf;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WrapperInvocation {
    pub rustc_path: OsString,
    pub rustc_args: Vec<OsString>,
    pub selected_package: String,
    pub output_dir: PathBuf,
}
