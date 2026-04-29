//! ButtonAction: a data struct for defining action buttons in cards, dialogs, etc.

/// The visual kind of an action button.
#[derive(Clone)]
pub enum ActionKind {
    Primary,
    Secondary,
    Danger,
}

/// A data struct describing a button action.
///
/// This is not a widget — it's consumed by builders like `Card::footer()`.
pub struct ButtonAction<Msg> {
    pub label: String,
    pub kind: ActionKind,
    pub on_activate: Msg,
}

impl<Msg> ButtonAction<Msg> {
    /// Create a new action with the default (Secondary) kind.
    pub fn new(label: impl Into<String>, msg: Msg) -> Self {
        Self {
            label: label.into(),
            kind: ActionKind::Secondary,
            on_activate: msg,
        }
    }

    /// Create a primary action button.
    pub fn primary(label: impl Into<String>, msg: Msg) -> Self {
        Self {
            label: label.into(),
            kind: ActionKind::Primary,
            on_activate: msg,
        }
    }

    /// Create a danger action button.
    pub fn danger(label: impl Into<String>, msg: Msg) -> Self {
        Self {
            label: label.into(),
            kind: ActionKind::Danger,
            on_activate: msg,
        }
    }
}
