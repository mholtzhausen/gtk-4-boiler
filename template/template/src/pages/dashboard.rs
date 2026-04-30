//! Dashboard page — welcome card with quick stats.

use relm4_kit::primitives::{Card, CardStyle};

/// Build the dashboard content widget.
pub fn page() -> gtk4::Box {
    // `Card<()>` — no footer actions, so `()` suffices as the message type.
    Card::<()>::new()
        .title("Welcome to {{project-name}}")
        .subtitle("Your new GTK4 + relm4 application, powered by relm4-kit.")
        .style(CardStyle::Elevated)
        .build()
}
