pub mod impls;

use arraygen::Arraygen;
use mongodb::bson::{ Bson, Document };
use serde::{ Serialize, Deserialize };

use crate::Cipher;
use crate::traits::Decrypt;
use crate::traits::Encrypt;
use crate::traits::IsEmpty;
use crate::traits::ToBson;
use crate::traits::ToJson;

/// S3 struct
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, Arraygen)]
#[gen_array(fn get_array_ciphers: &mut Option<Cipher>)]
pub struct S3 {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[in_array(get_array_ciphers)]
    pub access_key_id: Option<Cipher>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[in_array(get_array_ciphers)]
    pub secret_access_key: Option<Cipher>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[in_array(get_array_ciphers)]
    pub bucket: Option<Cipher>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[in_array(get_array_ciphers)]
    pub path: Option<Cipher>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[in_array(get_array_ciphers)]
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

        for cipher in data.get_array_ciphers() {
            *cipher = cipher.clone().and_then(|d| match d.is_empty() {
                true => None,
                false => match d.decrypt_master() {
                    Ok(d) => Some(d),
                    Err(_) => Some(d)
                }
            });
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

        for cipher in data.get_array_ciphers() {
            *cipher = cipher.clone().and_then(|d| match d.is_empty() {
                true => None,
                false => match d.encrypt_master() {
                    Ok(d) => Some(d),
                    Err(_) => Some(d)
                }
            });
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

impl ToBson for S3 {
    fn to_bson(&self) -> Option<Self> {
        match self.is_empty() {
            true => None,
            false => self.encrypt()
        }
    }
}

impl ToJson for S3 {
    fn to_json(&self) -> Option<Self> {
        match self.is_empty() {
            true => None,
            false => self.decrypt()
        }
    }
}