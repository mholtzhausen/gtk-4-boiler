//! Card primitive — a container with title, subtitle, optional footer actions.

use crate::primitives::action::ButtonAction;
use gtk4::prelude::*;

/// The visual style of a card.
pub enum CardStyle {
    Flat,
    Elevated,
    Outlined,
}

/// A card builder that constructs a [`gtk4::Box`].
pub struct Card<Msg> {
    title: Option<String>,
    subtitle: Option<String>,
    style: CardStyle,
    child: Option<gtk4::Widget>,
    footer: Vec<ButtonAction<Msg>>,
}

impl<Msg> Default for Card<Msg> {
    fn default() -> Self {
        Self {
            title: None,
            subtitle: None,
            style: CardStyle::Elevated,
            child: None,
            footer: Vec::new(),
        }
    }
}

impl<Msg> Card<Msg> {
    /// Create a new card builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the card title.
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Set the card subtitle.
    pub fn subtitle(mut self, subtitle: impl Into<String>) -> Self {
        self.subtitle = Some(subtitle.into());
        self
    }

    /// Set the card visual style.
    pub fn style(mut self, style: CardStyle) -> Self {
        self.style = style;
        self
    }

    /// Set the child widget placed in the card's content area.
    pub fn child(mut self, child: &impl IsA<gtk4::Widget>) -> Self {
        self.child = Some(child.clone().upcast());
        self
    }

    /// Set footer action buttons.
    pub fn footer(mut self, actions: Vec<ButtonAction<Msg>>) -> Self {
        self.footer = actions;
        self
    }

    /// Build the card widget.
    pub fn build(self) -> gtk4::Box {
        gtk4::Box::new(gtk4::Orientation::Vertical, 0)
    }
}
