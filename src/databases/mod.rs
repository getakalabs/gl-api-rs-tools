use actix_web::Result;
use diesel;
use diesel::pg::PgConnection;
use diesel::r2d2::{ Pool, PooledConnection, ConnectionManager, PoolError };
use std::env;

use crate::Errors;
use crate::Payload;

/// Create PgPool type which is basically a `Pool<ConnectionManager<PgConnection>>`
pub type PgPool = Pool<ConnectionManager<PgConnection>>;

/// Create PgPooledConnection type which is basically a `PooledConnection<ConnectionManager<PgConnection>>`
pub type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

/// Database manager pool enum which will allows the actix web server
/// to run with or without proper database connection
#[derive(Clone)]
pub enum DatabaseManager {
    Postgres(PgPool),
    None
}

/// DatabaseManager implementations
impl DatabaseManager {
    /// Set new DatabaseManager instance
    pub fn new(pool: PgPool) -> Self {
        Self::Postgres(pool)
    }

    /// Get database from r2d2 pool
    pub fn get(&self) -> Result<PgPooledConnection> {
        match self {
            Self::Postgres(_pool) => match _pool.get() {
                Ok(conn) => Ok(conn),
                Err(error) => Err(Payload::error(error))
            },
            _ => Err(Payload::error("Unable to initialize your database pool"))
        }
    }

    /// Parse database from async graphql context
    pub fn parse(ctx: &async_graphql::Context<'_>) -> async_graphql::Result<PgPooledConnection> {
        // Retrieve database
        if let Some(database) = ctx.data_opt::<DatabaseManager>() {
            if let Ok(database) = database.get() {
                return Ok(database);
            }
        }

        Err(Errors::internal_server_error_message("Unable to initialize database"))
    }
}

/// Returns a connection from the PgPool directly
pub fn pool_conn(pool: &PgPool) -> Result<PgPooledConnection, PoolError> {
    pool.get()
}

/// Connects to Postgres and call init pool
pub fn stage() -> Result<DatabaseManager> {
    // Set database url
    let database_url = match env::var("DATABASE_URL") {
        Ok(url) => url,
        Err(error) => return Err(Payload::error(error))
    };

    // Create a default R2D2 Postgres DB Pool
    match Pool::builder().build(ConnectionManager::<PgConnection>::new(database_url)) {
        Ok(pool) => Ok(DatabaseManager::new(pool)),
        Err(error) => Err(Payload::error(error))
    }
}