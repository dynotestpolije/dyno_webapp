#!/usr/bin/env bash
# This scripts runs various CI-like checks in a convenient way.
set -eux

cargo check --all-targets --all-features --workspace
cargo fmt --all -- --check --workspace
cargo clippy --all-targets --all-features --  -D warnings -W clippy::all
