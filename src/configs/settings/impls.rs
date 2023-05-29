use actix_web::Result;
use mongodb::Database;

use crate::configs::Settings;
use crate::configs::SETTINGS;

use crate::traits::Decrypt;
use crate::traits::Encrypt;

use crate::Payload;

impl Settings {
    pub async fn create(&self, database: &Database) -> Result<Self> {
        if let Some(value) = self.encrypt() {
            let collection = database.collection::<Settings>(SETTINGS);

            return match collection.insert_one(value, None).await {
                Ok(_) => Ok(self.decrypt().unwrap_or(self.clone())),
                Err(errors) => Err(Payload::error(errors))
            };
        }

        Err(Payload::error("An error occurred while trying to save a new settings entry"))
    }
}
