# ─── relm4-kit command runner ────────────────────────────────────────────────

# Run the demo app
dev:
    cargo run -p demo

# Quick check
check:
    cargo check --workspace

# Lint with clippy
lint:
    cargo clippy --workspace -- -D warnings

# Format code
fmt:
    cargo fmt

# Format check (CI)
fmt-check:
    cargo fmt --check

# Build docs
docs:
    cargo doc --open -p relm4-kit

# Run tests
test:
    cargo test --workspace

# Build all
build:
    cargo build --workspace

# Build demo release
build-demo:
    cargo build --release -p demo

# Clean all
clean:
    cargo clean

# Scaffold a new project from the template
new project-name="my-app":
    ./scripts/new-project.sh {{project-name}}

# Show available commands
default:
    @just --list
