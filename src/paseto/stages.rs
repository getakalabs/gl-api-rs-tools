use actix_web::Result;
use diesel::{ ExpressionMethods, QueryDsl, RunQueryDsl };
use std::sync::{ Arc, RwLock };

use crate::configs::Module;
use crate::configs::Settings;

use crate::Paseto;
use crate::PgPooledConnection;
use crate::traits::{Decrypt, Encrypt};

use crate::schema;

/// Paseto stage implementation
impl Paseto {
    /// Retrieve initial paseto configuration
    pub fn stage<T>(conn: &mut PgPooledConnection, name: T) -> Result<Arc<RwLock<Self>>>
        where T: ToString
    {
        // Set query
        let query = schema::settings::dsl::settings
            .filter(schema::settings::dsl::module.eq(Module::Paseto))
            .order(schema::settings::dsl::created_at.desc())
            .limit(1);

        // Retrieve settings
        if let Ok(data) = query.first::<Settings>(conn) {
            if let Some( data ) = data.paseto.unwrap_or_default().decrypt() {
                return Ok(Arc::new(RwLock::new(data)));
            }
        }

        // Return value
        match Self::setup(conn, name) {
            Ok(value) => Ok(value),
            Err(_) => Ok(Arc::new(RwLock::new(Self::default())))
        }
    }

    /// Setup initial paseto configuration
    pub fn setup<T>(conn: &mut PgPooledConnection, name: T) -> Result<Arc<RwLock<Self>>>
        where T: ToString
    {
        let paseto = Paseto::from(name.to_string());
        let settings = Settings::from(paseto)
            .encrypt()
            .unwrap_or_default();

        let result = diesel::insert_into(schema::settings::dsl::settings)
            .values(settings.clone())
            .execute(conn);

        if result.is_ok() {
            let settings = settings
                .decrypt()
                .unwrap_or_default()
                .paseto
                .unwrap_or_default();

            return Ok(Arc::new(RwLock::new(settings)));
        }

        Ok(Arc::new(RwLock::new(Self::default())))
    }
}