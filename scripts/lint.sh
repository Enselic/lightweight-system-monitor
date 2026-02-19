#!/usr/bin/env bash
set -o nounset -o errexit -o pipefail -o xtrace

cargo fmt -- --check

RUSTDOCFLAGS='--deny warnings' cargo doc --locked --no-deps --document-private-items

cargo clippy \
    --locked \
    --all-targets \
    --all-features \
    -- \
    --deny warnings \
    --forbid unsafe_code
