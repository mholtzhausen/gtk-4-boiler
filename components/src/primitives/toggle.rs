//! Toggle primitive — a switch with a label and optional description.

/// A toggle (switch) builder.
pub struct Toggle<Msg> {
    title: Option<String>,
    description: Option<String>,
    active: bool,
    on_toggle: Option<Box<dyn Fn(bool) -> Msg>>,
}

impl<Msg> Toggle<Msg> {
    /// Create a new toggle builder.
    pub fn new() -> Self {
        Self {
            title: None,
            description: None,
            active: false,
            on_toggle: None,
        }
    }

    /// Set the toggle title/label.
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Set the toggle description (shown below the title).
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Set the initial active state.
    pub fn active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }

    /// Set the callback for toggle state changes.
    pub fn on_toggle(mut self, f: impl Fn(bool) -> Msg + 'static) -> Self {
        self.on_toggle = Some(Box::new(f));
        self
    }

    /// Build the toggle widget.
    pub fn build(self) -> gtk4::Box {
        gtk4::Box::new(gtk4::Orientation::Horizontal, 0)
    }
}
