use actix_web::Result;
use mongodb::{ bson::doc, options::FindOneOptions, Database };
use std::sync::{ Arc, RwLock };

use crate::configs::SETTINGS;
use crate::configs::Module;
use crate::configs::Settings;

use crate::Base;
use crate::traits::Decrypt;

/// Base stage implementation
impl Base {
    /// Retrieve initial base configuration
    pub async fn stage(database: &Database) -> Result<Arc<RwLock<Self>>> {
        // Set collection
        let collection = database.collection::<Settings>(SETTINGS);

        // Create filter & options
        let filter = doc! { "module": Module::Base };
        let options = FindOneOptions::builder()
            .sort(doc! { "created_at": -1 })
            .build();

        // Retrieve settings
        let settings = collection.find_one(filter, options)
            .await
            .unwrap_or_default()
            .unwrap_or_default();

        // Return base value if it exists
        if let Some( base ) = settings.base.unwrap_or_default().decrypt() {
            return Ok(Arc::new(RwLock::new(base)));
        }

        // Return default value
        Ok(Arc::new(RwLock::new(Self::default())))
    }
}