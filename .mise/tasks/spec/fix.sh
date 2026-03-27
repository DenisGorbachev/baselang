#!/usr/bin/env bash

set -euo pipefail

project_root=${MISE_PROJECT_ROOT:-$(pwd)}
spec_binary_path="$project_root/target/release/spec"

RUSTC_BOOTSTRAP=spec cargo build --release --manifest-path spec/Cargo.toml

#RUSTC_WRAPPER= RUSTC_WORKSPACE_WRAPPER="$spec_binary_path" cargo fix --manifest-path Cargo.toml --allow-dirty --allow-staged
RUSTC_WRAPPER= RUSTC_WORKSPACE_WRAPPER="$spec_binary_path" cargo fix --manifest-path Cargo.toml --package baselang --lib --allow-dirty --allow-staged
