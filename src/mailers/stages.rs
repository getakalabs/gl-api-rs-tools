use actix_web::Result;
use mongodb::{ bson::doc, options::FindOneOptions, Database };
use std::sync::{ Arc, RwLock };

use crate::configs::SETTINGS;
use crate::configs::Module;
use crate::configs::Settings;

use crate::Mailer;
use crate::traits::Decrypt;

/// Mailer stage implementation
impl Mailer {
    /// Retrieve initial mailer configuration
    pub async fn stage(datamailer: &Database) -> Result<Arc<RwLock<Self>>> {
        // Set collection
        let collection = datamailer.collection::<Settings>(SETTINGS);

        // Create filter & options
        let filter = doc! { "module": Module::Mailer };
        let options = FindOneOptions::builder()
            .sort(doc! { "created_at": -1 })
            .build();

        // Retrieve settings
        let settings = collection.find_one(filter, options)
            .await
            .unwrap_or_default()
            .unwrap_or_default();

        // Return mailer value if it exists
        if let Some( mailer ) = settings.mailer.unwrap_or_default().decrypt() {
            return Ok(Arc::new(RwLock::new(mailer)));
        }

        // Return default value
        Ok(Arc::new(RwLock::new(Self::default())))
    }
}