pub mod mutations;
pub mod stages;

use arraygen::Arraygen;
use mongodb::bson::{ Bson, Document };
use serde::{ Serialize, Deserialize };
use std::default::Default;

use crate::Cipher;
use crate::traits::Decrypt;
use crate::traits::Encrypt;
use crate::traits::IsEmpty;

/// Base container for basic info of the API
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, Arraygen)]
#[gen_array(fn get_ciphers: &mut Option<Cipher>)]
pub struct Base {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[in_array(get_ciphers)]
    pub api_url: Option<Cipher>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[in_array(get_ciphers)]
    pub web_url: Option<Cipher>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[in_array(get_ciphers)]
    pub admin_url: Option<Cipher>,
}

/// Decrypt Base information
impl Decrypt for Base {
    fn decrypt(&self) -> Option<Self> {
        let mut data = self.clone();

        for cipher in data.get_ciphers() {
            *cipher = cipher.clone()
                .unwrap_or_default()
                .decrypt_master()
                .map(Some)
                .unwrap_or(cipher.clone());
        }

        match data.is_empty() {
            true => None,
            false => Some(data)
        }
    }
}

/// Encrypt Base information
impl Encrypt for Base {
    fn encrypt(&self) -> Option<Self> {
        let mut data = self.clone();

        for cipher in data.get_ciphers() {
            *cipher = cipher.clone()
                .unwrap_or_default()
                .encrypt_master()
                .map(Some)
                .unwrap_or(cipher.clone());
        }

        match data.is_empty() {
            true => None,
            false => Some(data)
        }
    }
}

/// Convert Base to Bson
impl From<Base> for Bson {
    fn from(value: Base) -> Self {
        Bson::Document(value.into())
    }
}

/// Convert Base to Document
impl From<Base> for Document {
    fn from(value: Base) -> Document {
        match value.is_empty() {
            true => Document::new(),
            false => {
                let mut doc = Document::new();
                doc.insert("api_url", Bson::from(value.api_url));
                doc.insert("web_url", Bson::from(value.web_url));
                doc.insert("admin_url", Bson::from(value.admin_url));
                doc
            }
        }
    }
}

/// Check if Base is empty
impl IsEmpty for Base {
    fn is_empty(&self) -> bool {
        Self::default() == *self
    }
}