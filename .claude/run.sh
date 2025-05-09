#!/usr/bin/env -S usage bash
#USAGE arg "<child_branch>" help="The child branch name"
#USAGE arg "<prompt_file>" help="The prompt file path"

set -xeuo pipefail

child_branch=${usage_child_branch:?}
prompt_file=${usage_prompt_file:?}

git checkout -b --track=inherit "$child_branch"
cat "$prompt_file" | claude --debug --verbose --output-format stream-json --print
mise run test
mise run fmt
