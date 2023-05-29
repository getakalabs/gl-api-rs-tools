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

/// Mailer container for basic info of the API
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, Arraygen)]
#[gen_array(fn get_ciphers: &mut Option<Cipher>)]
pub struct Mailer {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[in_array(get_ciphers)]
    pub username: Option<Cipher>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[in_array(get_ciphers)]
    pub password: Option<Cipher>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[in_array(get_ciphers)]
    pub smtp_host: Option<Cipher>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[in_array(get_ciphers)]
    pub service: Option<Cipher>,
}

/// Mailer attachment container
pub struct MailerAttachment {
    pub filename: String,
    pub name: String
}

/// Decrypt Mailer information
impl Decrypt for Mailer {
    fn decrypt(&self) -> Option<Self> {
        let mut data = self.clone();

        for cipher in data.get_ciphers() {
            *cipher = cipher.clone()
                .unwrap_or_default()
                .decrypt_master().map(Some)
                .unwrap_or(cipher.clone());
        }

        match data.is_empty() {
            true => None,
            false => Some(data)
        }
    }
}

/// Encrypt Mailer information
impl Encrypt for Mailer {
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

/// Convert Mailer to Bson
impl From<Mailer> for Bson {
    fn from(value: Mailer) -> Self {
        Bson::Document(value.into())
    }
}

/// Convert Mailer to Document
impl From<Mailer> for Document {
    fn from(value: Mailer) -> Document {
        match value.is_empty() {
            true => Document::new(),
            false => {
                let mut doc = Document::new();
                doc.insert("username", Bson::from(value.username));
                doc.insert("password", Bson::from(value.password));
                doc.insert("smtp_host", Bson::from(value.smtp_host));
                doc.insert("service", Bson::from(value.service));
                doc
            }
        }
    }
}

/// Check if Mailer is empty
impl IsEmpty for Mailer {
    fn is_empty(&self) -> bool {
        Self::default() == *self
    }
}