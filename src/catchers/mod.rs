use actix_web::{HttpResponse, Result, web};
use actix_web::http::{header::{CacheControl, CacheDirective}, StatusCode};
use handlebars::Handlebars;

use crate::Payload;

/// Set options
const CACHE_DIRECTIVES: u32 = 86400u32;
const MIME_HTML: &str = "text/html; charset=utf-8";
const TEMPLATE_404_PATH: &str = "error/404.html";

/// Create response page
fn http_response_page(hbs: web::Data<Handlebars<'_>>, status_code: StatusCode) -> Result<HttpResponse> {
    let body = match hbs.render(TEMPLATE_404_PATH, &None::<String>) {
        Ok(body) => body,
        Err(error) => return Err(Payload::error(error))
    };

    let builder = HttpResponse::build(status_code)
        .insert_header(CacheControl(vec![
            CacheDirective::Public,
            CacheDirective::MaxAge(CACHE_DIRECTIVES),
        ]))
        .content_type(MIME_HTML)
        .body(body);

    Ok(builder)
}

/// Generate not found page
pub async fn not_found_page(hbs: web::Data<Handlebars<'_>>) -> Result<HttpResponse> {
    http_response_page(hbs, StatusCode::NOT_FOUND)
}

/// Create not found json response
pub async fn not_found_json() -> Payload {
    Payload {
        code: Some(404),
        error: Some(String::from("Not Found")),
        ..Default::default()
    }
}

/// Create not found middleware
pub fn not_found_middleware(hbs: web::Data<Handlebars<'_>>) -> HttpResponse {
    // Set body
    let body = match hbs.render(TEMPLATE_404_PATH, &None::<String>) {
        Ok(body) => body,
        Err(error) => error.to_string()
    };

    // Return http response
    HttpResponse::NotFound()
        .content_type(MIME_HTML.to_string())
        .insert_header(CacheControl(vec![
            CacheDirective::Public,
            CacheDirective::MaxAge(CACHE_DIRECTIVES),
        ]))
        .body(serde_json::to_string(&body).unwrap())
}