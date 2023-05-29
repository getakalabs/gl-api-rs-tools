use actix_web::Result;
use chrono::{DateTime, Duration, Utc};
use paseto_lib::tokens::{validate_local_token, PasetoBuilder, TimeBackend};
use serde::Serialize;

use crate::{Cipher, Paseto};
use crate::Payload;
use crate::Token;

impl Paseto {
    /// Retrieve duration based on configuration
    pub fn get_duration(unit: &Option<Cipher>, time: &Option<Cipher>, min: i64) -> Duration {
        // Set unit
        let unit = unit
            .clone()
            .unwrap_or_default()
            .decrypt_master()
            .unwrap_or_default()
            .to_string()
            .parse::<i64>()
            .unwrap_or_else(|_| 0.max(min));

        // Set time
        let time = time
            .clone()
            .unwrap_or_default()
            .decrypt_master()
            .unwrap_or_default()
            .to_string();

        match time.to_lowercase().as_str() {
            "minutes" => Duration::minutes(unit),
            "hours" => Duration::hours(unit),
            "days" => Duration::days(unit),
            "weeks" => Duration::weeks(unit),
            "months" => Duration::days(unit * 30),
            _ => Duration::minutes(unit)
        }
    }

    /// Retrieve expiration date based on duration
    pub fn get_expiration_date(duration: &Duration) -> DateTime<Utc> {
        Utc::now().checked_add_signed(*duration).unwrap_or_default()
    }

    // Generate tokens
    pub fn generate_tokens<I, C>(&self, aid:I, claims: &C) -> Result<Token>
        where I: ToString,
              C: Serialize + Clone
    {
        // Convert claims to value
        let claims = serde_json::to_value(&(*claims).clone()).unwrap();

        // Retrieve aid (actor id)
        let aid = aid.to_string();

        // Retrieve access token values
        let access_token_duration = Self::get_duration(&self.access_token_key_unit, &self.access_token_key_time, 5);
        let access_token_expiry = Self::get_expiration_date(&access_token_duration);
        let access_token_signing = match base64_url::decode(&self.access_token_key_signing.clone().map_or(String::default(), |d| d.to_string())) {
            Ok(d) => d,
            Err(_) => return Err(Payload::error("Unable to generate access token"))
        };

        // Set access token
        let access_token = match PasetoBuilder::new()
            .set_encryption_key(&access_token_signing[..])
            .set_expiration(&access_token_expiry)
            .set_subject(&aid)
            .set_footer(format!("key-id:{}", &self.app_name.clone().map_or(String::default(), |d| d.to_string())).as_str())
            .set_claim("data", claims.clone())
            .build() {
            Ok(d) => d,
            Err(_) => return Err(Payload::error("Unable to generate access token"))
        };

        // Retrieve refresh token values
        let refresh_token_duration = Self::get_duration(&self.refresh_token_key_unit, &self.refresh_token_key_time, 30);
        let refresh_token_expiry = Self::get_expiration_date(&refresh_token_duration);
        let refresh_token_signing = match base64_url::decode(&self.refresh_token_key_signing.clone().map_or(String::default(), |d| d.to_string())) {
            Ok(d) => d,
            Err(_) => return Err(Payload::error("Unable to generate refresh token"))
        };

        // Set refresh token
        let refresh_token = match PasetoBuilder::new()
            .set_encryption_key(&refresh_token_signing[..])
            .set_expiration(&refresh_token_expiry)
            .set_subject(&aid)
            .set_footer(format!("key-id:{}", &self.app_name.clone().map_or(String::default(), |d| d.to_string())).as_str())
            .set_claim("data", claims.clone())
            .build() {
            Ok(d) => d,
            Err(_) => return Err(Payload::error("Unable to generate refresh token"))
        };

        // Create encrypted web token
        let encrypted = match crate::ciphers::encrypt(claims.to_string().trim(), "WEB_KEY") {
            Ok(d) => d,
            Err(_) => return Err(Payload::error("Unable to generate web token"))
        };

        // Create mutable token
        let tokens = Token {
            access: access_token,
            refresh: refresh_token,
            web: encrypted
        };

        // Return tokens
        Ok(tokens)
    }

    /// Validate access token
    pub fn validate_access_token<T, C>(&self, token: T, _: C) -> Result<C>
        where T: ToString,
              C: serde::de::DeserializeOwned + Default
    {

        // Retrieve access token key signing
        let access_token_key_signing = match self.access_token_key_signing {
            Some(ref value) => value.to_string(),
            None => return Err(Payload::error("Invalid authentication token"))
        };

        // Decrypt access token signing
        let access_token_signing = match base64_url::decode(&access_token_key_signing) {
            Ok(value) => value,
            Err(error) => return Err(Payload::error(error))
        };

        // Retrieve app name
        let app_name = match self.app_name {
            Some(ref value) => value.to_string(),
            None => return Err(Payload::error("Invalid authentication token"))
        };

        // Verify token
        let result = match validate_local_token(
            &token.to_string(),
            Some(&format!("key-id:{app_name}")),
            &access_token_signing[..],
            &TimeBackend::Chrono
        ) {
            Ok(value) => value,
            Err(error) => {
                let is_expired = error
                    .to_string()
                    .to_lowercase()
                    .as_str() == "this token is expired (exp claim).";

                return match is_expired {
                    true => Err(Payload::error("Your authentication token has expired")),
                    false => Err(Payload::error("Invalid authentication token"))
                }
            }
        };

        // Retrieve values from paseto
        let result = match result.get("data") {
            Some(value) => value.to_owned(),
            None => return Err(Payload::error("Invalid authentication token"))
        };

        // Return value to custom struct
        let claims = serde_json::from_value::<C>(result)?;

        // Return claims
        Ok(claims)
    }

    /// Validate refresh token
    pub fn validate_refresh_token<T, C>(&self, token: T, _: C) -> Result<C>
        where T: ToString,
              C: serde::de::DeserializeOwned + Default
    {
        // Decrypt refresh token signing
        let refresh_token_signing = base64_url::decode(&self.refresh_token_key_signing.clone().map_or(String::default(), |d| d.to_string()));
        if refresh_token_signing.is_err() {
            return Err(Payload::error("Invalid refresh token"));
        }

        // Set access token signing
        let refresh_token_signing = refresh_token_signing.unwrap();

        // Verify token
        let result = match validate_local_token(
            &token.to_string(),
            Some(format!("key-id:{}", &self.app_name.clone().map_or(String::default(), |d| d.to_string())).as_str()),
            &refresh_token_signing[..],
            &TimeBackend::Chrono
        ) {
            Ok(result) => match result.get("data") {
                Some(_) => result,
                None => return Err(Payload::error("Invalid refresh token"))
            },
            Err(error) => {
                let is_expired = error
                    .to_string()
                    .to_lowercase()
                    .as_str() == "this token is expired (exp claim).";

                return match is_expired {
                    true => Err(Payload::error("Your refresh token has expired")),
                    false => Err(Payload::error("Invalid refresh token"))
                }
            }
        };

        // Retrieve values from paseto
        let result = result.get("data");
        if result.is_none() {
            return Err(Payload::error("Invalid refresh token"));
        }

        // Return value to custom struct
        let result:Result<C, _> = serde_json::from_value(result.unwrap().clone());
        if result.is_err() {
            return Err(Payload::error("Invalid refresh token"));
        }

        // Return claims
        Ok(result.unwrap())
    }

    /// Validate web token
    pub fn validate_web_token<T, C>(&self, token: T, _: C) -> Result<C>
        where T: ToString,
              C: serde::de::DeserializeOwned + Default
    {
        // Create decrypt web token
        let result = crate::ciphers::encrypt(token.to_string(), "WEB_KEY");
        if result.is_err() {
            return Err(Payload::error("Decryption failed"));
        }

        // Return value to custom struct
        let result:Result<C, _> = serde_json::from_str(&result.unwrap());
        if result.is_err() {
            return Err(Payload::error("Invalid authentication token"));
        }

        // Return claims
        Ok(result.unwrap())
    }

    /// Get access token expiry
    pub fn get_access_token_expiry(&self) -> DateTime<Utc> {
        let duration = Self::get_duration(&self.access_token_key_unit, &self.access_token_key_time, 5);
        Self::get_expiration_date(&duration)
    }

    /// Get refresh token expiry
    pub fn get_refresh_token_expiry(&self) -> DateTime<Utc> {
        let duration = Self::get_duration(&self.refresh_token_key_unit, &self.refresh_token_key_time, 30);
        Self::get_expiration_date(&duration)
    }
}