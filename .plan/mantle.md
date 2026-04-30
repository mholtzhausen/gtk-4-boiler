# Mantle — AI Coding Config Manager

> **Status:** Design phase | **Target:** v0.1.0 MVP | **Role:** Demo app for relm4-kit

---

## Product Description

Mantle is a desktop application for discovering, visualizing, and editing AI coding assistant configuration files. Run it from any project folder and it instantly shows you every configuration file that tools like Claude Code, GitHub Copilot, Cursor, Windsurf, Aider, Pi, and Continue.dev will read — organized by product, layer (global/project/folder), and displayed as a computed "effective config" with source-level blame annotations.

### The Problem

The AI coding tool ecosystem has exploded. Every tool has its own config file conventions:

| Tool | Config File(s) | Layer Model |
|------|---------------|-------------|
| **Claude Code** | `CLAUDE.md`, `.claude/rules.md` | global + project + folder (appended) |
| **GitHub Copilot** | `.github/copilot-instructions.md` | global + repo (flat) |
| **Cursor** | `.cursorrules`, `.cursor/rules/*.mdc` | project + folder (override) |
| **Windsurf** | `.windsurfrules` | project (single file) |
| **Aider** | `.aider.conf.yml`, `CONVENTIONS.md` | global + project (merge) |
| **Continue.dev** | `.continue/config.json`, `.continue/rules/` | global + project (merge) |
| **Pi** | `CLAUDE.md`, `.pi/` directory | global + project (appended) |
| **Cline/Roo** | `.clinerules`, `ROO.md` | project |
| **Codex CLI** | `CODEX.md`, `.codex/` | global + project |

A developer using 3-4 of these tools has config files scattered across their home directory, project root, and subdirectories. There is no unified way to:

1. **Discover** what config files exist for a given project
2. **Understand** which files affect which tool
3. **Visualize** the composed result (what does the AI actually see?)
4. **Edit** the right file at the right layer
5. **Validate** that configs are syntactically correct and consistent

Mantle solves all five.

### The Solution

```
┌──────────────────────────────────────────────────────────────────────┐
│  🔍 [~/Projects/my-app ▼]       ⚙️ Settings    🧪 Validate          │
├──────────────┬───────────────────────────────────────────────────────┤
│  PRODUCT     │  [CLAUDE.md — Project Layer]                         │
│  ○ Claude    │  ┌─────────────────────────────────────────────────┐ │
│  ○ Copilot   │  │ # System Prompt                                │ │
│  ● Cursor    │  │                                                 │ │
│  ○ Windsurf  │  │ You are an expert Rust developer. Always        │ │
│  ○ Aider     │  │ prefer idiomatic Rust with proper error         │ │
│  ○ Pi        │  │ handling using anyhow and thiserror.           │ │
│  ○ All       │  │                                                 │ │
│              │  │ Use iterators over explicit loops where          │ │
│  LAYER       │  │ appropriate.                                    │ │
│  ○ Effective │  └─────────────────────────────────────────────────┘ │
│  ● Global    │                                                    │
│  ○ Project   │  [ Unsaved • ]  [ Save ]  [ Show Effective ]       │
│  ○ Folder    │                                                    │
│              │  📁 Project files                                   │
│  📁 Global   │  ├── CLAUDE.md                                      │
│  ├── CLAUDE… │  ├── .claude/                                       │
│  📁 Project  │  │   └── rules.md                                   │
│  ├── CLAUDE… │  ├── .github/                                       │
│  ├── .claude/│  │   └── copilot-instructions.md                    │
│  │   └── ru… │  ├── .cursor/                                       │
│  ├── .cursor/│  │   └── rules/                                     │
│  │   └── ru… │  └── .aider.conf.yml                                │
│  ├── .aider… │                                                    │
│  └── .github…│                                                    │
├──────────────┴──────────────────────────────────────────────────────┤
│  🌙 Dark mode  |  Claude • Cursor • Copilot  |  💾 All saved       │
└─────────────────────────────────────────────────────────────────────┘
```

### Key Features

1. **Product & Layer Filtering** — Select one product and one layer. The tree shows only relevant files. "Effective" is a special synthetic layer showing the composed config.

2. **Effective Config with Blame** — The composed view shows what the AI actually sees, with a blame column identifying which source file + line contributed each line. Blame badges are clickable and navigate directly to the source.

3. **Side-by-Side View** — Effective config on the left, source file on the right. Edits to the source update the effective preview in real time. This is the "Inspect Element" pattern for AI config.

4. **Autosave on Blur + Validate** — When focus leaves an edited file, Mantle validates it (YAML parse, frontmatter syntax, structural rules). Valid → save silently. Invalid → don't save, show problems panel with errors.

5. **Zero-Config Onboarding** — Running Mantle in a project with no AI configs shows a wizard: select your tools, pick layers, and Mantle scaffolds sensible starter configs.

6. **Global Config Management** — Edit global configs (`~/.config/claude/CLAUDE.md`, etc.) from the same interface. Create them if they don't exist via context menu.

7. **Plugin System** — Each AI tool is defined by a declarative JSON descriptor: what files to look for, which globs, what layer strategy (append/override/merge), and how to compute the effective config. Community-contributable.

---

## Core Data Model

### Dimensions

Mantle operates on two independent axes:

| Axis | Values | Description |
|------|--------|-------------|
| **Product** | `Claude`, `Copilot`, `Cursor`, `Windsurf`, `Aider`, `Pi`, `Continue`, `All` | Which AI tool's config landscape to view |
| **Layer** | `Effective`*, `Global`, `Project`, `Folder` | Which config layer to view |

`*` Effective is a **synthetic layer** — it doesn't correspond to a real directory. It is computed by the plugin's composition logic (append/override/merge as declared by the plugin).

### File Discovery

On launch, Mantle determines:
- **Project root** = git root of the CWD (or `--folder` override)
- **Global root** = `~/.config/` (or `$XDG_CONFIG_HOME`, checked per tool)
- **Folder scope** = CWD (or `--folder` override), used to find subfolder-specific rules

For each known product, Mantle globs the relevant paths using patterns declared in the plugin descriptor. All discovered files are indexed and displayed in the tree, filtered by the selected product and layer.

### Effective Config Composition

Each plugin declares a `layering.strategy`:

| Strategy | Behavior | Example Tools |
|----------|----------|---------------|
| `append` | Files concatenated in layer order (global → project → folder) | Claude Code, Pi |
| `override` | Deeper layers replace same-named rules from shallower layers | Cursor |
| `deep-merge` | Structured configs (YAML/JSON) merged recursively | Aider, Continue.dev |

The `compute_effective_config()` function returns an `EffectiveConfig`:

```rust
struct EffectiveConfig {
    lines: Vec<AnnotatedLine>,
}

struct AnnotatedLine {
    text: String,
    source: Option<SourceLocation>,  // None = synthetic (generated by plugin)
    layer: Layer,
}

struct SourceLocation {
    file: PathBuf,    // absolute path
    line: usize,      // 1-indexed
}
```

This data structure powers:
- The blame column in the effective config view
- Click-to-source navigation
- Layer conflict detection
- Diff views between layers

---

## UI Layout (VS Code-Style)

```
┌──────────────────────────────────────────────────────────────────────┐
│  Top Navigation Bar                                                    │
│  [Folder Path ▼]  [Product ▼]  [Layer ▼]  [⚙️ Settings] [🧪 Validate]│
├──────────────┬───────────────────────────────────────────────────────┤
│              │                                                       │
│  Left Panel  │                Right Panel (Editor)                   │
│  (Tree View) │                                                       │
│              │  ┌─────────────────────────────────────────────────┐  │
│  • Product   │  │  Tab 1: CLAUDE.md    Tab 2: rules.md  [+ New]  │  │
│    selector  │  ├─────────────────────────────────────────────────┤  │
│  • Layer     │  │                                                 │  │
│    selector  │  │         Syntax-highlighted editor               │  │
│  • File tree │  │         (GtkSourceView)                         │  │
│    grouped   │  │                                                 │  │
│    by root   │  │                                                 │  │
│              │  └─────────────────────────────────────────────────┘  │
│              │                                                       │
│              │  ┌─────────────────────────────────────────────────┐  │
│              │  │ Problems Panel (collapsible)                    │  │
│              │  │ ❌ CLAUDE.md:15  Unterminated YAML frontmatter  │  │
│              │  │ ⚠️ .cursorrules:1  Unknown directive "strict"  │  │
│              │  └─────────────────────────────────────────────────┘  │
├──────────────┴───────────────────────────────────────────────────────┤
│  Footer Bar                                                            │
│  🌙 Dark mode  |  Claude • Cursor • Copilot  |  4 configs • 3 layers │
└──────────────────────────────────────────────────────────────────────┘
```

### Top Navigation Bar

| Element | Description |
|---------|-------------|
| **Folder Path** | Dropdown showing current workspace folder. Click to change via file picker. Supports `--folder` CLI arg. |
| **Product Selector** | Dropdown: Claude, Copilot, Cursor, Windsurf, Aider, Pi, Continue, All. "All" shows files for all products (grayed out for non-selected product). |
| **Layer Selector** | Dropdown: Effective (default, synthetic), Global, Project, Folder. Add custom path option at bottom. |
| **Settings** | Opens SettingsPanel (app preferences: autosave, theme, plugin management). |
| **Validate** | Runs validation across all open/config files and opens the Problems panel. |

### Left Panel (Tree View)

The tree view shows config files for the selected product and layer:

- **When a real layer is selected** (Global, Project, Folder): Shows the actual filesystem tree rooted at that layer's directory. Files belonging to the selected product are highlighted; other products' files are grayed out but visible (discoverability).
- **When Effective is selected**: Shows a synthetic tree grouped by source layer. Each file node shows a summary of contributed lines. The tree is read-only — clicking a file opens the corresponding real file at the real layer.
- **Context menu** on tree items: Open file, Reveal in File Manager, Copy to Global Layer, Delete File, New Config File for [Product], Show in Effective Config.

### Right Panel (Editor)

- **TabView**: Multiple open files. Each tab shows filename + layer badge + unsaved indicator (dot).
- **Editor**: GtkSourceView with syntax highlighting for Markdown (with frontmatter), YAML, JSON, TOML, and plain text.
- **Side-by-Side toggle**: When enabled, the right panel splits horizontally — effective config on the left, source file on the right. Edits to the source update the effective preview.

### Problems Panel (Bottom, Collapsible)

Shows validation errors and warnings grouped by file:

```
[❌ Problems: 2 errors, 1 warning]

CLAUDE.md:15      error   Unterminated YAML frontmatter string
.cursorrules:1    error   Unknown directive "mode: strict"
CLAUDE.md:1       warn    File doesn't start with "# System Prompt"
```

Each entry is clickable — jumps to the file and line.

### Footer Bar

| Element | Description |
|---------|-------------|
| **Dark Mode toggle** | Icon button to switch light/dark (syncs with system default). |
| **Product indicators** | Icons/badges for each product detected in the current project. |
| **Config summary** | "4 configs • 3 layers" — quick status. |
| **Save status** | "All saved" or "Unsaved changes in 2 files". |

---

## Click-Through Flow (Effective → Source)

This is the "Inspect Element" pattern for AI config:

1. User selects **Effective** layer for **Claude Code**
2. Tree shows composed config with blame column
3. User sees a line they want to change
4. Clicks the blame badge: `project/.claude/rules.md:22`
5. Layer selector **auto-switches to Project**
6. Tree view shows the real filesystem under Project
7. `.claude/rules.md` is **auto-opened** and **scrolled to line 22**
8. A breadcrumb appears: "Navigated from Effective → Project (rules.md:22)"
9. A "Back to Effective" button restores the previous state
10. User edits, blurs, saves

This flow makes the effective config not just informative but **actionable**.

---

## Autosave on Blur + Validate

The save system balances convenience with safety:

```
User edits file → User clicks away (blur) → Mantle validates:

  ✅ Valid   → Save silently. Brief toast: "Saved" (1s, auto-dismiss)
  ❌ Invalid → DON'T SAVE. Toast: "Validation failed — 2 errors"
               Tab shows red dot (unsaved indicator)
               Problems panel slides up showing errors
               User can fix or explicitly "Save Anyway" from menu
```

**Edge cases:**
- **Closing app with unsaved changes** → Dialog: "You have unsaved changes in 3 files. Discard? Review? Cancel?"
- **Multiple rapid blurs** → Debounce validation (300ms after last blur)
- **Editing in effective config view** → Read-only; blur navigates to source file
- **Manual save option** → Settings toggle "Disable autosave" for power users

---

## Plugin System

### Tiered Architecture

| Tier | Mechanism | Use Case | Ship Strategy |
|------|-----------|----------|---------------|
| **1 (MVP)** | Compiled-in Rust traits | Core tools: Claude, Copilot, Cursor | Bundled with Mantle |
| **2 (v2)** | Declarative JSON descriptors | Community tools: any tool with standard file globs + append/override/merge strategy | Loaded from `~/.config/mantle/tools/*.json` |
| **3 (future)** | WASM plugins | Custom composition logic beyond append/override/merge | Sandboxed runtime loading |

### Tier 2 Declarative Format (Goal for Community Contributions)

```json
{
  "name": "Claude Code",
  "version": "1.0",
  "icon": "claude-symbolic",
  "config_files": [
    { "glob": "CLAUDE.md", "layer": "project", "format": "markdown" },
    { "glob": ".claude/**/*.md", "layer": "project", "format": "markdown" },
    { "glob": "~/.config/claude/CLAUDE.md", "layer": "global", "format": "markdown" },
    { "glob": "~/.config/claude/rules.md", "layer": "global", "format": "markdown" }
  ],
  "layering": {
    "strategy": "append",
    "order": ["global", "project"]
  },
  "validation": {
    "rules": [
      { "type": "frontmatter", "file_glob": "*.md", "required": false },
      { "type": "heading", "value": "# System Prompt", "file_glob": "CLAUDE.md", "required": false }
    ]
  }
}
```

### The Plugin Trait (Tier 1 — Rust)

```rust
pub trait AiToolPlugin: Send + Sync {
    fn name(&self) -> &'static str;
    fn config_files(&self) -> Vec<ConfigGlob>;
    fn layering_strategy(&self) -> LayeringStrategy;
    fn compute_effective_config(&self, found_files: &[FoundFile]) -> EffectiveConfig;
    fn validate(&self, file: &ConfigFile) -> Vec<ValidationDiagnostic>;
}
```

---

## Zero-Config Onboarding

When Mantle opens in a project with no AI config files, it shows:

```
┌──────────────────────────────────────────────────────────────────────┐
│                                                                      │
│                         🔍 No AI configs found                       │
│                                                                      │
│    Mantle scanned this folder and found no configuration files       │
│    for any known AI coding tools.                                    │
│                                                                      │
│    ┌──────────────────────────────────────────────────────────────┐  │
│    │  ✨ Generate starter configs                                 │  │
│    │  📁 Open global config directory                              │  │
│    │  📚 Learn about AI coding configs (docs)                     │  │
│    └──────────────────────────────────────────────────────────────┘  │
│                                                                      │
│    Which tools do you use?                                           │
│    ☑ Claude Code    ☐ Copilot    ☑ Cursor                           │
│    ☐ Windsurf       ☐ Aider      ☐ Pi                               │
│    ☐ Continue.dev   ☐ Cline                                         │
│                                                                      │
└──────────────────────────────────────────────────────────────────────┘
```

Clicking "Generate starter configs":
1. User selects tools from a checklist
2. User picks layers (project root? global? both?)
3. Mantle detects project language (Cargo.toml → Rust, etc.)
4. Scaffolds default config files with language-appropriate rules
5. Tree view populates immediately

The CLI equivalent: `mantle init` does the same thing from the terminal.

---

## CLI Interface

```
mantle                      # Opens GUI in current directory
mantle ~/Projects/my-app    # Opens GUI in specified directory
mantle --folder src/lib     # Opens with folder scope override
mantle init                 # Scaffold configs interactively (CLI wizard)
mantle init --tools claude,cursor  # Scaffold for specific tools
mantle --help               # Show help
```

---

## Growth Path

### v0.1.0 — MVP
- [ ] Product selection: Claude Code, GitHub Copilot, Cursor, Pi
- [ ] Layer selection: Effective, Global, Project, Folder
- [ ] File discovery via compiled-in plugin descriptors
- [ ] Tree view with file listing, grouped by layer root
- [ ] GtkSourceView editor with syntax highlighting (Markdown, YAML, JSON)
- [ ] Effective config view with blame column
- [ ] Click-through navigation (effective → source)
- [ ] Autosave on blur with validation
- [ ] Zero-config onboarding wizard
- [ ] Problems panel (validation errors)
- [ ] Dark mode
- [ ] CLI: `mantle`, `mantle <path>`, `mantle --folder`

### v0.2.0 — Community & Depth
- [ ] Declarative JSON plugin format (Tier 2)
- [ ] Plugin directory: `~/.config/mantle/tools/`
- [ ] Global config sync (opt-in git repo)
- [ ] Diff view between layers
- [ ] Config health score
- [ ] Cross-tool consistency checking
- [ ] Settings panel (app preferences)

### v0.3.0 — Power Features
- [ ] WASM plugin support (Tier 3)
- [ ] Rule snippet insertion (common patterns)
- [ ] Drag-and-drop layer reordering
- [ ] Multi-root workspace support
- [ ] File watcher for external changes (with diff notification)
- [ ] `mantle init` CLI wizard

---

## Component-to-Mantle Mapping

Every relm4-kit component has a genuine role in Mantle:

| Component | Role in Mantle | Priority |
|-----------|---------------|----------|
| **AppShell** | Main window frame with sidebar, dark mode, header/footer | 🔴 MVP |
| **Sidebar** | Product selector + layer selector + file tree navigation | 🔴 MVP |
| **NavItem** | Tree items with validation error badges | 🔴 MVP |
| **TreeView** | Config file tree viewer (real and synthetic) | 🔴 MVP |
| **TabView** | Multiple open config file tabs | 🔴 MVP |
| **Card** | Onboarding wizard cards, config summary cards | 🔴 MVP |
| **Button** | Save, Validate, Show Effective, Generate Configs | 🔴 MVP |
| **Toggle** | Autosave toggle, dark mode toggle in settings | 🔴 MVP |
| **Badge** | Validation error counts, unsaved change indicators, product badges | 🔴 MVP |
| **ToastStack** | Save confirmations, validation pass/fail, error notifications | 🔴 MVP |
| **Dialog** | Unsaved changes on quit, confirm delete, confirm override | 🔴 MVP |
| **EmptyState** | Zero-config onboarding (the core first-run experience) | 🔴 MVP |
| **SearchBar** | Search across all configs, filter tree items | 🟡 v0.2 |
| **SettingsPanel** | App preferences, plugin management, editor settings | 🟡 v0.2 |

---

## Important Design Decisions

### Decision 1: Product × Layer as Independent Axes
**Date:** 2026-04-30  
**Context:** The tree view needed to show files for one product at one layer. Earlier design considered sorting/filtering as a single dimension.  
**Decision:** Two independent dropdowns (Product and Layer). "All" for Product shows files for all products (grayed out for non-selected product). "Effective" is a synthetic layer computed by the plugin.  
**Rationale:** Clean separation. The tree is always showing a real (or synthetic) directory structure filtered by product. No ambiguous sorting.

### Decision 2: Effective Config is a Synthetic Layer
**Date:** 2026-04-30  
**Context:** How to present the composed config — as a separate view, or integrated into the tree?  
**Decision:** Effective is a Layer option in the dropdown. When selected, the tree shows a synthetic grouping by source layer. The editor shows the composed content with blame column.  
**Rationale:** The two-axis model (product × layer) naturally accommodates Effective as just another layer value. No special UI needed — it's just another tree + editor state.

### Decision 3: Blame Badges Are Clickable Links
**Date:** 2026-04-30  
**Context:** How to make the effective config actionable, not just informative.  
**Decision:** Each blame badge (e.g., `project/CLAUDE.md:15`) is clickable. Clicking navigates to the source file at the real layer, auto-switching the layer selector.  
**Rationale:** The "Inspect Element" pattern from browser DevTools is well-understood. It makes the effective view bidirectional — informative AND actionable.

### Decision 4: Autosave on Blur + Validate
**Date:** 2026-04-30  
**Context:** Autosave is convenient but dangerous if it saves a broken config mid-edit.  
**Decision:** Save triggers on focus loss (blur), but only after validation passes. If validation fails, the file is NOT saved and errors are shown in the Problems panel. A "Save Anyway" option exists for power users.  
**Rationale:** Maximum safety without explicit save friction. 90% of edits will be simple and valid. The 10% that break things get caught before they can crash a running AI tool.

### Decision 5: Side-by-Side Effective + Source View
**Date:** 2026-04-30  
**Context:** Is the effective config a separate mode/toggle or a persistent split pane?  
**Decision:** Side-by-side is the primary mode when Effective layer is selected. Left pane shows the composed config (read-only), right pane shows the source file when one is selected. When a real layer is selected, the editor takes full width.  
**Rationale:** The side-by-side view is the killer feature — "see what the AI sees, edit what controls it." But it shouldn't be persistent when it adds no value (editing a real file directly).

### Decision 6: Plugin Architecture is Tiered
**Date:** 2026-04-30  
**Context:** Rust has no stable ABI for dynamic plugins. Need a way to support community contributions without requiring Rust knowledge.  
**Decision:** Three tiers: (1) compiled-in Rust traits for MVP, (2) declarative JSON descriptors for community contributions, (3) WASM for complex logic. Tier 2 is the primary community surface.  
**Rationale:** Reduces friction for contribution. Adding a new AI tool should be as simple as writing a JSON file.

### Decision 7: Zero-Config State is a Wizard, Not a Dead End
**Date:** 2026-04-30  
**Context:** First-run experience when a project has no AI configs.  
**Decision:** The empty state shows a list of detected/available tools, a "Generate starter configs" button that scaffolds appropriate configs based on project language, and links to global config management.  
**Rationale:** Turns "nothing to see here" into a launchpad. The user leaves with a better-configured project than they arrived with.

### Decision 8: CLI as First-Class Entry Point
**Date:** 2026-04-30  
**Context:** How users discover and launch Mantle.  
**Decision:** `mantle` with no args opens in CWD. Path arg overrides. `--folder` overrides folder scope. `mantle init` is a separate CLI flow for scaffolding.  
**Rationale:** Terminal-native developers should feel at home. The GUI is the main experience, but the CLI is the gateway.

### Decision 9: "All Products" Shows Everything
**Date:** 2026-04-30  
**Context:** What does selecting "All" for Product mean?  
**Decision:** "All" shows files for every known product. Files for the most recently selected product are highlighted; all others are visually muted (grayed out). The purpose is discovery — "here's every AI config in your project, even if it doesn't affect the tool you're currently thinking about."  
**Rationale:** Users often don't know what configs exist. "All" reveals the full landscape.

---

## Technical Architecture (Rough Sketch)

### Entry Point
```rust
fn main() {
    // 1. Parse CLI args (folder, --folder, init subcommand)
    // 2. Initialize GTK4 + libadwaita
    // 3. Call relm4_kit::theme::init()
    // 4. Discover workspace (find git root, scan for config files)
    // 5. Initialize plugin registry (load compiled-in plugins)
    // 6. Launch AppShell with MantleModel
    // 7. Run relm4 app
}
```

### Core State (MantleModel)

```rust
struct MantleModel {
    // Workspace
    workspace_root: PathBuf,
    folder_scope: PathBuf,
    
    // Selection
    selected_product: ProductId,
    selected_layer: Layer,
    
    // Discovery
    discovered_files: Vec<DiscoveredFile>,
    products_detected: Vec<ProductId>,
    
    // Editor
    open_tabs: Vec<OpenTab>,
    active_tab: Option<usize>,
    
    // Effective config (cached)
    effective_config: Option<EffectiveConfig>,
    
    // Validation
    problems: Vec<ValidationDiagnostic>,
    
    // App state
    autosave_enabled: bool,
    dark_mode: bool,
}
```

### Message Enum

```rust
enum MantleMsg {
    // Navigation
    SelectProduct(ProductId),
    SelectLayer(Layer),
    SelectTreeItem(TreePath),
    
    // Editor
    FileEdited(FileId, String),       // Content changed
    FileBlurred(FileId),               // Focus left
    TabClosed(TabIndex),
    TabSwitched(TabIndex),
    
    // Actions
    Save(FileId),
    SaveAll,
    Validate,
    ShowEffective,
    ToggleSideBySide,
    
    // Onboarding
    GenerateConfigs(Vec<ProductId>),
    OpenGlobalConfigDir,
    
    // Dialogs
    ConfirmClose(Vec<FileId>),
    ConfirmDelete(FileId),
    
    // System
    DarkModeToggled(bool),
    AutosaveToggled(bool),
    FolderChanged(PathBuf),
    ExternalFileChanged(PathBuf),
}
```

---

## Session Log

### 2026-04-30 — Initial Design Brainstorm

**Participants:** User + AI assistant  
**Context:** Brainstorming the demo app for relm4-kit. The current demo is a single "Welcome" label. The user wants a genuinely useful app, not a kitchen-sink showcase.

**Key decisions made:**
- Rejected generic PIM ("Briefcase") and developer hub ("Workspace") in favor of an AI config manager — forward-thinking, immediately useful to AI-assisted developers.
- Named "Mantle" — evokes "layer beneath the crust" (config) and "cloak" (rules the AI wears). No meaningful conflicts found in the software ecosystem.
- Product × Layer two-axis model established as the core paradigm.
- "Effective" is a synthetic layer, not a separate view mode.
- Blame column is clickable (Inspect Element pattern).
- Side-by-side effective + source view is the primary mode for Effective layer.
- Autosave on blur + validate (not autosave on every keystroke).
- Plugin system is tiered: compiled-in → JSON descriptors → WASM.
- Zero-config state is a wizard, not a dead end.
- CLI is a first-class entry point.
- "All products" shows everything with non-selected products grayed out.

**Open questions deferred:**
- File watching for external changes (v0.3).
- Multi-root workspace support (v0.3).
- Global config sync via git (v0.2).
- Config health score specifics (v0.2).

### 2026-04-30 — Architecture Refinement

- Mapped every relm4-kit component to a Mantle role (see Component-to-Mantle Mapping above).
- Identified missing primitives needed: GtkSourceView-based CodeEditor (not yet in relm4-kit).
- Confirmed that building Mantle will naturally force all relm4-kit components to be implemented for real (currently all are placeholders).
- Established the click-through flow (Effective blame → source file) as the signature UX moment.

### Next Steps
1. Begin implementing relm4-kit component placeholders into real widgets (starting with AppShell, Sidebar, NavItem, TreeView).
2. Prototype the Mantle core model (file discovery, plugin descriptors, effective config computation).
3. Build the onboarding/empty-state flow first — it's the simplest vertical slice that exercises EmptyState, Card, Button, Dialog, and ToastStack.
4. Add GtkSourceView dependency and build the CodeEditor primitive.
