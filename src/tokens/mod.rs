use serde::{Serialize, Deserialize};

/// Token container struct
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Token {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub access: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub refresh: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub web: String
}

/// Token implementations
impl Token {
    pub fn new() -> Self {
        Self::default()
    }
}
