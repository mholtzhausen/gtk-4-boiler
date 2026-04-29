# relm4-kit — Plan

A curated component library & project template for GTK4 + relm4 Rust applications.

**Vision:** Make it fast (token-efficient for AI too) to go from idea to polished, shippable GTK4 application by providing beautiful defaults, reusable components, and a consistent architecture.

---

## Principles

| Principle | Meaning |
|-----------|---------|
| **Beautiful defaults** | Everything looks great out of the box. No CSS required. |
| **Customizable by design** | Theming via CSS variables. Override only what you need. |
| **Composable API** | Every component is a `Controller<T>` — wire them up, done. |
| **Two tiers** | Primitives (stateless) + Containers (stateful with message passing). |
| **AI-first ergonomics** | Small, consistent API surface. Well-documented. Token-efficient. |
| **Opinionated architecture, modular components** | The template enforces a structure. But you can use individual components however you want. |
| **libadwaita-based** | Polished GNOME look. Adaptive layout. Dark mode for free. |

---

## Repository Structure

```
relm4-kit/
├── Cargo.toml                    # Workspace root
├── components/                   # The lib crate (relm4-kit)
│   ├── Cargo.toml
│   ├── build.rs                  # Embed theme CSS at compile time
│   └── src/
│       ├── lib.rs                # Re-exports, pub mod structure
│       ├── prelude.rs            # #[doc(hidden)] — everything users need
│       ├── theme/                # CSS variables, dark mode, init
│       │   ├── mod.rs
│       │   ├── tokens.rs         # Rust-side design token structs
│       │   └── theme.css         # Default CSS variables + classes
│       ├── primitives/           # Simple stateless widgets
│       │   ├── mod.rs
│       │   ├── card.rs
│       │   ├── button.rs
│       │   ├── toggle.rs
│       │   ├── badge.rs
│       │   ├── avatar.rs
│       │   └── action.rs         # ButtonAction data struct
│       └── containers/           # Stateful managed components
│           ├── mod.rs
│           ├── shell.rs          # AppShell — window + sidebar + header + toasts
│           ├── sidebar.rs        # Sidebar — nav list with icons, badges, nesting
│           ├── nav_item.rs       # NavItem data struct
│           ├── toast_stack.rs    # Non-blocking notification stack
│           ├── tree_view.rs      # Multi-column tree/table with sorting, selection
│           ├── search_bar.rs     # Debounced search with dropdown
│           ├── settings_panel.rs # Wraps AdwPreferencesPage/Group with fluent API
│           ├── dialog.rs         # Modal confirm/alert/custom dialogs
│           ├── tab_view.rs       # Tabbed document/workspace views
│           └── empty_state.rs    # Placeholder for empty lists/searches
│
├── demo/                         # Showcase binary
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs               # App shell setup, sidebar pages
│       └── pages/                # One per component
│           ├── mod.rs
│           ├── welcome.rs
│           ├── theme_showcase.rs
│           ├── card_demo.rs
│           ├── button_demo.rs
│           ├── toggle_demo.rs
│           ├── badge_demo.rs
│           ├── sidebar_demo.rs
│           ├── tree_view_demo.rs
│           ├── search_bar_demo.rs
│           ├── settings_demo.rs
│           ├── dialog_demo.rs
│           ├── tab_view_demo.rs
│           └── empty_state_demo.rs
│
├── template/                     # cargo-generate template
│   ├── cargo-generate.toml       # Template variables: project-name, app-id, etc.
│   └── template/
│       ├── Cargo.toml            # Depends on relm4-kit (git)
│       ├── build.rs
│       ├── src/
│       │   ├── main.rs           # Minimal entry point
│       │   ├── app.rs            # AppModel + shell setup
│       │   └── pages/
│       │       ├── mod.rs
│       │       ├── dashboard.rs
│       │       └── settings.rs
│       ├── resources/
│       │   └── style.css         # App-specific CSS overrides (empty initially)
│       └── README.md
│
├── scripts/
│   ├── dev.sh                    # cargo watch the demo app
│   ├── new-project.sh            # cargo generate wrapper
│   └── setup.sh                  # Check system deps
│
├── docs/                         # Extended documentation
│   ├── ARCHITECTURE.md           # How components work, how to compose them
│   ├── COMPONENTS.md             # Full API reference per component
│   ├── THEMING.md                # Design token reference, customization guide
│   └── CONTRIBUTING.md           # How to add a new component
│
├── .github/
│   └── workflows/
│       └── ci.yml                # check, lint, fmt, test, build-demo
│
├── examples/                     # Real-world example apps (post-v0.1.0)
│   ├── todo-app/
│   └── file-explorer/
│
├── CHANGELOG.md
├── README.md
└── justfile                      # Alternative to scripts/
```

---

## Component API Design

### Pattern: Primitive (stateless widget)

Every primitive follows a builder pattern that returns a `gtk::Widget`:

```rust
// Pseudocode — the actual API
Card::new()
    .title("...")
    .subtitle(Option<String>)
    .style(CardStyle::Elevated)          // Flat | Elevated | Outlined
    .child(&some_widget)                  // Content area
    .footer(&[ButtonAction::new(...)])    // Optional action buttons
    .build()
    .upcast::<gtk::Widget>()
```

### Pattern: Container (stateful, managed by relm4)

Every container is a relm4 `Component` that returns a `Controller<Self>`:

```rust
// Pseudocode
AppShell::builder()
    .title("My App")
    .width(1200).height(800)
    .sidebar(vec![
        NavItem::new("Dashboard", "dashboard-symbolic", "dashboard"),
        NavItem::new("Settings", "settings-symbolic", "settings"),
    ])
    .on_navigate(|page_id| Msg::Navigate(page_id))
    .build()
// Returns: Controller<AppShell>
```

### Internal pattern for implementing a container

```rust
// Each container component follows this structure:
pub struct MyComponent {
    // Widget references
    widget: gtk::Box,
    // Child controllers
    // ...
}

pub enum MyComponentMsg {
    // User interactions
}

#[relm4::component(pub)]
impl SimpleComponent for MyComponent {
    type Init = MyComponentInit;  // Builder params
    type Input = MyComponentMsg;
    type Output = MyComponentMsg;  // Messages sent to parent

    view! {
        // ...
    }

    fn init(
        init: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        // Build the widget tree
    }

    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>) {
        // Handle messages
    }
}
```

---

## Theme System

### Design tokens (Rust side, `tokens.rs`)

```rust
pub struct Theme {
    pub colors: ColorTokens,
    pub spacing: SpacingTokens,
    pub radii: RadiiTokens,
    pub typography: TypographyTokens,
    pub shadows: ShadowTokens,
}

pub struct ColorTokens {
    pub primary: &'static str,       // "#3584e4"
    pub primary_hover: &'static str,
    pub surface: &'static str,
    pub surface_secondary: &'static str,
    pub background: &'static str,
    pub text: &'static str,
    pub text_secondary: &'static str,
    pub accent: &'static str,
    pub danger: &'static str,
    pub warning: &'static str,
}
```

- `Theme::default()` → light mode
- `Theme::dark()` → dark mode overrides

### CSS variables (`theme.css`)

```css
:root {
    --color-primary: #3584e4;
    --color-surface: #ffffff;
    --color-text: #1a1a1a;
    --spacing-md: 16px;
    --radius-md: 8px;
    --shadow-md: 0 4px 12px rgba(0,0,0,0.1);
    /* ... ~30 variables total */
}

:root.dark {
    --color-surface: #1a1a1a;
    --color-text: #ffffff;
    /* ... dark overrides */
}
```

### Init function

```rust
pub fn init() {
    // Load embedded CSS into a CssProvider
    // Attach to gtk::StyleContext at APPLICATION priority
    // Works automatically — no user action needed
}

pub fn init_with_overrides(css: &str) {
    // Same as init() but appends additional CSS
}
```

### Dark mode

- Uses `adw::StyleManager` to detect system preference
- Toggles `.dark` class on the root window automatically
- Integrated into `AppShell` by default

---

## Component Catalog

### Primitives (Phase 2)

| Component | File | CSS Class | Description |
|-----------|------|-----------|-------------|
| **Card** | `primitives/card.rs` | `.relm4-card` | Container with title/subtitle, optional footer actions |
| **Button** | `primitives/button.rs` | `.relm4-btn-{variant}` | Styled button: Primary, Secondary, Ghost, Danger, Link |
| **Toggle** | `primitives/toggle.rs` | `.relm4-toggle` | Switch with label and optional description |
| **Badge** | `primitives/badge.rs` | `.relm4-badge` | Count/status label: Success, Danger, Warning, Info |
| **Avatar** | `primitives/avatar.rs` | `.relm4-avatar` | Circular initial/icon display |
| **ButtonAction** | `primitives/action.rs` | — | Data struct consumed by Card, Dialog (not a widget) |

### Containers (Phase 3)

| Component | File | Description |
|-----------|------|-------------|
| **AppShell** | `containers/shell.rs` | Full app frame: window, header bar, sidebar, content area, toast overlay, dark mode |
| **Sidebar** | `containers/sidebar.rs` | Navigation list with icons, badges, collapsible nesting, active state |
| **NavItem** | `containers/nav_item.rs` | Data struct for sidebar items (not a widget) |
| **ToastStack** | `containers/toast_stack.rs` | Stack of auto-dismissing notifications with optional action buttons |
| **TreeView** | `containers/tree_view.rs` | Multi-column tree/table with sorting, selection, resizable columns, context menu |
| **SearchBar** | `containers/search_bar.rs` | Debounced search input with dropdown results and keyboard navigation |
| **SettingsPanel** | `containers/settings_panel.rs` | Wraps AdwPreferencesPage/Group with fluent row-definition API |
| **Dialog** | `containers/dialog.rs` | Modal confirm/alert/custom content dialogs (wraps AdwAlertDialog) |
| **TabView** | `containers/tab_view.rs` | Tabbed document/workspace views (wraps AdwTabBar + AdwTabView) |
| **EmptyState** | `containers/empty_state.rs` | Placeholder: icon + title + description + single action button |

---

## Implementation Phases

### Phase 0: Project Scaffolding & Toolchain

- [x] Workspace `Cargo.toml` with `components/`, `demo/`, `template/` members
- [x] `components/Cargo.toml` with `gtk4`, `libadwaita`, `relm4` (libadwaita feature), `relm4-components`
- [x] `demo/Cargo.toml` with path dependency on relm4-kit
- [x] `.gitignore` (Rust standard)
- [x] CI workflow: check, lint, fmt, build-demo
- [ ] `justfile` with dev commands

### Phase 1: Theme System

- [ ] `theme/tokens.rs` — design token structs + defaults
- [ ] `theme/theme.css` — CSS variables file
- [ ] `build.rs` — embed CSS file
- [ ] `theme/mod.rs` — `init()`, `init_with_overrides()`, `DarkModeWatcher`
- [ ] `init_theme!()` macro (optional, or just call `theme::init()` directly)

### Phase 2: Primitives

- [ ] `primitives/action.rs` — `ButtonAction` + `ActionKind`
- [ ] `primitives/card.rs` — `Card`, `CardStyle`
- [ ] `primitives/button.rs` — `Button`, `ButtonVariant`, `ButtonSize`
- [ ] `primitives/toggle.rs` — `Toggle`
- [ ] `primitives/badge.rs` — `Badge`, `BadgeVariant`
- [ ] `primitives/avatar.rs` — `Avatar`

### Phase 3: Containers

- [ ] `containers/nav_item.rs` — `NavItem`
- [ ] `containers/sidebar.rs` — `Sidebar`
- [ ] `containers/toast_stack.rs` — `ToastStack`
- [ ] `containers/shell.rs` — `AppShell` (depends on Sidebar, ToastStack)
- [ ] `containers/dialog.rs` — `Dialog`
- [ ] `containers/search_bar.rs` — `SearchBar`
- [ ] `containers/tree_view.rs` — `TreeView`, `TreeItem` trait
- [ ] `containers/settings_panel.rs` — `SettingsPanel`, `SettingsRow`
- [ ] `containers/tab_view.rs` — `TabView`
- [ ] `containers/empty_state.rs` — `EmptyState`

### Phase 4: Demo Application

- [ ] `demo/src/main.rs` — App shell with all pages
- [ ] One page per component (see structure above)
- [ ] Page layout: component demo + source code + CSS reference
- [ ] Theme showcase page

### Phase 5: cargo-generate Template

- [ ] `template/cargo-generate.toml`
- [ ] `template/template/Cargo.toml`
- [ ] `template/template/src/main.rs`
- [ ] `template/template/src/pages/dashboard.rs`
- [ ] `template/template/src/pages/settings.rs`
- [ ] `template/template/resources/style.css`

### Phase 6: Documentation

- [ ] `docs/ARCHITECTURE.md`
- [ ] `docs/COMPONENTS.md`
- [ ] `docs/THEMING.md`
- [ ] `docs/CONTRIBUTING.md`
- [ ] Inline rustdoc on every public item

### Phase 7: Developer Scripts

- [ ] `scripts/dev.sh`
- [ ] `scripts/new-project.sh`
- [ ] `scripts/setup.sh`

### Phase 8: Testing

- [ ] Unit tests per component (construction, messages)
- [ ] Integration tests (AppShell + Sidebar + ToastStack)
- [ ] CI verifies demo compiles

### Phase 9: Release

- [ ] Git tag: `v0.1.0`
- [ ] CHANGELOG.md entry
- [ ] Example apps (todo, file-explorer)

---

## MVP Scope (v0.1.0)

The first release includes everything in Phases 0–5 that fits:

| Component | Must-have for MVP? | Reason |
|-----------|-------------------|--------|
| Theme system | ✅ | Every component needs it |
| Card | ✅ | Most common layout primitive |
| Button | ✅ | Most common interactive primitive |
| Toggle | ✅ | Settings/configuration UX |
| Badge | ✅ | Notification counts, status indicators |
| AppShell | ✅ | The template needs it |
| Sidebar | ✅ | Navigation pattern for template |
| ToastStack | ✅ | User feedback is essential |
| Dialog | ✅ | Confirms, alerts, prompts |
| EmptyState | ✅ | Every app has empty lists |
| SearchBar | 🟡 | Important but can be v0.2.0 |
| TreeView | 🟡 | Important but complex, can be v0.2.0 |
| SettingsPanel | 🟡 | Can be built manually with primitives |
| TabView | 🟡 | Niche, can be v0.2.0 |
| Avatar | 🟡 | Nice-to-have, can be v0.2.0 |

**MVP components:** Theme, Card, Button, Toggle, Badge, AppShell, Sidebar, ToastStack, Dialog, EmptyState + demo app + template.

---

## CSS Class Naming Convention

```
.relm4-{component}              # Root element
.relm4-{component}-{variant}     # Style variant
.relm4-{component}--{modifier}   # State: active, disabled, selected
```

Examples:
- `.relm4-card`, `.relm4-card-elevated`, `.relm4-card--active`
- `.relm4-btn-primary`, `.relm4-btn-small`, `.relm4-btn--disabled`
- `.relm4-sidebar-item`, `.relm4-sidebar-item--active`

---

## Key Dependencies

| Crate | Purpose | Version Pin |
|-------|---------|-------------|
| `relm4` | Architecture (MVU components) | Latest (features: `libadwaita`) |
| `relm4-components` | Reusable Adw components (prefs, dialogs) | Latest |
| `gtk4` | Core widget toolkit | Compatible with relm4 |
| `libadwaita` | GNOME design system widgets | Latest |
| `glib` | GResource, clone, and binding utilities | Latest |
| `log` + `pretty_env_logger` | Development logging | Latest |
| `cargo-generate` | Project scaffolding (dev dependency) | Latest |
| `cargo-watch` | Live reload (dev dependency) | Latest |

---

## Build & Dev Commands

```bash
# Build everything (requires PKG_CONFIG_PATH set — see setup note below)
cargo build --workspace

# Run the demo app
cargo run -p demo

# Watch & reload demo
cargo watch -x "run -p demo"

# Check (fast)
cargo check --workspace

# Lint
cargo clippy --workspace -- -D warnings

# Test
cargo test --workspace

# Generate docs
cargo doc --open -p relm4-kit

# Scaffold a new project
./scripts/new-project.sh my-awesome-app
# or: cargo generate --path ./template --name my-awesome-app

# Generate justfile:
just dev
just new my-app
just check
just lint
just docs
```

> **Setup note:** On some systems, `pkg-config` needs `PKG_CONFIG_PATH` to find GTK4/libadwaita:
> ```bash
> export PKG_CONFIG_PATH="/usr/lib/x86_64-linux-gnu/pkgconfig:/usr/share/pkgconfig"
> ```

---

## Session Log

### 2026-04-30 — Phase 1.2 (CSS variables file) + demo fix

**Completed:**
- Created `components/src/theme/theme.css` with:
  - All CSS variables matching Rust design tokens (colors, spacing, radii, shadows, typography)
  - `:root.dark` overrides for dark mode
  - Pre-styled component classes for all 11 primitives/containers
  - Base element styles (text color, selection, scrollbar)

**Bug fixed:**
- Demo's `connect_activate` callback was empty — no window appeared. Fixed to create a proper `ApplicationWindow` with a centered label.

**Discoveries:**
- **libadwaita 0.7 crate naming:** The Rust module is `libadwaita::`, *not* `adw::`. The `adw` shorthand was introduced in libadwaita 1.x. Our `Cargo.toml` pins `libadwaita = "0.7"`, so all code must use `libadwaita::*`.
- **libadwaita 0.7 API gaps:** `ToolbarView`, `NavigationSplitView` do not exist in 0.7. The plan's `AppShell` layout (which references `AdwNavigationSplitView`) will need to use an alternative layout — likely `gtk::Paned` or a simpler `gtk::Box` + `gtk::Stack` approach.
- **CSS embedding strategy:** Using a standalone `.css` file + `include_str!()` in `theme/mod.rs`, not a `build.rs` script. The `css.rs` module currently holds the CSS as a string constant — this should be replaced by `include_str!("theme.css")` in task 1.3.
- **CSS class convention confirmed:** `.relm4-{component}-{variant}` for visual variants, `.relm4-{component}--{modifier}` for states (BEM-style), matching the plan's documented convention.

### 2026-04-30 — Phase 1.3 (Embed CSS at compile time)

**Completed:**
- Replaced `css.rs` hardcoded CSS string with `include_str!("theme.css")` in `theme/mod.rs`
- Removed `css.rs` module — the full `theme.css` file is now the single source of truth
- Confirmed `cargo check --workspace` passes cleanly

**Notes:**
- Task 1.1 (Design tokens) was already fully implemented in Phase 0 but not marked as done in TODO. Updated TODO to reflect actual state.
- The `THEME_CSS` constant is now a `pub const` exported from `theme/mod.rs` for use by theme initialization code in task 1.4.
