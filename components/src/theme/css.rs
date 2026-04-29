//! Embedded CSS stylesheet for the relm4-kit theme.

pub const THEME_CSS: &str = r#"
:root {
    --color-primary: #3584e4;
    --color-primary-hover: #2a6fc7;
    --color-surface: #ffffff;
    --color-surface-secondary: #f6f5f4;
    --color-background: #f0f0f0;
    --color-text: #1a1a1a;
    --color-text-secondary: #5e5c64;
    --color-accent: #33d17a;
    --color-danger: #e66156;
    --color-warning: #f6d32d;

    --spacing-xs: 4px;
    --spacing-sm: 8px;
    --spacing-md: 16px;
    --spacing-lg: 24px;
    --spacing-xl: 32px;

    --radius-sm: 4px;
    --radius-md: 8px;
    --radius-lg: 12px;
    --radius-xl: 16px;

    --shadow-sm: 0 1px 3px rgba(0,0,0,0.12);
    --shadow-md: 0 4px 12px rgba(0,0,0,0.1);
    --shadow-lg: 0 8px 24px rgba(0,0,0,0.12);

    --font-sm: 12px;
    --font-md: 14px;
    --font-lg: 16px;
    --font-xl: 20px;
    --font-xxl: 24px;
}

:root.dark {
    --color-primary: #4a90d9;
    --color-primary-hover: #5a9fe6;
    --color-surface: #1a1a1a;
    --color-surface-secondary: #2a2a2a;
    --color-background: #0d0d0d;
    --color-text: #ffffff;
    --color-text-secondary: #b0b0b0;
    --shadow-sm: 0 1px 3px rgba(0,0,0,0.3);
    --shadow-md: 0 4px 12px rgba(0,0,0,0.3);
    --shadow-lg: 0 8px 24px rgba(0,0,0,0.3);
}
"#;
