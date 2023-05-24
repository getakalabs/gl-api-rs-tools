pub(super) mod action;
pub(super) mod cipher;

pub use cipher::Cipher;

use anyhow::Result;
use rand::Rng;
use xsalsa20poly1305::aead::{ Aead, KeyInit };
use xsalsa20poly1305::aead::generic_array::{ GenericArray, typenum };
use xsalsa20poly1305::XSalsa20Poly1305;

/// Set nonce length
const NONCE_LENGTH: usize = 24;

/// Generate a random key encoded with base64 url
pub fn generate() -> String {
    base64_url::encode(&rand::thread_rng().gen::<[u8; 32]>())
}

/// Decrypt a content
pub fn decrypt<C, K>(content: C, key: K) -> Result<String>
    where C: ToString,
          K: ToString
{
    // Create bindings and then generate cipher instance
    let bindings = base64_url::decode(&std::env::var(key.to_string())?)?;
    let key = GenericArray::from_slice(&bindings);
    let cipher = XSalsa20Poly1305::new(key);

    // Set content
    let content = base64_url::decode(&content.to_string())?;
    if content.len() <= NONCE_LENGTH {
        return Err(anyhow::anyhow!("Invalid content length"));
    }

    // Split content
    let (nonce, content) = content.split_at(NONCE_LENGTH);

    // Set nonce & content
    let nonce:&GenericArray<u8, typenum::U24> = GenericArray::from_slice(nonce);
    let content = match cipher.decrypt(nonce, content) {
        Ok(content) => content,
        Err(_) => return Err(anyhow::anyhow!("Unable to decrypt content"))
    };

    // Return decrypted content
    Ok(String::from_utf8_lossy(&content).to_string())
}

/// Encrypt a content
pub fn encrypt<C, K>(content: C, key: K) -> Result<String>
    where C: ToString,
          K: ToString
{
    // Create bindings and then generate cipher instance
    let bindings = base64_url::decode(&std::env::var(key.to_string())?)?;
    let key = GenericArray::from_slice(&bindings);
    let cipher = XSalsa20Poly1305::new(key);

    // Set nonce
    let nonce = XSalsa20Poly1305::generate_nonce(&mut rand::rngs::OsRng);
    let content = match cipher.encrypt(&nonce, content.to_string().as_bytes()) {
        Ok(content) => content,
        Err(_) => return Err(anyhow::anyhow!("Unable to encrypt content"))
    };

    // Return encrypted content
    Ok(base64_url::encode(&[&nonce[..], &content[..]].concat()))
}