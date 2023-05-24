use mongodb::bson::Bson;
use serde::{Serialize, Serializer, Deserialize, Deserializer};


use crate::traits::{IsEmpty, ToOption};

#[derive(Debug, Clone, PartialEq)]
pub enum Account {
    Manual,
    Facebook,
    Google,
    Twitter,
    Linkedin,
    Apple,
    Instagram,
    String(String)
}

impl Serialize for Account {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        match self.clone() {
            Self::Manual => serializer.serialize_str("MANUAL"),
            Self::Facebook => serializer.serialize_str("FACEBOOK"),
            Self::Google => serializer.serialize_str("GOOGLE"),
            Self::Twitter => serializer.serialize_str("TWITTER"),
            Self::Linkedin => serializer.serialize_str("LINKEDIN"),
            Self::Apple => serializer.serialize_str("APPLE"),
            Self::Instagram => serializer.serialize_str("INSTAGRAM"),
            Self::String(value) => serializer.serialize_str(&value)
        }
    }
}

impl<'de> Deserialize<'de> for Account {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        match value.clone().to_lowercase().as_str() {
            "manual" => Ok(Self::Manual),
            "facebook" => Ok(Self::Facebook),
            "google" => Ok(Self::Google),
            "twitter" => Ok(Self::Twitter),
            "linkedin" => Ok(Self::Linkedin),
            "apple" => Ok(Self::Apple),
            "instagram" => Ok(Self::Instagram),
            _ => Ok(Self::String(value)),
        }
    }
}

impl IsEmpty for Account {
    fn is_empty(&self) -> bool {
        match self {
            Self::String(value) => value.is_empty(),
            _ => false
        }
    }
}

impl ToOption for Account {
    fn to_option(&self) -> Option<Self> {
        match self.is_empty() {
            true => None,
            false => Some(self.clone())
        }
    }
}

impl ToString for Account {
    fn to_string(&self) -> String {
        match self.clone() {
            Self::Manual => String::from("MANUAL"),
            Self::Facebook => String::from("FACEBOOK"),
            Self::Google => String::from("GOOGLE"),
            Self::Twitter => String::from("TWITTER"),
            Self::Linkedin => String::from("LINKEDIN"),
            Self::Apple => String::from("APPLE"),
            Self::Instagram => String::from("INSTAGRAM"),
            Self::String(value) => value
        }
    }
}

impl From<Account> for Bson {
    fn from(value: Account) -> Self {
        Bson::String(value.to_string())
    }
}

impl Account {
    pub fn new<T>(value: T) -> Self
        where T: ToString
    {
        let value = value.to_string();
        match value.to_lowercase().as_str() {
            "manual" => Self::Manual,
            "facebook" => Self::Facebook,
            "google" => Self::Google,
            "twitter" => Self::Twitter,
            "linkedin" => Self::Linkedin,
            "apple" => Self::Apple,
            "instagram" => Self::Instagram,
            _ => Self::String(value),
        }
    }
}