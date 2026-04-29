# relm4-kit

A curated component library & project template for **GTK4 + relm4** Rust applications.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
![Rust](https://img.shields.io/badge/rust-stable-brightgreen)

> **Status:** v0.1.0 — MVP ✨

---

## Overview

**relm4-kit** provides beautiful, reusable UI components and a project template to go from idea to polished GTK4 application fast.

- **Beautiful defaults** — everything looks great out of the box, no CSS required.
- **Customizable by design** — theming via CSS variables, override only what you need.
- **Composable API** — every component is a `Controller<T>` — wire them up, done.
- **Two tiers** — Primitives (stateless widgets) + Containers (stateful with message passing).
- **AI-first ergonomics** — small, consistent API surface, well-documented, token-efficient.
- **libadwaita-based** — polished GNOME look, adaptive layout, dark mode for free.

---

## Quick Start

```bash
# Prerequisites: Rust 1.75+, GTK4 + libadwaita dev packages

# Clone and explore the demo
git clone https://github.com/YOUR_USER/relm4-kit
cd relm4-kit
cargo run -p demo

# Or scaffold a new project
./scripts/new-project.sh my-app
cd my-app
cargo run
```

---

## Components

### Primitives (stateless widgets)

| Component | Description |
|-----------|-------------|
| **Card**    | Container with title/subtitle, optional footer actions |
| **Button**  | Styled button: Primary, Secondary, Ghost, Danger, Link |
| **Toggle**  | Switch with label and optional description |
| **Badge**   | Count/status label: Success, Danger, Warning, Info |
| **Avatar**  | Circular initial/icon display |

### Containers (stateful components)

| Component | Description |
|-----------|-------------|
| **AppShell**     | Full app frame: window, header bar, sidebar, content area, toasts, dark mode |
| **Sidebar**      | Navigation list with icons, badges, collapsible nesting |
| **ToastStack**   | Stack of auto-dismissing notifications with action buttons |
| **Dialog**       | Modal confirm/alert/custom content dialogs |
| **EmptyState**   | Placeholder for empty lists/searches |

---

## Project Structure

```
relm4-kit/
├── Cargo.toml           # Workspace root
├── components/          # The library crate (relm4-kit)
│   ├── src/theme/       # CSS variables, dark mode, init
│   ├── src/primitives/  # Stateless widget builders
│   └── src/containers/  # Stateful managed components
├── demo/                # Showcase binary with all components
├── template/            # cargo-generate project template
├── scripts/             # Developer tooling
└── docs/                # Architecture & component documentation
```

---

## Development

```bash
# Build everything
cargo build --workspace

# Run the demo app
cargo run -p demo

# Check (fast)
cargo check --workspace

# Lint
cargo clippy --workspace -- -D warnings

# Test
cargo test --workspace

# Generate docs
cargo doc --open -p relm4-kit
```

---

## License

MIT
