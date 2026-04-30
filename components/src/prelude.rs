//! Prelude: re-export all public API for convenient glob imports.
//!
//! ```rust
//! use relm4_kit::prelude::*;
//! ```

// Re-export all primitives so users can do `use relm4_kit::prelude::*;`
// and get Card, Button, Toggle, Badge, Avatar, ButtonAction, etc.
pub use crate::primitives::*;
