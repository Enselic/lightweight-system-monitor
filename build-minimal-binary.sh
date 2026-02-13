#!/usr/bin/env bash
set -o errexit -o nounset -o pipefail

RUSTFLAGS="-Cforce-unwind-tables=no" \
cargo +nightly build \
    -Zpanic-immediate-abort \
    -Zbuild-std=std,panic_abort \
    -Zbuild-std-features= \
    --config profile.release.panic='"immediate-abort"' \
    --config profile.release.strip=true \
    --config profile.release.debug=false \
    --release \
    && \
ls -lh target/release/lightweight-system-monitor
