use actix_web::Result;
use diesel::{ ExpressionMethods, QueryDsl, RunQueryDsl };
use std::sync::{ Arc, RwLock };

use crate::configs::Module;
use crate::configs::Settings;

use crate::Mailer;
use crate::PgPooledConnection;
use crate::traits::Decrypt;

use crate::schema;

/// Mailer stage implementation
impl Mailer {
    /// Retrieve initial mailer configuration
    pub fn stage(conn: &mut PgPooledConnection) -> Result<Arc<RwLock<Self>>> {
        // Set query
        let query = schema::settings::dsl::settings
            .filter(schema::settings::dsl::module.eq(Module::Mailer))
            .order(schema::settings::dsl::created_at.desc())
            .limit(1);

        // Retrieve settings
        if let Ok(data) = query.first::<Settings>(conn) {
            if let Some( data ) = data.mailer.unwrap_or_default().decrypt() {
                return Ok(Arc::new(RwLock::new(data)));
            }
        }

        // Return default value
        Ok(Arc::new(RwLock::new(Self::default())))
    }
}