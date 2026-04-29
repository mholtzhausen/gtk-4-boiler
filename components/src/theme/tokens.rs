//! Color, spacing, radii, shadow, and typography tokens for theming.

/// Color tokens for the theme.
pub struct ColorTokens {
    pub primary: &'static str,
    pub primary_hover: &'static str,
    pub surface: &'static str,
    pub surface_secondary: &'static str,
    pub background: &'static str,
    pub text: &'static str,
    pub text_secondary: &'static str,
    pub accent: &'static str,
    pub danger: &'static str,
    pub warning: &'static str,
}

impl Default for ColorTokens {
    fn default() -> Self {
        Self {
            primary: "#3584e4",
            primary_hover: "#2a6fc7",
            surface: "#ffffff",
            surface_secondary: "#f6f5f4",
            background: "#f0f0f0",
            text: "#1a1a1a",
            text_secondary: "#5e5c64",
            accent: "#33d17a",
            danger: "#e66156",
            warning: "#f6d32d",
        }
    }
}

/// Spacing tokens for the theme.
pub struct SpacingTokens {
    pub xs: &'static str,
    pub sm: &'static str,
    pub md: &'static str,
    pub lg: &'static str,
    pub xl: &'static str,
}

impl Default for SpacingTokens {
    fn default() -> Self {
        Self {
            xs: "4px",
            sm: "8px",
            md: "16px",
            lg: "24px",
            xl: "32px",
        }
    }
}

/// Border radius tokens for the theme.
pub struct RadiiTokens {
    pub sm: &'static str,
    pub md: &'static str,
    pub lg: &'static str,
    pub xl: &'static str,
}

impl Default for RadiiTokens {
    fn default() -> Self {
        Self {
            sm: "4px",
            md: "8px",
            lg: "12px",
            xl: "16px",
        }
    }
}

/// Shadow tokens for the theme.
pub struct ShadowTokens {
    pub sm: &'static str,
    pub md: &'static str,
    pub lg: &'static str,
}

impl Default for ShadowTokens {
    fn default() -> Self {
        Self {
            sm: "0 1px 3px rgba(0,0,0,0.12)",
            md: "0 4px 12px rgba(0,0,0,0.1)",
            lg: "0 8px 24px rgba(0,0,0,0.12)",
        }
    }
}

/// Typography tokens for the theme.
pub struct TypographyTokens {
    pub sm: &'static str,
    pub md: &'static str,
    pub lg: &'static str,
    pub xl: &'static str,
    pub xxl: &'static str,
}

impl Default for TypographyTokens {
    fn default() -> Self {
        Self {
            sm: "12px",
            md: "14px",
            lg: "16px",
            xl: "20px",
            xxl: "24px",
        }
    }
}

/// The complete theme definition containing all token categories.
#[derive(Default)]
pub struct Theme {
    pub colors: ColorTokens,
    pub spacing: SpacingTokens,
    pub radii: RadiiTokens,
    pub typography: TypographyTokens,
    pub shadows: ShadowTokens,
}

impl Theme {
    /// Returns a dark-mode theme with overridden values.
    pub fn dark() -> Self {
        Self {
            colors: ColorTokens {
                primary: "#4a90d9",
                primary_hover: "#5a9fe6",
                surface: "#1a1a1a",
                surface_secondary: "#2a2a2a",
                background: "#0d0d0d",
                text: "#ffffff",
                text_secondary: "#b0b0b0",
                ..ColorTokens::default()
            },
            ..Theme::default()
        }
    }
}
