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

/// Mailer container for basic info of the API
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, Arraygen, AsExpression, FromSqlRow)]
#[diesel(sql_type = Jsonb)]
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

/// Check if Mailer is empty
impl IsEmpty for Mailer {
    fn is_empty(&self) -> bool {
        Self::default() == *self
    }
}

/// FromSql implementation for Mailer
impl FromSql<Jsonb, Pg> for Mailer {
    fn from_sql(bytes: PgValue) -> diesel::deserialize::Result<Self> {
        Ok(serde_json::from_value(<serde_json::Value as FromSql<Jsonb, Pg>>::from_sql(bytes)?)?)
    }
}

/// ToSql implementation for Mailer
impl ToSql<Jsonb, Pg> for Mailer {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> diesel::serialize::Result {
        ToSql::<Jsonb, Pg>::to_sql(&serde_json::to_value(self)?, &mut out.reborrow())
    }
}