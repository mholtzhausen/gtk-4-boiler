//! Card primitive — a container with title, subtitle, optional footer actions.

use crate::primitives::action::{ActionKind, ButtonAction};
use gtk4::prelude::*;

/// The visual style of a card.
#[derive(Clone, Copy, PartialEq)]
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
    ///
    /// Returns a vertical [`gtk4::Box`] with the following structure:
    ///
    /// ```ignore
    /// ├── (if title)   Label.card-title
    /// ├── (if subtitle) Label.card-subtitle
    /// ├── (if child)   child widget
    /// └── (if footer)  Box.card-footer [Button, Button, …]
    /// ```
    pub fn build(self) -> gtk4::Box {
        let card = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
        card.add_css_class("relm4-card");

        // Apply style variant CSS class.
        match self.style {
            CardStyle::Flat => card.add_css_class("card-flat"),
            CardStyle::Elevated => card.add_css_class("card-elevated"),
            CardStyle::Outlined => card.add_css_class("card-outlined"),
        }

        // Title.
        if let Some(title) = self.title {
            let title_label = gtk4::Label::new(Some(&title));
            title_label.add_css_class("card-title");
            title_label.set_halign(gtk4::Align::Start);
            title_label.set_xalign(0.0);
            card.append(&title_label);
        }

        // Subtitle.
        if let Some(subtitle) = self.subtitle {
            let subtitle_label = gtk4::Label::new(Some(&subtitle));
            subtitle_label.add_css_class("card-subtitle");
            subtitle_label.set_halign(gtk4::Align::Start);
            subtitle_label.set_xalign(0.0);
            card.append(&subtitle_label);
        }

        // Child widget (content area).
        if let Some(child) = self.child {
            card.append(&child);
        }

        // Footer action buttons.
        if !self.footer.is_empty() {
            let footer = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
            footer.add_css_class("card-footer");
            footer.set_halign(gtk4::Align::End);

            for action in self.footer {
                let button = gtk4::Button::with_label(&action.label);
                button.add_css_class("relm4-btn");
                let kind_class = match action.kind {
                    ActionKind::Primary => "relm4-btn-primary",
                    ActionKind::Secondary => "relm4-btn-secondary",
                    ActionKind::Danger => "relm4-btn-danger",
                };
                button.add_css_class(kind_class);
                footer.append(&button);
            }
            card.append(&footer);
        }

        card
    }
}
