use diesel;
use diesel::deserialize::{ FromSql, FromSqlRow };
use diesel::expression::AsExpression;
use diesel::pg::{ Pg, PgValue };
use diesel::serialize::{ Output, ToSql };
use diesel::sql_types::Text;
use serde::{ Serialize, Serializer, Deserialize, Deserializer };

/// Modules enum for settings
#[derive(Debug, Clone, PartialEq, AsExpression, FromSqlRow)]
#[diesel(sql_type = Text)]
pub enum Module {
    Base,
    Mailer,
    Paseto,
    S3
}

/// Serialize module
impl Serialize for Module {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        match self.clone() {
            Self::Base => serializer.serialize_str("BASE"),
            Self::Mailer => serializer.serialize_str("MAILER"),
            Self::Paseto => serializer.serialize_str("PASETO"),
            Self::S3 => serializer.serialize_str("S3")
        }
    }
}

/// Deserialize module
impl<'de> Deserialize<'de> for Module {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        match value.to_lowercase().as_str() {
            "base" => Ok(Self::Base),
            "mailer" => Ok(Self::Mailer),
            "paseto" => Ok(Self::Paseto),
            "s3" => Ok(Self::S3),
            _ => Err(serde::de::Error::custom(format!("Invalid module value: {}", value)))
        }
    }
}

/// Convert module to string
impl ToString for Module {
    fn to_string(&self) -> String {
        match self {
            Module::Base => String::from("BASE"),
            Module::Mailer => String::from("MAILER"),
            Module::Paseto => String::from("PASETO"),
            Module::S3 => String::from("S3")
        }
    }
}

/// Convert string to module
impl FromSql<Text, Pg> for Module {
    fn from_sql(bytes: PgValue) -> diesel::deserialize::Result<Self> {
        match <String as FromSql<Text, Pg>>::from_sql(bytes)?.to_lowercase().as_str() {
            "base" => Ok(Self::Base),
            "mailer" => Ok(Self::Mailer),
            "paseto" => Ok(Self::Paseto),
            "s3" => Ok(Self::S3),
            _ => Err("Invalid module value".into())
        }
    }
}

impl ToSql<Text, Pg> for Module {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> diesel::serialize::Result {
        let text = match self {
            Module::Base => "BASE",
            Module::Mailer => "MAILER",
            Module::Paseto => "PASETO",
            Module::S3 => "S3"
        };

        ToSql::<Text, Pg>::to_sql(text, &mut out.reborrow())
    }
}