use std::sync::{ Arc, RwLock };

use crate::Base;
use crate::Errors;

impl Base {
    /// Parse from async graphql context
    pub fn parse<'a>(ctx: &'a async_graphql::Context<'_>) -> async_graphql::Result<&'a Arc<RwLock<Self>>> {
        match ctx.data_opt::<Arc<RwLock<Self>>>() {
            Some(settings) => Ok(settings),
            None =>  Err(Errors::internal_server_error_message("Unable to initialize base configuration"))
        }
    }
}