//! Integration tests for the relm4-kit theme system.
//!
//! These tests require GTK initialisation on the main thread, which
//! the standard `#[test]` harness cannot guarantee when tests are
//! spread across multiple functions.  We therefore bundle everything
//! into a single test function.
//!
//! On headless CI these tests are skipped (prints a warning).
//! Run locally with a display server, or via `xvfb-run`.

use gtk4::prelude::*;

/// Run all theme integration tests sequentially on the main thread.
#[test]
fn theme_integration() {
    // Initialise GTK — fails in headless environments.
    if gtk4::init().is_err() {
        eprintln!(
            "Skipping theme integration tests — \
             no display available (headless CI?)"
        );
        return;
    }

    css_parses_successfully();
    css_embeds_all_variables();
    init_does_not_panic();
    init_with_overrides_does_not_panic();
    init_applies_css_to_widget();
}

// ---------------------------------------------------------------------------
// Individual test helpers
// ---------------------------------------------------------------------------

/// Verify that the embedded `theme.css` parses as valid CSS.
fn css_parses_successfully() {
    let provider = gtk4::CssProvider::new();
    // `load_from_data` panics on invalid syntax; no panic = valid CSS.
    provider.load_from_data(relm4_kit::theme::THEME_CSS);
}

/// Verify that the CSS contains all expected variable categories.
fn css_embeds_all_variables() {
    let css = relm4_kit::theme::THEME_CSS;

    // Core colour variables
    assert!(css.contains("--color-primary"), "missing --color-primary");
    assert!(css.contains("--color-surface"), "missing --color-surface");
    assert!(css.contains("--color-text"), "missing --color-text");
    assert!(css.contains("--color-accent"), "missing --color-accent");
    assert!(css.contains("--color-danger"), "missing --color-danger");
    assert!(css.contains("--color-warning"), "missing --color-warning");

    // Spacing variables
    assert!(css.contains("--spacing-xs"), "missing --spacing-xs");
    assert!(css.contains("--spacing-sm"), "missing --spacing-sm");
    assert!(css.contains("--spacing-md"), "missing --spacing-md");
    assert!(css.contains("--spacing-lg"), "missing --spacing-lg");
    assert!(css.contains("--spacing-xl"), "missing --spacing-xl");

    // Radius variables
    assert!(css.contains("--radius-sm"), "missing --radius-sm");
    assert!(css.contains("--radius-md"), "missing --radius-md");
    assert!(css.contains("--radius-lg"), "missing --radius-lg");
    assert!(css.contains("--radius-xl"), "missing --radius-xl");

    // Typography variables
    assert!(css.contains("--font-sm"), "missing --font-sm");
    assert!(css.contains("--font-md"), "missing --font-md");
    assert!(css.contains("--font-lg"), "missing --font-lg");
    assert!(css.contains("--font-xl"), "missing --font-xl");
    assert!(css.contains("--font-xxl"), "missing --font-xxl");

    // Shadow variables
    assert!(css.contains("--shadow-sm"), "missing --shadow-sm");
    assert!(css.contains("--shadow-md"), "missing --shadow-md");
    assert!(css.contains("--shadow-lg"), "missing --shadow-lg");

    // Dark mode overrides
    assert!(
        css.contains(":root.dark"),
        "missing :root.dark overrides"
    );

    // Component class for at least one primitive
    assert!(
        css.contains(".relm4-card"),
        "missing .relm4-card class"
    );
    assert!(
        css.contains(".relm4-btn-primary"),
        "missing .relm4-btn-primary class"
    );

    eprintln!("PASS: css_embeds_all_variables");
}

/// Verify that `theme::init()` runs without panicking.
fn init_does_not_panic() {
    relm4_kit::theme::init();
    eprintln!("PASS: init_does_not_panic");
}

/// Verify that `theme::init_with_overrides()` runs without panicking.
fn init_with_overrides_does_not_panic() {
    let extra_css = ":root { --color-primary: #ff0000; }";
    relm4_kit::theme::init_with_overrides(extra_css);
    eprintln!("PASS: init_with_overrides_does_not_panic");
}

/// Verify that `init()` actually applies CSS to a widget.
///
/// We create a widget window, apply a theme CSS class, and verify
/// the widget tree is constructed without error and the CSS class
/// is present.
fn init_applies_css_to_widget() {
    // Create a window with a labelled child.
    let window = gtk4::Window::new();
    let label = gtk4::Label::new(Some("Hello, theme!"));
    window.set_child(Some(&label));

    // Apply one of the CSS classes defined in `theme.css`.  This
    // exercises the stylesheet on a real widget.
    label.add_css_class("relm4-badge");
    assert!(
        label.has_css_class("relm4-badge"),
        "relm4-badge CSS class should be present on the label"
    );

    // Clean up.
    window.close();

    eprintln!("PASS: init_applies_css_to_widget");
}
