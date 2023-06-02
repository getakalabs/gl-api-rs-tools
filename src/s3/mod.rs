pub mod impls;
pub mod mutations;
pub mod stages;

use arraygen::Arraygen;
use diesel;
use diesel::deserialize::{ FromSql, FromSqlRow };
use diesel::expression::AsExpression;
use diesel::pg::{ Pg, PgValue };
use diesel::serialize::{ Output, ToSql };
use diesel::sql_types::Jsonb;
use serde::{ Serialize, Deserialize };

use crate::Cipher;
use crate::traits::Decrypt;
use crate::traits::Encrypt;
use crate::traits::IsEmpty;

/// S3 struct
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, Arraygen, AsExpression, FromSqlRow)]
#[diesel(sql_type = Jsonb)]
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

/// IsEmpty implementation for S3
impl IsEmpty for S3 {
    fn is_empty(&self) -> bool {
        Self::default() == *self
    }
}


/// FromSql implementation for S3
impl FromSql<Jsonb, Pg> for S3 {
    fn from_sql(bytes: PgValue) -> diesel::deserialize::Result<Self> {
        Ok(serde_json::from_value(<serde_json::Value as FromSql<Jsonb, Pg>>::from_sql(bytes)?)?)
    }
}

/// ToSql implementation for S3
impl ToSql<Jsonb, Pg> for S3 {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> diesel::serialize::Result {
        ToSql::<Jsonb, Pg>::to_sql(&serde_json::to_value(self)?, &mut out.reborrow())
    }
}