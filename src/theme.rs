use serde::{Deserialize, Serialize};

/// Theme configuration for components.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentTheme {
    pub name: String,
    pub primary_color: String,
    pub background: String,
    pub text_color: String,
    pub spacing: usize,
}

impl ComponentTheme {
    pub fn dark() -> Self {
        Self {
            name: "dark".into(),
            primary_color: "#bb86fc".into(),
            background: "#121212".into(),
            text_color: "#e0e0e0".into(),
            spacing: 8,
        }
    }

    pub fn light() -> Self {
        Self {
            name: "light".into(),
            primary_color: "#6200ee".into(),
            background: "#ffffff".into(),
            text_color: "#333333".into(),
            spacing: 8,
        }
    }
}
