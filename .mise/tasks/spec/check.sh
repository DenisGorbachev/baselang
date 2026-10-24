#!/usr/bin/env bash

set -euo pipefail

project_root=${MISE_PROJECT_ROOT:-$(pwd)}
cd "$project_root"
cargo run --locked --package spec
