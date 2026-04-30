//! Theme Showcase page — colour palette, spacing scale, radius previews,
//! typography scale, shadow previews, and a light/dark mode toggle.
//!
//! Each section visually demonstrates the design tokens defined
//! in [`relm4_kit::theme::tokens`].

use gtk4::prelude::*;

// ============================================================================
// Public API
// ============================================================================

/// Build the theme showcase page widget.
pub fn create() -> gtk4::Box {
    // ---- Page-specific and dynamic CSS ----
    //
    // Register all showcase CSS at the display level.  This includes
    // static layout styles (`theme_showcase.css`) and dynamic styles
    // for radius, typography, and shadow previews that are generated
    // in Rust.

    let mut dynamic_css = String::new();

    // Radius preview classes.
    for entry in RADIUS_SCALE {
        dynamic_css.push_str(&format!(
            ".radius-preview-{} {{ border-radius: {}px; }}\n",
            entry.name, entry.radius
        ));
    }

    // Typography scale classes.
    for entry in TYPOGRAPHY_SCALE {
        dynamic_css.push_str(&format!(
            ".type-sample-{} {{ font-size: {}; }}\n",
            entry.name, entry.font_size
        ));
    }

    // Shadow preview classes.
    for entry in SHADOW_PREVIEWS {
        dynamic_css.push_str(&format!(
            ".shadow-preview-{} {{ box-shadow: {}; }}\n",
            entry.name, entry.shadow_css
        ));
    }

    let combined_css = format!(
        "{}\n{}",
        include_str!("theme_showcase.css"),
        dynamic_css
    );

    let provider = gtk4::CssProvider::new();
    provider.load_from_data(&combined_css);
    gtk4::style_context_add_provider_for_display(
        &gtk4::gdk::Display::default()
            .expect("no display for CSS provider"),
        &provider,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    // ---- Page layout ----

    let outer = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    outer.set_vexpand(true);
    outer.set_hexpand(true);
    outer.add_css_class("relm4-page");

    let scrolled = gtk4::ScrolledWindow::new();
    scrolled.set_vexpand(true);
    scrolled.set_hexpand(true);
    scrolled.set_policy(gtk4::PolicyType::Never, gtk4::PolicyType::Automatic);
    outer.append(&scrolled);

    let container = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    container.set_margin_top(24);
    container.set_margin_bottom(48);
    container.set_margin_start(24);
    container.set_margin_end(24);
    container.set_spacing(0);

    // ---- Page header ----

    let title = gtk4::Label::new(Some("Theme Showcase"));
    title.add_css_class("relm4-page-title");
    container.append(&title);

    let desc = gtk4::Label::new(Some(
        "Explore the visual design tokens — colours, spacing, radii, \
         typography, and shadows — that define the relm4-kit theme.",
    ));
    desc.add_css_class("relm4-page-subtitle");
    desc.set_wrap(true);
    desc.set_margin_bottom(16);
    container.append(&desc);

    // ---- Dark Mode Toggle ----

    let toggle_row = create_dark_mode_toggle();
    container.append(&toggle_row);

    // ---- 1. Colour Palette ----

    container.append(&create_section_title(
        "Colour Palette",
        "All theme colours with their hex values.",
    ));

    let color_grid = create_color_palette();
    container.append(&color_grid);

    // ---- 2. Spacing Scale ----

    container.append(&create_section_title(
        "Spacing Scale",
        "Visual bars representing each spacing token width.",
    ));

    let spacing_section = create_spacing_scale();
    container.append(&spacing_section);

    // ---- 3. Border Radius Scale ----

    container.append(&create_section_title(
        "Border Radius",
        "Corner-rounding preview for each radius token.",
    ));

    let radii_section = create_radius_scale();
    container.append(&radii_section);

    // ---- 4. Typography Scale ----

    container.append(&create_section_title(
        "Typography Scale",
        "Font size preview for each typography token.",
    ));

    let type_section = create_typography_scale();
    container.append(&type_section);

    // ---- 5. Shadow Previews ----

    container.append(&create_section_title(
        "Shadow Depths",
        "Box shadows from subtle (sm) to prominent (lg).",
    ));

    let shadow_section = create_shadow_previews();
    container.append(&shadow_section);

    scrolled.set_child(Some(&container));
    outer
}

// ============================================================================
// Section helpers
// ============================================================================

/// Build a section heading + description.
fn create_section_title(title: &str, description: &str) -> gtk4::Box {
    let section = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    section.set_margin_top(16);

    let label = gtk4::Label::new(Some(title));
    label.add_css_class("showcase-section-title");
    label.set_halign(gtk4::Align::Start);
    section.append(&label);

    let desc = gtk4::Label::new(Some(description));
    desc.add_css_class("showcase-section-desc");
    desc.set_halign(gtk4::Align::Start);
    section.append(&desc);

    section
}

// ============================================================================
// Dark Mode Toggle
// ============================================================================

/// Build a row with a switch that toggles `.dark` class on toplevel windows.
fn create_dark_mode_toggle() -> gtk4::Box {
    let row = gtk4::Box::new(gtk4::Orientation::Horizontal, 12);
    row.add_css_class("showcase-toggle-row");
    row.set_halign(gtk4::Align::Start);

    let icon = gtk4::Image::from_icon_name("weather-clear-night-symbolic");
    icon.set_pixel_size(20);
    row.append(&icon);

    let label = gtk4::Label::new(Some("Dark Mode"));
    label.add_css_class("showcase-toggle-label");
    row.append(&label);

    let toggle = gtk4::Switch::new();
    toggle.set_active(false);
    toggle.set_valign(gtk4::Align::Center);
    toggle.set_halign(gtk4::Align::End);
    toggle.set_hexpand(true);

    // When toggled, add or remove the `dark` CSS class on all toplevel windows.
    // The signal handler stays alive for the lifetime of the switch widget.
    toggle.connect_active_notify(move |sw| {
        let is_dark = sw.is_active();
        for widget in gtk4::Window::list_toplevels() {
            if is_dark {
                widget.add_css_class("dark");
            } else {
                widget.remove_css_class("dark");
            }
        }
    });

    row.append(&toggle);
    row
}

// ============================================================================
// Colour Palette
// ============================================================================

/// Colour entry for the palette grid.
struct ColorEntry {
    name: &'static str,
    hex: &'static str,
    swatch_class: &'static str,
}

const COLOR_PALETTE: &[ColorEntry] = &[
    ColorEntry { name: "Primary", hex: "#3584e4", swatch_class: "swatch-primary" },
    ColorEntry { name: "Primary Hover", hex: "#2a6fc7", swatch_class: "swatch-primary-hover" },
    ColorEntry { name: "Surface", hex: "#ffffff", swatch_class: "swatch-surface" },
    ColorEntry { name: "Surface Secondary", hex: "#f6f5f4", swatch_class: "swatch-surface-secondary" },
    ColorEntry { name: "Background", hex: "#f0f0f0", swatch_class: "swatch-background" },
    ColorEntry { name: "Text", hex: "#1a1a1a", swatch_class: "swatch-text" },
    ColorEntry { name: "Text Secondary", hex: "#5e5c64", swatch_class: "swatch-text-secondary" },
    ColorEntry { name: "Accent", hex: "#33d17a", swatch_class: "swatch-accent" },
    ColorEntry { name: "Danger", hex: "#e66156", swatch_class: "swatch-danger" },
    ColorEntry { name: "Warning", hex: "#f6d32d", swatch_class: "swatch-warning" },
];

/// Build a 2-column grid of colour swatch cards.
fn create_color_palette() -> gtk4::Grid {
    let grid = gtk4::Grid::new();
    grid.add_css_class("showcase-color-grid");
    grid.set_column_spacing(12);
    grid.set_row_spacing(12);
    grid.set_halign(gtk4::Align::Start);
    grid.set_margin_bottom(8);

    for (i, entry) in COLOR_PALETTE.iter().enumerate() {
        let col = (i % 2) as i32;
        let row = (i / 2) as i32;

        let card = create_color_card(entry);
        grid.attach(&card, col, row, 1, 1);
    }

    grid
}

/// Build a single colour swatch card.
fn create_color_card(entry: &ColorEntry) -> gtk4::Box {
    let card = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    card.add_css_class("showcase-color-card");

    // Colour swatch bar.
    let swatch = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    swatch.add_css_class("showcase-color-swatch");
    swatch.add_css_class(entry.swatch_class);
    card.append(&swatch);

    // Name + hex label area.
    let info = gtk4::Box::new(gtk4::Orientation::Vertical, 2);
    info.add_css_class("showcase-color-info");

    let name = gtk4::Label::new(Some(entry.name));
    name.add_css_class("showcase-color-name");
    name.set_halign(gtk4::Align::Start);
    info.append(&name);

    let hex = gtk4::Label::new(Some(entry.hex));
    hex.add_css_class("showcase-color-hex");
    hex.set_halign(gtk4::Align::Start);
    info.append(&hex);

    card.append(&info);
    card
}

// ============================================================================
// Spacing Scale
// ============================================================================

/// Entries for the spacing scale visualisation.
struct SpacingEntry {
    name: &'static str,
    value: &'static str,
    /// Width in pixels for the visual bar (scaled 4× for visibility).
    bar_width: i32,
}

const SPACING_SCALE: &[SpacingEntry] = &[
    SpacingEntry { name: "xs (extra small)", value: "4px", bar_width: 16 },
    SpacingEntry { name: "sm (small)", value: "8px", bar_width: 32 },
    SpacingEntry { name: "md (medium)", value: "16px", bar_width: 64 },
    SpacingEntry { name: "lg (large)", value: "24px", bar_width: 96 },
    SpacingEntry { name: "xl (extra large)", value: "32px", bar_width: 128 },
];

/// Build the spacing scale section with visual bars.
fn create_spacing_scale() -> gtk4::Box {
    let section = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    section.set_margin_bottom(8);

    for entry in SPACING_SCALE {
        let row = create_spacing_row(entry);
        section.append(&row);
    }

    section
}

/// Build a single spacing scale row (label | value | bar).
fn create_spacing_row(entry: &SpacingEntry) -> gtk4::Box {
    let row = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
    row.add_css_class("showcase-spacing-row");
    row.set_valign(gtk4::Align::Center);

    let label = gtk4::Label::new(Some(entry.name));
    label.add_css_class("showcase-spacing-label");
    label.set_halign(gtk4::Align::Start);
    row.append(&label);

    let value = gtk4::Label::new(Some(entry.value));
    value.add_css_class("showcase-spacing-value");
    value.set_halign(gtk4::Align::Start);
    row.append(&value);

    let bar = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    bar.add_css_class("showcase-spacing-bar");
    bar.set_size_request(entry.bar_width, -1);
    bar.set_margin_top(4);
    bar.set_margin_bottom(4);
    row.append(&bar);

    // Push remaining space.
    let spacer = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    spacer.set_hexpand(true);
    row.append(&spacer);

    row
}

// ============================================================================
// Border Radius Scale
// ============================================================================

/// Entries for the radius scale.
struct RadiusEntry {
    name: &'static str,
    value: &'static str,
    radius: i32,
}

const RADIUS_SCALE: &[RadiusEntry] = &[
    RadiusEntry { name: "sm", value: "4px", radius: 4 },
    RadiusEntry { name: "md", value: "8px", radius: 8 },
    RadiusEntry { name: "lg", value: "12px", radius: 12 },
    RadiusEntry { name: "xl", value: "16px", radius: 16 },
];

/// Build the radius scale section with corner-preview boxes.
fn create_radius_scale() -> gtk4::Box {
    let section = gtk4::Box::new(gtk4::Orientation::Horizontal, 20);
    section.set_halign(gtk4::Align::Start);
    section.set_margin_bottom(8);

    for entry in RADIUS_SCALE {
        let item = create_radius_card(entry);
        section.append(&item);
    }

    section
}

/// Build a single radius preview card.
fn create_radius_card(entry: &RadiusEntry) -> gtk4::Box {
    let card = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    card.set_halign(gtk4::Align::Center);

    let box_widget = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    box_widget.add_css_class("showcase-radius-box");
    box_widget.add_css_class(&format!("radius-preview-{}", entry.name));

    card.append(&box_widget);

    let label = gtk4::Label::new(Some(entry.name));
    label.add_css_class("showcase-radius-label");
    card.append(&label);

    let value = gtk4::Label::new(Some(entry.value));
    value.add_css_class("showcase-radius-value");
    card.append(&value);

    card
}

// ============================================================================
// Typography Scale
// ============================================================================

/// Entries for the typography scale.
struct TypeEntry {
    name: &'static str,
    value: &'static str,
    font_size: &'static str,
    sample: &'static str,
}

const TYPOGRAPHY_SCALE: &[TypeEntry] = &[
    TypeEntry { name: "sm", value: "12px", font_size: "12px", sample: "The quick brown fox" },
    TypeEntry { name: "md", value: "14px", font_size: "14px", sample: "The quick brown fox" },
    TypeEntry { name: "lg", value: "16px", font_size: "16px", sample: "The quick brown fox" },
    TypeEntry { name: "xl", value: "20px", font_size: "20px", sample: "The quick brown fox" },
    TypeEntry { name: "2xl", value: "24px", font_size: "24px", sample: "The quick brown fox" },
];

/// Build the typography scale section.
fn create_typography_scale() -> gtk4::Box {
    let section = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    section.set_margin_bottom(8);

    for entry in TYPOGRAPHY_SCALE {
        let row = create_type_row(entry);
        section.append(&row);
    }

    section
}

/// Build a single typography scale row.
fn create_type_row(entry: &TypeEntry) -> gtk4::Box {
    let row = gtk4::Box::new(gtk4::Orientation::Horizontal, 16);
    row.add_css_class("showcase-type-row");
    row.set_halign(gtk4::Align::Fill);

    let name = gtk4::Label::new(Some(entry.name));
    name.add_css_class("showcase-type-name");
    name.set_halign(gtk4::Align::Start);
    row.append(&name);

    let sample = gtk4::Label::new(Some(entry.sample));
    sample.add_css_class("showcase-type-sample");
    sample.add_css_class(&format!("type-sample-{}", entry.name));
    sample.set_halign(gtk4::Align::Start);
    sample.set_hexpand(true);
    row.append(&sample);

    let info = gtk4::Label::new(Some(entry.value));
    info.add_css_class("showcase-type-info");
    info.set_halign(gtk4::Align::End);
    row.append(&info);

    row
}

// ============================================================================
// Shadow Previews
// ============================================================================

/// Entries for the shadow previews.
struct ShadowEntry {
    name: &'static str,
    value: &'static str,
    shadow_css: &'static str,
}

const SHADOW_PREVIEWS: &[ShadowEntry] = &[
    ShadowEntry { name: "sm", value: "0 1px 3px rgba(0,0,0,0.12)", shadow_css: "0 1px 3px rgba(0,0,0,0.12)" },
    ShadowEntry { name: "md", value: "0 4px 12px rgba(0,0,0,0.10)", shadow_css: "0 4px 12px rgba(0,0,0,0.10)" },
    ShadowEntry { name: "lg", value: "0 8px 24px rgba(0,0,0,0.12)", shadow_css: "0 8px 24px rgba(0,0,0,0.12)" },
];

/// Build the shadow preview section.
fn create_shadow_previews() -> gtk4::Box {
    let section = gtk4::Box::new(gtk4::Orientation::Horizontal, 24);
    section.set_halign(gtk4::Align::Start);
    section.set_margin_bottom(8);

    for entry in SHADOW_PREVIEWS {
        let card = create_shadow_card(entry);
        section.append(&card);
    }

    section
}

/// Build a single shadow preview card.
fn create_shadow_card(entry: &ShadowEntry) -> gtk4::Box {
    let card = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    card.set_halign(gtk4::Align::Center);

    let box_widget = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    box_widget.add_css_class("showcase-shadow-box");
    box_widget.add_css_class(&format!("shadow-preview-{}", entry.name));

    // Inner label to give the box content.
    let inner = gtk4::Label::new(Some(entry.name));
    inner.set_opacity(0.4);
    box_widget.append(&inner);

    card.append(&box_widget);

    let label = gtk4::Label::new(Some(entry.name));
    label.add_css_class("showcase-shadow-label");
    card.append(&label);

    let value = gtk4::Label::new(Some(entry.value));
    value.add_css_class("showcase-shadow-value");
    value.set_wrap(true);
    value.set_max_width_chars(24);
    value.set_halign(gtk4::Align::Center);
    card.append(&value);

    card
}
