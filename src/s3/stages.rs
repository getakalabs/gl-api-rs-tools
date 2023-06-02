use actix_web::Result;
use diesel::{ ExpressionMethods, QueryDsl, RunQueryDsl };
use std::sync::{ Arc, RwLock };

use crate::configs::Module;
use crate::configs::Settings;

use crate::S3;
use crate::PgPooledConnection;
use crate::traits::Decrypt;

use crate::schema;

/// S3 stage implementation
impl S3 {
    /// Retrieve initial s3 configuration
    pub fn stage(conn: &mut PgPooledConnection) -> Result<Arc<RwLock<Self>>> {
        // Set query
        let query = schema::settings::dsl::settings
            .filter(schema::settings::dsl::module.eq(Module::S3))
            .order(schema::settings::dsl::created_at.desc())
            .limit(1);

        // Retrieve settings
        if let Ok(data) = query.first::<Settings>(conn) {
            if let Some( data ) = data.s3.unwrap_or_default().decrypt() {
                return Ok(Arc::new(RwLock::new(data)));
            }
        }

        // Return default value
        Ok(Arc::new(RwLock::new(Self::default())))
    }
}