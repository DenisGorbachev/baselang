#!/usr/bin/env bash

set -euo pipefail

project_root=${MISE_PROJECT_ROOT:-$(pwd)}
cd "$project_root/spec"
mise --cd "$project_root/spec" exec -- cargo run --locked -- --project-root "$project_root" "$@"
