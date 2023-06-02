use arraygen::Arraygen;
use diesel;
use diesel::prelude::*;
use chrono::{ NaiveDateTime, Utc };
use nanoid::nanoid;
use serde::{ Serialize, Deserialize };

use crate::Base;
use crate::configs::Module;
use crate::Mailer;
use crate::Paseto;
use crate::S3;
use crate::traits::Decrypt;
use crate::traits::Encrypt;
use crate::traits::IsEmpty;

use crate::schema;

/// Settings container struct
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, Arraygen, AsChangeset, Queryable, Insertable, QueryableByName)]
#[gen_array(fn get_base: &mut Option<Base>)]
#[gen_array(fn get_mailer: &mut Option<Mailer>)]
#[gen_array(fn get_paseto: &mut Option<Paseto>)]
#[gen_array(fn get_s3: &mut Option<S3>)]
#[diesel(table_name = schema::settings)]
pub(crate) struct Settings {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[in_array(get_base)]
    pub base: Option<Base>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[in_array(get_mailer)]
    pub mailer: Option<Mailer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[in_array(get_paseto)]
    pub paseto: Option<Paseto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[in_array(get_s3)]
    pub s3: Option<S3>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module: Option<Module>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<NaiveDateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<NaiveDateTime>
}

/// Decrypt Settings information
impl Decrypt for Settings {
    fn decrypt(&self) -> Option<Self> {
        let mut data = self.clone();

        for base in data.get_base() {
            *base = base.clone().unwrap_or_default().decrypt();
        }

        for mailer in data.get_mailer() {
            *mailer = mailer.clone().unwrap_or_default().decrypt();
        }

        for paseto in data.get_paseto() {
            *paseto = paseto.clone().unwrap_or_default().decrypt();
        }

        for s3 in data.get_s3() {
            *s3 = s3.clone().unwrap_or_default().decrypt();
        }

        match data.is_empty() {
            true => None,
            false => Some(data)
        }
    }
}

/// Encrypt Settings information
impl Encrypt for Settings {
    fn encrypt(&self) -> Option<Self> {
        let mut data = self.clone();

        for base in data.get_base() {
            *base = base.clone().unwrap_or_default().encrypt();
        }

        for mailer in data.get_mailer() {
            *mailer = mailer.clone().unwrap_or_default().encrypt();
        }

        for paseto in data.get_paseto() {
            *paseto = paseto.clone().unwrap_or_default().encrypt();
        }

        for s3 in data.get_s3() {
            *s3 = s3.clone().unwrap_or_default().encrypt();
        }

        match data.is_empty() {
            true => None,
            false => Some(data)
        }
    }
}

/// Create Settings from Paseto objects
impl From<Paseto> for Settings {
    fn from(paseto: Paseto) -> Self {
        let paseto = match paseto.is_empty() {
            true => None,
            false => Some(paseto)
        };

        Self {
            id: nanoid!(),
            module: Some(Module::Paseto),
            paseto,
            created_at: Some(Utc::now().naive_utc()),
            updated_at: Some(Utc::now().naive_utc()),
            ..Default::default()
        }
    }
}

/// Create Settings from Paseto objects
impl From<&Paseto> for Settings {
    fn from(paseto: &Paseto) -> Self {
        Self::from(paseto.clone())
    }
}

/// Check if Settings is empty
impl IsEmpty for Settings {
    fn is_empty(&self) -> bool {
        Self::default() == *self
    }
}