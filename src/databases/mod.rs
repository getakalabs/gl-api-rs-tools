pub mod mongo;

use actix_web::Result;
use mongodb::{ Client, options::ClientOptions, Database };
use std::env;

use crate::traits::IsEmpty;
use crate::Payload;

/// Set database manager
#[derive(Debug, Clone)]
pub enum DatabaseManager {
    MongoDB(mongo::MongoDB),
    None
}

/// Implement IsEmpty trait for DatabaseManager
impl IsEmpty for DatabaseManager {
    fn is_empty(&self) -> bool {
        matches!(self, Self::None)
    }
}

/// DatabaseManager implementations
impl DatabaseManager {
    /// Create new database manager
    pub async fn new<T, U>(url: T, name: U) -> Result<Self>
        where T: ToString,
              U: ToString
    {
        // Retrieve database url and name from environment variables
        let url = match env::var(url.to_string()) {
            Ok(value) => value,
            Err(_) => return Err(Payload::error("Unable to retrieve database url".to_string()))
        };

        // Retrieve database name from environment variables
        let name = match env::var(name.to_string()) {
            Ok(value) => value,
            Err(_) => return Err(Payload::error("Unable to retrieve database name"))
        };

        // Parse database url
        let options = match ClientOptions::parse(&url).await {
            Ok(value) => value,
            Err(error) => return Err(Payload::error(error))
        };

        // Create database client
        let client = match Client::with_options(options) {
            Ok(value) => value,
            Err(error) => return Err(Payload::error(error))
        };

        // Return database manager
        Ok(Self::MongoDB(mongo::MongoDB::from((client, name))))
    }

    /// Retrieve database instance
    pub fn get(&self) -> Result<Database> {
        match self {
            Self::MongoDB(value) => Ok(value.database.clone()),
            Self::None => Err(Payload::error("Unable to retrieve database"))
        }
    }

    /// Retrieve database client
    pub fn get_client(&self) -> Result<Client> {
        match self {
            Self::MongoDB(value) => Ok(value.client.clone()),
            Self::None => Err(Payload::error("Unable to retrieve database client"))
        }
    }
}