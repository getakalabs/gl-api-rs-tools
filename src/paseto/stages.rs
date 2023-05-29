use actix_web::Result;
use mongodb::{ bson::doc, options::FindOneOptions, Database };
use std::sync::{ Arc, RwLock };

use crate::configs::SETTINGS;
use crate::configs::Module;
use crate::configs::Settings;

use crate::Paseto;
use crate::traits::Decrypt;

/// Paseto stage implementation
impl Paseto {
    /// Retrieve initial paseto configuration
    pub async fn stage<T>(database: &Database, name: T) -> Result<Arc<RwLock<Self>>>
        where T: ToString
    {
        // Set collection
        let collection = database.collection::<Settings>(SETTINGS);

        // Create filter & options
        let filter = doc! { "module": Module::Paseto };
        let options = FindOneOptions::builder()
            .sort(doc! { "created_at": -1 })
            .build();

        // Retrieve settings
        let settings = collection.find_one(filter, options)
            .await
            .unwrap_or_default()
            .unwrap_or_default();

        // Return paseto value if it exists
        if let Some( paseto ) = settings.paseto.unwrap_or_default().decrypt() {
            return Ok(Arc::new(RwLock::new(paseto)));
        }

        // Return value
        match Self::setup(database, name).await {
            Ok(value) => Ok(value),
            Err(_) => Ok(Arc::new(RwLock::new(Self::default())))
        }
    }

    /// Setup initial paseto configuration
    pub async fn setup<T>(database: &Database, name: T) -> Result<Arc<RwLock<Self>>>
        where T: ToString
    {
        let paseto = Paseto::from(name.to_string());
        let settings = Settings::from(paseto);

        if let Ok(value) = settings.create(database).await {
            return Ok(Arc::new(RwLock::new(value.paseto.unwrap_or_default())));
        }

        Ok(Arc::new(RwLock::new(Self::default())))
    }
}