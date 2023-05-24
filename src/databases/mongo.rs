use mongodb::{ Client, Database };

/// MongoDB database instance
#[derive(Debug, Clone)]
pub struct MongoDB {
    pub client: Client,
    pub database: Database
}

/// Retrieve database instance from (client, String)
impl From<(Client, String)> for MongoDB {
    fn from((client, database): (Client, String)) -> Self {
        let database = client.database(&database);

        Self { client, database }
    }
}