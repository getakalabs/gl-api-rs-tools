use actix_web::Result;
use mongodb::{ bson::doc, options::FindOneOptions, Database };
use std::sync::{ Arc, RwLock };

use crate::configs::SETTINGS;
use crate::configs::Module;
use crate::configs::Settings;

use crate::S3;
use crate::traits::Decrypt;

/// S3 stage implementation
impl S3 {
    /// Retrieve initial s3 configuration
    pub async fn stage(datas3: &Database) -> Result<Arc<RwLock<Self>>> {
        // Set collection
        let collection = datas3.collection::<Settings>(SETTINGS);

        // Create filter & options
        let filter = doc! { "module": Module::S3 };
        let options = FindOneOptions::builder()
            .sort(doc! { "created_at": -1 })
            .build();

        // Retrieve settings
        let settings = collection.find_one(filter, options)
            .await
            .unwrap_or_default()
            .unwrap_or_default();

        // Return s3 value if it exists
        if let Some( s3 ) = settings.s3.unwrap_or_default().decrypt() {
            return Ok(Arc::new(RwLock::new(s3)));
        }

        // Return default value
        Ok(Arc::new(RwLock::new(Self::default())))
    }
}