pub mod impls;
pub mod mutations;
pub mod stages;

use arraygen::Arraygen;
use mongodb::bson::{ Bson, Document };
use serde::{ Serialize, Deserialize };

use crate::Cipher;
use crate::traits::Decrypt;
use crate::traits::Encrypt;
use crate::traits::IsEmpty;

/// S3 struct
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, Arraygen)]
#[gen_array(fn get_ciphers: &mut Option<Cipher>)]
pub struct S3 {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[in_array(get_ciphers)]
    pub access_key_id: Option<Cipher>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[in_array(get_ciphers)]
    pub secret_access_key: Option<Cipher>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[in_array(get_ciphers)]
    pub bucket: Option<Cipher>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[in_array(get_ciphers)]
    pub path: Option<Cipher>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[in_array(get_ciphers)]
    pub region: Option<Cipher>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_thumbnail_small_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_thumbnail_medium_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_thumbnail_large_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_thumbnail_xl_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_landscape_width_small_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_landscape_height_small_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_landscape_width_medium_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_landscape_height_medium_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_landscape_width_large_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_landscape_height_large_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_landscape_width_xl_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_landscape_height_xl_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_landscape_width_xxl_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_landscape_height_xxl_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_landscape_width_xxxl_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_landscape_height_xxxl_size: Option<i32>,
}

/// Decrypt implementation for S3
impl Decrypt for S3 {
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

/// Encrypt implementation for S3
impl Encrypt for S3 {
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

/// From implementation from S3 to Bson
impl From<S3> for Bson {
    fn from(value: S3) -> Self {
        Bson::Document(value.into())
    }
}

/// From implementation from S3 to Document
impl From<S3> for Document {
    fn from(value: S3) -> Document {
        let mut doc = Document::new();

        doc.insert("access_key_id", Bson::from(value.access_key_id));
        doc.insert("secret_access_key", Bson::from(value.secret_access_key));
        doc.insert("bucket", Bson::from(value.bucket));
        doc.insert("path", Bson::from(value.path));
        doc.insert("region", Bson::from(value.region));
        doc.insert("image_thumbnail_small_size", Bson::from(value.image_thumbnail_small_size));
        doc.insert("image_thumbnail_medium_size", Bson::from(value.image_thumbnail_medium_size));
        doc.insert("image_thumbnail_large_size", Bson::from(value.image_thumbnail_large_size));
        doc.insert("image_thumbnail_xl_size", Bson::from(value.image_thumbnail_xl_size));
        doc.insert("image_landscape_width_small_size", Bson::from(value.image_landscape_width_small_size));
        doc.insert("image_landscape_height_small_size", Bson::from(value.image_landscape_height_small_size));
        doc.insert("image_landscape_width_medium_size", Bson::from(value.image_landscape_width_medium_size));
        doc.insert("image_landscape_height_medium_size", Bson::from(value.image_landscape_height_medium_size));
        doc.insert("image_landscape_width_large_size", Bson::from(value.image_landscape_width_large_size));
        doc.insert("image_landscape_height_large_size", Bson::from(value.image_landscape_height_large_size));
        doc.insert("image_landscape_width_xl_size", Bson::from(value.image_landscape_width_xl_size));
        doc.insert("image_landscape_height_xl_size", Bson::from(value.image_landscape_height_xl_size));
        doc.insert("image_landscape_width_xxl_size", Bson::from(value.image_landscape_width_xxl_size));
        doc.insert("image_landscape_height_xxl_size", Bson::from(value.image_landscape_height_xxl_size));
        doc.insert("image_landscape_width_xxxl_size", Bson::from(value.image_landscape_width_xxxl_size));
        doc.insert("image_landscape_height_xxxl_size", Bson::from(value.image_landscape_height_xxxl_size));

        doc
    }
}

impl IsEmpty for S3 {
    fn is_empty(&self) -> bool {
        Self::default() == *self
    }
}
