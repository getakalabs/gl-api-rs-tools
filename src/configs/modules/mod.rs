use mongodb::bson::Bson;
use serde::{ Serialize, Serializer, Deserialize, Deserializer };

/// Modules enum for settings
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Module {
    Base,
    Mailer,
    Paseto,
    S3
}

/// Serialize module
impl Serialize for Module {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        match self.clone() {
            Self::Base => serializer.serialize_str("Base"),
            Self::Mailer => serializer.serialize_str("Mailer"),
            Self::Paseto => serializer.serialize_str("Paseto"),
            Self::S3 => serializer.serialize_str("S3")
        }
    }
}

/// Deserialize module
impl<'de> Deserialize<'de> for Module {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        match value.to_lowercase().as_str() {
            "base" => Ok(Self::Base),
            "mailer" => Ok(Self::Mailer),
            "paseto" => Ok(Self::Paseto),
            "s3" => Ok(Self::S3),
            _ => Err(serde::de::Error::custom(format!("Invalid module value: {}", value)))
        }
    }
}

/// Convert module to bson
impl From<Module> for Bson {
    fn from(value: Module) -> Self {
        Bson::String(value.to_string())
    }
}

/// Convert module to string
impl ToString for Module {
    fn to_string(&self) -> String {
        match self {
            Module::Base => String::from("Base"),
            Module::Mailer => String::from("Mailer"),
            Module::Paseto => String::from("Paseto"),
            Module::S3 => String::from("S3")
        }
    }
}

