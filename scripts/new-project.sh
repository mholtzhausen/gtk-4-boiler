#!/usr/bin/env bash
# Scaffold a new project from the relm4-kit template.
set -e

if [ -z "$1" ]; then
    echo "Usage: $0 <project-name>"
    exit 1
fi

cd "$(dirname "$0")/.."

if ! command -v cargo-generate &>/dev/null; then
    echo "cargo-generate is not installed."
    echo "Install it with: cargo install cargo-generate"
    exit 1
fi

cargo generate --path ./template --name "$1"
