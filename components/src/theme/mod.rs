pub mod tokens;

// Re-exports
pub use tokens::*;

/// The full theme CSS stylesheet, embedded at compile time.
/// Contains all CSS variables, dark mode overrides, and pre-styled component classes.
pub const THEME_CSS: &str = include_str!("theme.css");
