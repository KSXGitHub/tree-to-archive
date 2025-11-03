#! /bin/bash
set -o errexit -o pipefail -o nounset
cd "$(dirname "$0")"

just fmt
just doc --all-features

for task in clippy check test; do
  just $task "$@"
  just $task --no-default-features "$@"
  just $task --all-features "$@"
  just $task --features tar "$@"
done
