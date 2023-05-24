use actix_web::{Result, get};
use actix_files::NamedFile;
use std::path::PathBuf;

const ASSET_PATH: &str = "./assets/static/media/favicon.ico";

// Create favicon fileserver handler
#[get("/favicon.ico/")]
pub async fn stage() -> Result<NamedFile> {
    let path: PathBuf = ASSET_PATH.to_string().parse().unwrap();
    Ok(NamedFile::open(path)?)
}