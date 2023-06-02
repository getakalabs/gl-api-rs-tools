use serde::{Serialize, Deserialize};

/// Create BearerToken
#[derive(Default, Debug, Clone, PartialEq)]
pub struct BearerToken(pub String);

impl ToString for BearerToken {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

/// Create bearer token implementation
impl BearerToken {
    pub fn new<T>(token: T) -> Self
        where T: ToString
    {
        Self(token.to_string())
    }
}

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
