#!/usr/bin/env bash
# Watch the demo and rebuild on changes.
set -e

cd "$(dirname "$0")/.."

if ! command -v cargo-watch &>/dev/null; then
    echo "cargo-watch is not installed."
    echo "Install it with: cargo install cargo-watch"
    exit 1
fi

cargo watch -x "run -p demo"
