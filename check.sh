#!/usr/bin/env bash
# This scripts runs various CI-like checks in a convenient way.

_FLAGS="${1:-}"
set -ex

if [[ "-b" == ${_FLAGS} ]]; then 
    cargo check --all-targets --all-features --bin "${@:2}"
    cargo fmt --all -- --check --bin  "${@:2}"
    cargo clippy --all-targets --all-features --bin "${@:2}" --  -D warnings -W clippy::all
else
    cargo check --all-targets --all-features --workspace
    cargo fmt --all -- --check
    cargo clippy --all-targets --all-features --workspace --  -D warnings -W clippy::all
fi
