use std::ops::Index;
use anyhow::Result;
use std::str::FromStr;
use actix_web::http::header::HeaderValue;

use crate::BearerToken;

// Parse range header to get start and end
// Author: Deneir
pub fn get_range(range_header: &HeaderValue) -> Result<(usize, usize)> {
    let s = range_header.to_str().unwrap();
    let prefix = "bytes=";

    if !s.starts_with(prefix) {
        return Err(anyhow::anyhow!("Range header doesn't start with 'bytes='"));
    }

    let split = (s[prefix.len()..])
        .split('-')
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    if split.len() != 2 {
        return Err(anyhow::anyhow!("Range header has an invalid format"));
    }

    let start = usize::from_str(split.index(0))?;
    let end = usize::from_str(split.index(1))?;

    Ok((start, end))
}

// Parse graphql token
pub fn parse_graphql_token(req: &actix_web::HttpRequest, gql: async_graphql_actix_web::GraphQLRequest) -> async_graphql::Request {
    // Create new request context
    let mut request = gql.into_inner();

    // Retrieve bearer token if available
    if let Some(token) = get_bearer_token(req) {
        request = request.data(token);
    }

    // Return request
    request
}

// Parse token header
pub fn get_bearer_token(req: &actix_web::HttpRequest) -> Option<BearerToken> {
    if let Some(token) = req.headers().get("Authorization") {
        if let Ok(token) = token.to_str() {
            if let Some(token) = extract_bearer_token(token) {
                return Some(BearerToken::new(token));
            }
        }
    }

    None
}

// Extract bearer token
fn extract_bearer_token(auth_header: &str) -> Option<String> {
    const BEARER_PREFIX: &str = "Bearer ";

    // Check if the auth header starts with the "Bearer " prefix
    if let Some(stripped) = auth_header.strip_prefix(BEARER_PREFIX) {
        // Extract the token value after the prefix
        let token = stripped.trim();

        // Check if the token value is not empty
        if !token.is_empty() {
            return Some(token.to_string());
        }
    }

    None
}