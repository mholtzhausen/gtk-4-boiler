# relm4-kit — TODO

> **Status key:** ⬜ Not started | 🔄 In progress | ✅ Done | ❌ Blocked

---

## Phase 0: Project Scaffolding & Toolchain

### 0.0 — Pi tooling (extensions)

- [x] Create plan-mode extension at `.pi/extensions/plan-mode/`
  - `/plan` command, `Ctrl+Alt+P` shortcut, `--plan` flag
  - Read-only exploration mode with bash allowlist
  - Plan extraction from `Plan:` sections with `[DONE:n]` tracking
  - Progress widget + session persistence
- [x] Create brainstorm-mode extension at `.pi/extensions/brainstorm-mode/`
  - `/brainstorm` command, `Ctrl+Alt+B` shortcut, `--brainstorm` flag
  - Interactive `question` tool — LLM calls it to get user decisions via ↑↓ select UI
  - System prompt instructs LLM to use `question` for every decision point
  - Bash restricted to read-only allowlist
  - State persistence, context filtering, footer status indicator
  - 7 passing tests for `isSafeCommand()`

### 0.1 — Workspace root

- [x] Create root `Cargo.toml` with `[workspace]` members: `components`, `demo` (template is not a Rust crate)
- [x] Add shared `[profile.release]` settings (LTO, codegen-units=1)
- [x] Create `.gitignore` (standard Rust + macOS/Linux/Windows ignores)
- [x] Create root `README.md` with project overview, badges, quick-start

### 0.2 — components/ crate (relm4-kit)

- [x] Run `cargo init --lib components/` → name it `relm4-kit`
- [x] Add `Cargo.toml` dependencies:
  - `relm4 = { version = "0.9", features = ["libadwaita"] }`
  - `relm4-components`
  - `gtk4`
  - `libadwaita`
  - `glib`
  - `log`
- [x] Create `components/src/lib.rs` with module declarations:
  - `pub mod prelude`
  - `pub mod theme`
  - `pub mod primitives`
  - `pub mod containers`
- [x] Create `components/src/prelude.rs` — re-export placeholder
- [x] Create stub files for all modules so `cargo check --workspace` compiles

### 0.3 — demo/ binary

- [x] Run `cargo init --bin demo/`
- [x] Add `Cargo.toml` dependency: `relm4-kit = { path = "../components" }`
- [x] Create `demo/src/main.rs` with a minimal GTK4 window (placeholder)

### 0.4 — template/ directory

- [x] Create `template/cargo-generate.toml` with template variables:
  - `project-name` (default: "my-app")
  - `app-id` (default: "com.{{project-name}}")
  - `version` (default: "0.1.0")
  - `author`
- [x] Create `template/template/` structure (contents filled in Phase 5)

### 0.5 — CI & tooling

- [x] Create `.github/workflows/ci.yml`:
  - `check` job: `cargo check --workspace`
  - `lint` job: `cargo clippy --workspace -- -D warnings`
  - `fmt` job: `cargo fmt --check`
  - `build-demo` job: `cargo build --release -p demo`
  - `test` job: `cargo test --workspace`
- [x] Create `justfile`:
  - `dev`: `cargo run -p demo`
  - `check`: `cargo check --workspace`
  - `lint`: `cargo clippy --workspace -- -D warnings`
  - `fmt`: `cargo fmt`
  - `fmt-check`: `cargo fmt --check`
  - `docs`: `cargo doc --open -p relm4-kit`
  - `test`: `cargo test --workspace`
  - `build`: `cargo build --workspace`
  - `build-demo`: `cargo build --release -p demo`
  - `new`: `./scripts/new-project.sh`
- [x] Create `scripts/dev.sh`, `scripts/setup.sh`, `scripts/new-project.sh` (stubs)

**Phase 0 done when:** `cargo check --workspace` compiles cleanly.

---

## Phase 1: Theme System

### 1.1 — Design tokens (Rust side)

- [x] Create `components/src/theme/tokens.rs`:
  - `ColorTokens` struct with fields for primary, surface, text, accent, danger, warning
  - `SpacingTokens` struct (xs, sm, md, lg, xl)
  - `RadiiTokens` struct (sm, md, lg, xl)
  - `ShadowTokens` struct (sm, md, lg)
  - `TypographyTokens` struct (font sizes sm through 2xl)
  - `Theme` struct containing all above
  - `impl Default for Theme` — the light mode defaults
  - `impl Theme { pub fn dark() -> Self }` — dark mode overrides

### 1.2 — CSS variables file

- [x] Create `components/src/theme/theme.css`:
  - `:root { ... }` with all CSS variables matching the Rust tokens
  - `:root.dark { ... }` with dark mode overrides
  - Pre-styled classes for each primitive: `.relm4-card`, `.relm4-btn-primary`, etc.
  - Base element styles (body text, selection color, scrollbar)

### 1.3 — Embed CSS at compile time

- [x] Embed via `include_str!` in `theme/mod.rs`:
  - Replaced `css.rs` hardcoded string with `include_str!("theme.css")`
  - The full `theme.css` (variables + all component styles) is now the single source of truth
  - No build.rs needed

### 1.4 — Theme initialization

- [x] Create `components/src/theme/mod.rs`:
  - `pub fn init()` — loads embedded CSS into `CssProvider`, attaches to `StyleContext` at `APPLICATION` priority
  - `pub fn init_with_overrides(css: &str)` — same but appends extra CSS
  - `pub struct DarkModeWatcher` — relm4 component that:
    - Connects to `adw::StyleManager::default().is_dark()` property
    - Toggles `.dark` class on the root window
  - `DarkModeWatcher::new()` → returns `Controller<DarkModeWatcher>`

### 1.5 — Verify theme works

- [x] Write unit tests in `theme/mod.rs` verifying:
  - Default token values are correct
  - Dark mode overrides differ from light defaults
- [x] Write integration test in `tests/theme_integration.rs` verifying:
  - `THEME_CSS` parses as valid CSS via `CssProvider::load_from_data`
  - `theme::init()` runs without panic
  - `theme::init_with_overrides()` runs without panic
  - `init()` attaches the theme CSS to the display (widget CSS classes work)
  - All expected CSS variables are present in the stylesheet (`--color-*`, `--spacing-*`, `--radius-*`, `--font-*`, `--shadow-*`)
  - `:root.dark` overrides exist
  - Component class names are present (`.relm4-card`, `.relm4-btn-primary`)

**Phase 1 done when:** `theme::init()` can be called and dark mode toggles via `StyleManager`.

---

## Phase 2: Primitives (Stateless Widgets)

### 2.1 — ButtonAction (helper data struct)

- [x] Create `components/src/primitives/action.rs`:
  - `pub enum ActionKind { Primary, Secondary, Danger }`
  - `pub struct ButtonAction<Msg> { label: String, kind: ActionKind, on_activate: Msg }`
  - `impl<Msg> ButtonAction<Msg> { pub fn new(label, msg) -> Self, pub fn primary(label, msg) -> Self, pub fn danger(label, msg) -> Self }`
- [x] Export from `primitives/mod.rs`

### 2.2 — Card

- [x] Create `components/src/primitives/card.rs`:
  - `pub enum CardStyle { Flat, Elevated, Outlined }`
  - `pub struct Card` — builder pattern, returns `gtk::Box`
  - Builder methods:
    - `.title(&str)` → `gtk::Label`
    - `.subtitle(&str)` → optional subtitle
    - `.style(CardStyle)` → CSS class
    - `.child(&impl IsA<gtk::Widget>)` → content area
    - `.footer(&[ButtonAction<Msg>])` → action buttons at bottom
    - `.build()` → returns `gtk::Box`
  - CSS classes: `.relm4-card`, `.card-flat`, `.card-elevated`, `.card-outlined`, `.card-title`, `.card-subtitle`, `.card-footer`
  - CSS in `theme.css`: padding, background, border-radius, shadow per variant
- [x] Export from `primitives/mod.rs`

### 2.3 — Button

- [x] Create `components/src/primitives/button.rs`:
  - `pub enum ButtonVariant { Primary, Secondary, Ghost, Danger, Link }`
  - `pub enum ButtonSize { Small, Medium, Large }`
  - `pub struct Button` — wraps `gtk::Button`, returns `gtk::Button`
  - Builder methods:
    - `.label(&str)`
    - `.variant(ButtonVariant)`
    - `.size(ButtonSize)`
    - `.icon(&str)` → adds a `gtk::Image` before the label
    - `.on_click(Msg)` → connects to `connect_clicked`
    - `.build()` → returns `gtk::Button`
  - CSS classes: `.relm4-btn`, `.relm4-btn-primary`, `.relm4-btn-small`, `.btn-icon`
- [x] Export from `primitives/mod.rs`

### 2.4 — Toggle

- [x] Create `components/src/primitives/toggle.rs`:
  - `pub struct Toggle` — wraps `gtk::Box` with `gtk::Switch`
  - Builder:
    - `.title(&str)`
    - `.description(&str)`
    - `.active(bool)`
    - `.on_toggle(impl Fn(bool) -> Msg)`
    - `.build()`
  - CSS classes: `.relm4-toggle`
- [x] Export from `primitives/mod.rs`

### 2.5 — Badge

- [x] Create `components/src/primitives/badge.rs`:
  - `pub enum BadgeVariant { Success, Danger, Warning, Info, Neutral }`
  - `pub enum BadgeSize { Small, Medium }`
  - `pub struct Badge` — returns `gtk::Label` styled
  - Builder:
    - `.text(&str)`
    - `.variant(BadgeVariant)`
    - `.size(BadgeSize)`
    - `.build()`
  - CSS classes: `.relm4-badge`, `.badge-success`, `.badge-danger`
- [x] Export from `primitives/mod.rs`

### 2.6 — Avatar

- [x] Create `components/src/primitives/avatar.rs`:
  - `pub struct Avatar` — returns `AdwAvatar`
  - Builder:
    - `.initials(&str)`
    - `.icon(&str)` → overrides initials
    - `.size(i32)`
    - `.build()`
  - CSS classes: `.relm4-avatar`
- [x] Export from `primitives/mod.rs`

### 2.7 — Primitives prelude

- [x] Update `components/src/prelude.rs` to re-export all primitives

**Phase 2 done when:** All primitives compile, have CSS, and can be instantiated via the builder pattern.

---

## Phase 3: Containers (Stateful Components)

### 3.1 — NavItem (data struct)

- [x] Create `components/src/containers/nav_item.rs`:
  - `pub struct NavItem { pub label: String, pub icon: Option<String>, pub id: String, pub badge: Option<u32>, pub children: Vec<NavItem>, pub section: bool }`
  - `impl NavItem { pub fn new(label, icon, id) -> Self, pub fn with_badge(self, u32) -> Self, pub fn with_children(self, Vec<NavItem>) -> Self, pub fn section(label) -> Self }`
- [x] Export from `containers/mod.rs`

### 3.2 — Sidebar

- [ ] Create `components/src/containers/sidebar.rs`:
  - `pub struct SidebarModel { items: Vec<NavItem>, active_id: Option<String>, collapsed: HashSet<String> }`
  - `pub enum SidebarMsg { ItemSelected(String), ToggleCollapse(String) }`
  - `pub enum SidebarOutput { Navigate(String) }`
  - Uses `gtk::ListBox` (simple) or `gtk::ColumnView` (more complex) for rendering
  - Each item shows: icon (if present), label, badge (if present)
  - Active item has highlight, children are indented or collapsible
  - `impl SimpleComponent for Sidebar`
  - CSS: `.relm4-sidebar`, `.relm4-sidebar-item`, `.relm4-sidebar-item--active`, `.relm4-sidebar-section`
- [ ] Export from `containers/mod.rs`

### 3.3 — ToastStack

- [ ] Create `components/src/containers/toast_stack.rs`:
  - `pub enum ToastKind { Success, Error, Warning, Info }`
  - `struct ToastEntry { id: u32, message: String, kind: ToastKind, action_label: Option<String>, action_msg: Option<Msg> }`
  - `pub struct ToastStackModel { toasts: Vec<ToastEntry>, next_id: u32 }`
  - `pub enum ToastStackMsg { Show(ToastEntry), Dismiss(u32), DismissTop }`
  - Uses `gtk::Overlay` positioned at bottom-right
  - Each toast auto-dismisses after 4 seconds (glib::timeout_add_seconds_local)
  - Stack shows up to 3 toasts, oldest is dismissed when >3
  - Static API: `ToastStack::show(message, kind)` — uses global sender or component reference
  - CSS: `.relm4-toast`, `.toast-success`, `.toast-error`, `.toast-warning`, `.toast-info`
- [ ] Export from `containers/mod.rs`

### 3.4 — AppShell

- [ ] Create `components/src/containers/shell.rs`:
  - `pub struct AppShell { window: AdwApplicationWindow, header: AdwHeaderBar, sidebar: Controller<Sidebar>, content: gtk::Stack, toasts: Controller<ToastStack>, dark_mode: DarkModeWatcher }`
  - `pub struct AppShellBuilder { title, width, height, sidebar_items, on_navigate_cb, ... }`
  - `impl AppShellBuilder { pub fn title(), pub fn size(), pub fn sidebar(), pub fn on_navigate(), pub fn build() -> Controller<AppShell> }`
  - Layout: `AdwNavigationSplitView` or `gtk::Paned` with sidebar left + content right
  - Header has: window title, optional search button, window controls (libadwaita handles this)
  - ToastStack overlaid on top of content area
  - DarkModeWatcher instantiated automatically
  - CSS: `.relm4-shell`, `.relm4-header`
- [ ] Export from `containers/mod.rs`

### 3.5 — Dialog

- [ ] Create `components/src/containers/dialog.rs`:
  - Wraps `adw::AlertDialog`
  - `pub struct Dialog;` — static methods, not a managed component:
    - `Dialog::confirm(title, description) -> DialogBuilder`
    - `Dialog::info(title, description) -> DialogBuilder`
    - `DialogBuilder::confirm_label(&str)`, `.cancel_label(&str)`, `.dangerous(bool)`
    - `.on_confirm(Msg)`, `.on_cancel(Msg)`
    - `.present(parent_window)` — shows the dialog
  - No CSS needed (wraps libadwaita's styled dialog)
- [ ] Export from `containers/mod.rs`

### 3.6 — EmptyState

- [ ] Create `components/src/containers/empty_state.rs`:
  - `pub struct EmptyState` — builder pattern, returns `gtk::Box`
  - Builder:
    - `.icon(&str)` — symbolic icon name
    - `.title(&str)`
    - `.description(&str)`
    - `.action(label, Msg)` — optional single action button
    - `.build()`
  - CSS: `.relm4-empty-state`, `.relm4-empty-state-icon`, `.relm4-empty-state-title`, `.relm4-empty-state-description`
- [ ] Export from `containers/mod.rs`

### 3.7 — SearchBar

- [ ] Create `components/src/containers/search_bar.rs`:
  - `pub struct SearchBarModel { query: String, results: Vec<SearchResult>, debounce_source: Option<glib::SourceId>, is_focused: bool }`
  - `pub enum SearchBarMsg { QueryChanged(String), ResultSelected(usize), FocusGained, FocusLost, Dismiss }`
  - Uses `gtk::SearchEntry` + popover dropdown
  - Debounce timer: 300ms default, configurable
  - Keyboard nav: Up/Down arrows to select result, Enter to confirm, Escape to dismiss
  - `SearchBar::builder()` → `Controller<SearchBar>`
  - CSS: `.relm4-search-bar`, `.relm4-search-results`, `.relm4-search-result`, `.relm4-search-result--selected`
- [ ] Export from `containers/mod.rs`

### 3.8 — TreeView

- [ ] Create `components/src/containers/tree_view.rs`:
  - `pub trait TreeItem { fn columns(&self) -> Vec<gtk::Widget>; fn children(&self) -> Vec<Box<dyn TreeItem>>; fn is_expanded() -> bool }`
  - `pub struct TreeViewModel { columns: Vec<ColumnDef>, items: Vec<Box<dyn TreeItem>>, selected: Option<gtk::TreePath> }`
  - `pub enum TreeViewMsg { RowSelected(gtk::TreePath), ToggleExpand(gtk::TreePath), SortChanged(u32, SortDirection) }`
  - Uses `gtk::ColumnView` + `gtk::TreeListModel`
  - Features:
    - Sortable columns (click header)
    - Resizable column widths
    - Row selection (single)
    - Context menu on right-click
  - Builder: `TreeView::new().columns(&[...]).rows(impl IntoIterator).on_select(fn).context_menu(&[...])`
  - CSS: `.relm4-tree-view`, `.relm4-tree-header`, `.relm4-tree-row`, `.relm4-tree-row--selected`, `.relm4-tree-cell`
- [ ] Export from `containers/mod.rs`

### 3.9 — SettingsPanel

- [ ] Create `components/src/containers/settings_panel.rs`:
  - Wraps `AdwPreferencesPage` + `AdwPreferencesGroup`
  - `pub struct SettingsPanel { page: AdwPreferencesPage }`
  - Builder:
    - `.group("Group Name")` → starts a new group
    - `.add(SettingsRow::Toggle("Label", Msg))`
    - `.add(SettingsRow::Dropdown("Label", &[options], Msg))`
    - `.add(SettingsRow::Entry("Label", Msg))`
    - `.add(SettingsRow::Slider("Label", min, max, default, Msg))`
    - `.build()` → returns `AdwPreferencesPage`
- [ ] Export from `containers/mod.rs`

### 3.10 — TabView

- [ ] Create `components/src/containers/tab_view.rs`:
  - Wraps `AdwTabBar` + `AdwTabView`
  - `pub struct TabViewModel { tabs: Vec<TabEntry>, active: u32 }`
  - `pub enum TabViewMsg { SwitchTab(u32), CloseTab(u32), Reorder(u32, u32) }`
  - Builder: `TabView::new().tab("Title", &widget).closable(true).on_switch(fn).build()`
- [ ] Export from `containers/mod.rs`

**Phase 3 done when:** All container components compile and have at least basic tests.

---

## Phase 4: Demo Application

### 4.1 — App shell setup

- [ ] `demo/src/main.rs`:
  - Call `relm4_kit::theme::init()` at startup
  - Create `AppShell` with sidebar pages for each component demo
  - Wire navigation to switch between pages

### 4.2 — Welcome page

- [ ] `demo/src/pages/welcome.rs`:
  - Large title: "relm4-kit"
  - Subtitle: "Curated components for GTK4 + relm4"
  - Quickstart code block showing basic usage
  - Links to docs

### 4.3 — Theme Showcase page

- [ ] `demo/src/pages/theme_showcase.rs`:
  - Color palette grid with swatches and hex values
  - Spacing scale with visual bars
  - Radius scale with corner previews
  - Typography scale
  - Light/dark toggle

### 4.4 — Card Demo page

- [ ] `demo/src/pages/card_demo.rs`:
  - Shows all 3 card styles: Flat, Elevated, Outlined
  - Cards with footer actions
  - Cards with long content (scrollable)
  - Knobs: toggle style, toggle footer visibility
  - Code panel showing the source

### 4.5 — Button Demo page

- [ ] `demo/src/pages/button_demo.rs`:
  - All variants: Primary, Secondary, Ghost, Danger, Link
  - All sizes: Small, Medium, Large
  - With icon, without icon
  - Disabled state
  - Interactive: clicking shows toast

### 4.6 — Toggle Demo page

- [ ] `demo/src/pages/toggle_demo.rs`:
  - Basic toggle
  - Toggle with description
  - State displayed in real-time

### 4.7 — Badge Demo page

- [ ] `demo/src/pages/badge_demo.rs`:
  - All variants: Success, Danger, Warning, Info, Neutral
  - All sizes
  - Badges on icons (mocked)
  - Animated badge count

### 4.8 — Sidebar Demo page

- [ ] `demo/src/pages/sidebar_demo.rs`:
  - Sidebar embedded in content area (not the app shell sidebar)
  - Items with icons, badges, nested children
  - Section headers
  - Collapsible sections

### 4.9 — Toast Demo page

- [ ] `demo/src/pages/toast_demo.rs`:
  - Buttons to trigger: Success, Error, Warning, Info toasts
  - Toast with action button
  - Rapid toast (stacking)
  - Shows toast queue state

### 4.10 — Dialog Demo page

- [ ] `demo/src/pages/dialog_demo.rs`:
  - Confirm dialog (with dangerous action)
  - Info dialog
  - Custom content dialog
  - Shows result of dialog action

### 4.11 — EmptyState Demo page

- [ ] `demo/src/pages/empty_state_demo.rs`:
  - Basic empty state
  - With action button
  - Different icons

### 4.12 — SearchBar Demo page (post-MVP)

- [ ] `demo/src/pages/search_bar_demo.rs`

### 4.13 — TreeView Demo page (post-MVP)

- [ ] `demo/src/pages/tree_view_demo.rs`

### 4.14 — Settings Demo page (post-MVP)

- [ ] `demo/src/pages/settings_demo.rs`

### 4.15 — TabView Demo page (post-MVP)

- [ ] `demo/src/pages/tab_view_demo.rs`

**Phase 4 done when:** `cargo run -p demo` opens a window with all MVP pages navigable.

---

## Phase 5: cargo-generate Template

### 5.1 — Template metadata

- [ ] `template/cargo-generate.toml`:
  - `[placeholders]` for project-name, app-id, version, author
  - `[template]`: `ignore = [".git"]`

### 5.2 — Template Cargo.toml

- [ ] `template/template/Cargo.toml`:
  - Package name = `{{project-name}}`
  - Version = `{{version}}`
  - Dependency: `relm4-kit = { git = "https://github.com/YOUR_USER/relm4-kit", tag = "v0.1.0" }`
  - Dependency: `relm4` with libadwaita feature

### 5.3 — Template entry point

- [ ] `template/template/src/main.rs`:
  - Minimal structure:
    - `relm4_kit::theme::init()`
    - `RelmApp::new("{{app-id}}")`
    - `AppShell::builder().title("{{project-name}}").sidebar(...).build()`
    - `app.run(shell)`

### 5.4 — Template pages

- [ ] `template/template/src/pages/mod.rs`
- [ ] `template/template/src/pages/dashboard.rs` — welcome card + placeholder
- [ ] `template/template/src/pages/settings.rs` — settings panel stubs

### 5.5 — Template resources

- [ ] `template/template/resources/style.css` — empty, ready for overrides
- [ ] `template/template/build.rs` — minimal (or empty, for future GResource use)
- [ ] `template/template/README.md` — instructions for the new project

**Phase 5 done when:** `cargo generate --path ./template --name test-app` produces a compilable project.

---

## Phase 6: Documentation

### 6.1 — ARCHITECTURE.md

- [ ] How relm4-kit components are structured
- [ ] How to compose components (Controller<T> wiring)
- [ ] How theming works (init, overrides, dark mode)
- [ ] How to add a new page
- [ ] How to create a custom component

### 6.2 — COMPONENTS.md

- [ ] One section per component with:
  - Description and use case
  - Import path
  - All builder methods with signatures
  - 1-2 complete code examples
  - CSS class names and customizable variables

### 6.3 — THEMING.md

- [ ] All design tokens with descriptions
- [ ] How to override CSS variables
- [ ] How to add app-specific CSS
- [ ] Dark mode customization guide
- [ ] Complete CSS variable reference table

### 6.4 — CONTRIBUTING.md

- [ ] How to set up the dev environment
- [ ] Coding standards
- [ ] How to add a new component (checklist)
- [ ] PR process

### 6.5 — Inline rustdoc

- [ ] Every `pub` item has doc comments
- [ ] Doc examples compile as tests
- [ ] `#[doc(alias)]` for discoverability

**Phase 6 done when:** `cargo doc --open -p relm4-kit` shows complete documentation.

---

## Phase 7: Developer Scripts

### 7.1 — dev.sh

- [ ] `scripts/dev.sh`: watches demo and rebuilds on change:
  ```bash
  #!/usr/bin/env bash
  cargo watch -x "run -p demo"
  ```

### 7.2 — new-project.sh

- [ ] `scripts/new-project.sh`: wrapper around cargo-generate:
  ```bash
  #!/usr/bin/env bash
  set -e
  if [ -z "$1" ]; then echo "Usage: $0 <project-name>"; exit 1; fi
  cargo generate --path "$(dirname "$0")/../template" --name "$1"
  ```

### 7.3 — setup.sh

- [ ] `scripts/setup.sh`: checks system deps:
  ```bash
  #!/usr/bin/env bash
  command -v cargo >/dev/null 2>&1 || { echo "Install Rust: https://rustup.rs"; exit 1; }
  pkg-config --exists gtk4 || { echo "Install GTK4 + libadwaita dev packages"; exit 1; }
  echo "All dependencies found."
  ```

**Phase 7 done when:** All scripts are executable and work.

---

## Phase 8: Testing

### 8.1 — Unit tests (per component)

- [ ] Each `primitives/*.rs`: tests for:
  - Default construction succeeds
  - Builder methods set fields correctly
  - `build()` returns correct type
- [ ] Each `containers/*.rs`: tests for:
  - `init()` returns `ComponentParts` with valid root widget
  - `update()` handles all message variants without panic
  - Output messages are sent correctly

### 8.2 — Integration tests

- [ ] `components/tests/shell_integration.rs`:
  - Create AppShell with sidebar items
  - Simulate navigation
  - Verify content page switches
- [ ] `components/tests/toast_integration.rs`:
  - Create ToastStack
  - Send Show messages
  - Verify toasts appear and dismiss

### 8.3 — CI integration

- [ ] CI runs `cargo test --workspace`
- [ ] CI fails on warnings with `-D warnings`
- [ ] Demo builds without errors

**Phase 8 done when:** `cargo test --workspace` passes 100%.

---

## Phase 9: Release

### 9.1 — v0.1.0 release

- [ ] All MVP components implemented (Card, Button, Toggle, Badge, AppShell, Sidebar, ToastStack, Dialog, EmptyState)
- [ ] Demo app complete
- [ ] Template works
- [ ] Documentation written
- [ ] CHANGELOG.md entry for v0.1.0
- [ ] Git tag `v0.1.0`
- [ ] Push to GitHub

### 9.2 — Post-MVP components

- [ ] SearchBar
- [ ] TreeView
- [ ] SettingsPanel
- [ ] TabView
- [ ] Avatar

### 9.3 — Example apps

- [ ] `examples/todo-app/`
- [ ] `examples/file-explorer/`

### 9.4 — Polish

- [ ] Performance optimization (lazy loading, model reuse)
- [ ] Animation/transition pass (smooth sidebar collapse, toast slide-in)
- [ ] Accessibility audit (keyboard nav, screen reader labels)
- [ ] Cross-platform testing (macOS, Windows)

---

## Quick Reference: Build Order

```
Phase 0 ───→ Phase 1 ───→ Phase 2 ───→ Phase 3 ───→ Phase 4 ───→ Phase 5
(scaffold)   (theme)      (primitives)  (containers)  (demo app)   (template)
                                                                      │
                                                                      ▼
                                                              Phase 6 + Phase 7
                                                              (docs + scripts)
                                                                      │
                                                                      ▼
                                                              Phase 8 + Phase 9
                                                              (tests + release)
```

## MVP Checklist (v0.1.0 cut line)

- [ ] Phase 0: ✅ Done
- [ ] Phase 1: ✅ Done
- [ ] Phase 2: Card, Button, Toggle, Badge
- [ ] Phase 3: AppShell, Sidebar, ToastStack, Dialog, EmptyState
- [ ] Phase 4: Welcome, Theme Showcase, Card, Button, Toggle, Badge, Toast, Dialog, EmptyState pages
- [ ] Phase 5: Template scaffolds a compilable app
- [ ] Phase 6: ARCHITECTURE.md, COMPONENTS.md (at least for MVP components)
- [ ] Phase 7: dev.sh, new-project.sh
- [ ] Phase 8: Unit tests for MVP components
- [ ] Phase 9: Release tag
