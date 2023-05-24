use actix_web::{ http::header, web::{ Bytes, BytesMut, Data }, HttpRequest, HttpResponse, Result };
use futures::TryStreamExt;
use handlebars::Handlebars;
use rusoto_core::credential::StaticProvider;
use rusoto_core::{ HttpClient, Region };
use rusoto_s3::{ GetObjectRequest, S3 as RusotoS3, S3Client };
use std::str::FromStr;

use crate::Payload;
use crate::S3;

use crate::catchers;
use crate::traits::IsEmpty;

/// S3 implementation
impl S3 {
    /// Get s3 client
    pub fn get_client(&self) -> Result<S3Client> {
        if self.is_empty() {
            return Err(Payload::error("AWS s3 is not configured"));
        }

        let access_key = self.access_key_id.clone().map_or(String::default(), |value| value.to_string());
        let secret_access_key = self.secret_access_key.clone().map_or(String::default(), |value| value.to_string());
        let region = match Region::from_str(&self.region.clone().map_or(String::default(), |value| value.to_string())) {
            Ok(region) => region,
            Err(_) => return Err(Payload::error("AWS region is not configured"))
        };

        // Set aws credentials
        let credentials = StaticProvider::new_minimal(access_key, secret_access_key);

        // Set client
        let client = S3Client::new_with(
            HttpClient::new().expect("Failed to create request dispatcher"),
            credentials,
            region,
        );

        Ok(client)
    }

    /// Retrieve file as HttpResponse
    pub async fn get<T>(&self, hbs: Data<Handlebars<'_>>, filename: T, req: HttpRequest) -> HttpResponse
        where T: ToString
    {
        // Create bindings
        let bindings = filename.to_string();

        // Retrieve client
        let client = match self.get_client() {
            Ok(client) => client,
            Err(_) => return catchers::not_found_page(hbs).await.unwrap()
        };

        // Retrieve item
        let object = match client.get_object(GetObjectRequest{
            bucket: self.bucket.clone().map_or(String::default(), |d| d.to_string()),
            key: format!("{}/{}", self.path.clone().map_or(String::default(), |d| d.to_string()).as_str(), bindings),
            ..Default::default()
        }).await {
            Ok(result) => result,
            Err(_) => return catchers::not_found_page(hbs).await.unwrap()
        };

        let body = match object.body {
            Some(body) => body,
            None => return catchers::not_found_page(hbs).await.unwrap()
        };

        // Set content type
        let content_type = object
            .content_type
            .unwrap_or_else(|| "application/octet-stream".to_owned());

        // Set response for requests containing range header
        if let Some(range_header) = req.headers().get(header::RANGE) {
            if let Ok((start, end)) = crate::parsers::headers::get_range(range_header) {
                let body_bytes = body
                    .map_ok(|b| BytesMut::from(&b[..]))
                    .try_concat()
                    .await;

                return match body_bytes {
                    Ok(b) => {
                        let accept_ranges = object.accept_ranges.unwrap_or_else(|| "bytes".to_owned());
                        let content_length = b.len();

                        HttpResponse::PartialContent()
                            .content_type(content_type)
                            .append_header((header::ACCEPT_RANGES, accept_ranges))
                            .append_header((header::CONTENT_LENGTH, content_length))
                            .append_header((
                                header::CONTENT_RANGE,
                                format!("bytes  {start}-{end}/{content_length}"),
                            ))
                            .body(Bytes::from(b[start..=end].to_vec()))
                    }
                    Err(_) => HttpResponse::InternalServerError().finish(),
                }
            }
        }

        HttpResponse::Ok()
            .content_type(content_type)
            .streaming(body.map_ok(|chunk| chunk))
    }
}