use anyhow::Result;
use mongodb::bson::{ Bson, Document };
use serde::{ Serialize, Deserialize };
use xsalsa20poly1305::aead::{ Aead, KeyInit };
use xsalsa20poly1305::aead::generic_array::{ GenericArray, typenum::{ self, Unsigned } };
use xsalsa20poly1305::XSalsa20Poly1305;


use crate::traits::IsEmpty;

use super::action::Action;

/// Create const for master key and web key
const MASTER_KEY: &str = "MASTER_KEY";
const WEB_KEY: &str = "WEB_KEY";

/// Cipher struct
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Cipher {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) is_encrypted: Option<bool>,
}

impl IsEmpty for Cipher {
    fn is_empty(&self) -> bool {
        Self::default() == *self
    }
}

/// From implementation for Cipher and convert it to Bson
impl From<Cipher> for Bson {
    fn from(value: Cipher) -> Self {
        Bson::Document(value.into())
    }
}

/// From implementation for Cipher and convert it to Mongo Document
impl From<Cipher> for Document {
    fn from(value: Cipher) -> Document {
        let mut doc = Document::new();

        doc.insert("content", Bson::from(value.content));
        doc.insert("hash", Bson::from(value.hash));
        doc.insert("is_encrypted", Bson::from(value.is_encrypted));

        doc
    }
}

/// ToString implementation of cipher
impl ToString for Cipher {
    fn to_string(&self) -> String {
        self.content.clone().unwrap_or(String::default())
    }
}

/// Cipher implementation of internal methods
impl Cipher {
    /// Create new cipher object from string value
    pub fn new<C>(content: C) -> Self
        where C: ToString
    {
        Self {
            content: Some(content.to_string()),
            hash: None,
            is_encrypted: None,
        }
    }

    /// Checks if cipher is ready for encryption/decryption
    pub(super) fn is_ready(&self, action: Action) -> bool {
        let is_encrypted = self.is_encrypted.unwrap_or(false);
        let is_empty_content = self.content.clone().unwrap_or(String::default()).is_empty();
        let is_empty_hash = self.hash.clone().unwrap_or(String::default()).is_empty();

        match action {
            Action::Encrypt => !is_encrypted && !is_empty_content,
            Action::Decrypt => is_encrypted && !is_empty_content && !is_empty_hash,
        }
    }

    /// Encrypt providing env key
    pub(super) fn encrypt<K>(&self, key: K) -> Result<Self>
        where K: ToString
    {
        // Check if encryption is ready
        if !self.is_ready(Action::Encrypt) {
            return Err(anyhow::anyhow!("Unable to encrypt content"));
        }

        // Get manager
        let mut manager = self.clone();

        // Encrypt content
        let hash = base64_url::decode(&super::generate())?;
        let nonce = XSalsa20Poly1305::generate_nonce(&mut rand::rngs::OsRng);
        let cipher = XSalsa20Poly1305::new(GenericArray::from_slice(&hash));
        let content = match cipher.encrypt(&nonce, manager.content.clone().unwrap_or(String::default()).as_bytes()) {
            Ok(value) => value,
            Err(_) => return Err(anyhow::anyhow!("Unable to encrypt content"))
        };

        // Populate manager content
        manager.content = Some(base64_url::encode(&[&nonce[..], &content[..]].concat()));

        // Encrypt hash
        let binding = base64_url::decode(&std::env::var(key.to_string())?)?;
        let key =  GenericArray::from_slice(&binding);
        let cipher = XSalsa20Poly1305::new(key);

        // Encrypt hash
        let nonce = XSalsa20Poly1305::generate_nonce(&mut rand::rngs::OsRng);
        let hash = match cipher.encrypt(&nonce, hash.as_slice()) {
            Ok(value) => value,
            Err(_) => return Err(anyhow::anyhow!("Unable to encrypt hash"))
        };

        // Populate manager
        manager.hash = Some(base64_url::encode( &[&nonce[..], &hash[..]].concat()));
        manager.is_encrypted = Some(true);

        // Return manager
        Ok(manager)
    }

    /// Decrypt providing env key
    pub(super) fn decrypt<K>(&self, key: K) -> Result<Self>
        where K: ToString
    {
        // Check if decryption is ready
        if !self.is_ready(Action::Decrypt) {
            return Err(anyhow::anyhow!("Unable to decrypt content"));
        }

        // Get manager
        let mut manager = self.clone();

        // Decrypt hash
        let binding = base64_url::decode(&std::env::var(key.to_string())?)?;
        let key =  GenericArray::from_slice(&binding);
        let cipher = XSalsa20Poly1305::new(key);
        let hash = base64_url::decode(&manager.hash.clone().unwrap_or(String::default()))?;
        let nonce = GenericArray::from_slice(&hash[..typenum::U24::to_usize()]);
        let hash = match cipher.decrypt(nonce, &hash[typenum::U24::to_usize()..]) {
            Ok(value) => value,
            Err(_) => return Err(anyhow::anyhow!("Unable to decrypt hash"))
        };

        // Decrypt content
        let cipher = XSalsa20Poly1305::new(GenericArray::from_slice(&hash));
        let content = base64_url::decode(&manager.content.clone().unwrap_or(String::default()))?;
        let nonce = GenericArray::from_slice(&content[..typenum::U24::to_usize()]);
        let content = match cipher.decrypt(nonce, &content[typenum::U24::to_usize()..]) {
            Ok(value) => value,
            Err(_) => return Err(anyhow::anyhow!("Unable to decrypt content"))
        };

        // Populate manager
        manager.content = Some(String::from_utf8_lossy(content.as_slice()).to_string());
        manager.is_encrypted = Some(false);

        // Return manager
        Ok(manager)
    }

    /// Encrypt using master key
    pub fn encrypt_master(&self) -> Result<Self> {
        self.encrypt(MASTER_KEY)
    }

    /// Encrypt using web key
    pub fn encrypt_web(&self) -> Result<Self> {
        self.encrypt(WEB_KEY)
    }

    /// Decrypt using master key
    pub fn decrypt_master(&self) -> Result<Self> {
        self.decrypt(MASTER_KEY)
    }

    /// Decrypt using web key
    pub fn decrypt_web(&self) -> Result<Self> {
        self.decrypt(WEB_KEY)
    }
}