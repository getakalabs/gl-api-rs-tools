pub mod impls;
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

/// Paseto container for basic info of the API
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, Arraygen)]
#[gen_array(fn get_ciphers: &mut Option<Cipher>)]
pub struct Paseto {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[in_array(get_ciphers)]
    pub app_name: Option<Cipher>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[in_array(get_ciphers)]
    pub access_token_key_unit: Option<Cipher>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[in_array(get_ciphers)]
    pub access_token_key_time: Option<Cipher>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[in_array(get_ciphers)]
    pub access_token_key_signing: Option<Cipher>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[in_array(get_ciphers)]
    pub refresh_token_key_unit: Option<Cipher>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[in_array(get_ciphers)]
    pub refresh_token_key_time: Option<Cipher>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[in_array(get_ciphers)]
    pub refresh_token_key_signing: Option<Cipher>,
}

/// Decrypt Paseto information
impl Decrypt for Paseto {
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

/// Encrypt Paseto information
impl Encrypt for Paseto {
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

/// Convert Paseto to Bson
impl From<Paseto> for Bson {
    fn from(value: Paseto) -> Self {
        Bson::Document(value.into())
    }
}

/// Convert Paseto to Document
impl From<Paseto> for Document {
    fn from(value: Paseto) -> Document {
        match value.is_empty() {
            true => Document::new(),
            false => {
                let mut doc = Document::new();
                doc.insert("app_name", Bson::from(value.app_name));
                doc.insert("access_token_key_unit", Bson::from(value.access_token_key_unit));
                doc.insert("access_token_key_time", Bson::from(value.access_token_key_time));
                doc.insert("access_token_key_signing", Bson::from(value.access_token_key_signing));
                doc.insert("refresh_token_key_unit", Bson::from(value.refresh_token_key_unit));
                doc.insert("refresh_token_key_time", Bson::from(value.refresh_token_key_time));
                doc.insert("refresh_token_key_signing", Bson::from(value.refresh_token_key_signing));
                doc
            }
        }
    }
}

/// Convert String to Paseto instance
impl From<String> for Paseto {
    fn from(value: String) -> Self {
        Self {
            app_name: Some(Cipher::new(value)),
            access_token_key_unit: Some(Cipher::new("5")),
            access_token_key_time: Some(Cipher::new("Minutes")),
            access_token_key_signing: Some(Cipher::new(crate::ciphers::generate())),
            refresh_token_key_unit: Some(Cipher::new("30")),
            refresh_token_key_time: Some(Cipher::new("Minutes")),
            refresh_token_key_signing: Some(Cipher::new(crate::ciphers::generate())),
        }
    }
}

/// Check if Paseto is empty
impl IsEmpty for Paseto {
    fn is_empty(&self) -> bool {
        Self::default() == *self
    }
}