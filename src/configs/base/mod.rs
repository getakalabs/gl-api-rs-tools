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
use std::default::Default;

use crate::Cipher;
use crate::traits::Decrypt;
use crate::traits::Encrypt;
use crate::traits::IsEmpty;

/// Base container for basic info of the API
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, Arraygen, AsExpression, FromSqlRow)]
#[diesel(sql_type = Jsonb)]
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

/// Check if Base is empty
impl IsEmpty for Base {
    fn is_empty(&self) -> bool {
        Self::default() == *self
    }
}

/// FromSql implementation for Base
impl FromSql<Jsonb, Pg> for Base {
    fn from_sql(bytes: PgValue) -> diesel::deserialize::Result<Self> {
        Ok(serde_json::from_value(<serde_json::Value as FromSql<Jsonb, Pg>>::from_sql(bytes)?)?)
    }
}

/// ToSql implementation for Base
impl ToSql<Jsonb, Pg> for Base {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> diesel::serialize::Result {
        ToSql::<Jsonb, Pg>::to_sql(&serde_json::to_value(self)?, &mut out.reborrow())
    }
}