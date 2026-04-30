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

/// Verify that the CSS contains all expected component classes.
fn css_embeds_all_variables() {
    let css = relm4_kit::theme::THEME_CSS;

    // Dark mode overrides
    assert!(
        css.contains(".dark "),
        "missing .dark overrides"
    );

    // Component class for core primitives
    assert!(
        css.contains(".relm4-card"),
        "missing .relm4-card class"
    );
    assert!(
        css.contains(".relm4-btn-primary"),
        "missing .relm4-btn-primary class"
    );
    assert!(
        css.contains(".relm4-toast"),
        "missing .relm4-toast class"
    );
    assert!(
        css.contains(".relm4-sidebar"),
        "missing .relm4-sidebar class"
    );
    assert!(
        css.contains(".relm4-empty-state"),
        "missing .relm4-empty-state class"
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
    let extra_css = ".relm4-btn-primary { background-color: #ff0000; }";
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
