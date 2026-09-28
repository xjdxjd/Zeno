use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm, Nonce
};
use argon2::Argon2;
use rand::RngCore;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Record {
    pub id: String,
    pub category: String,
    pub title: String,
    pub fields: std::collections::HashMap<String, String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Vault {
    pub records: Vec<Record>,
}

pub struct AppState {
    pub key: Mutex<Option<[u8; 32]>>,
    pub vault: Mutex<Option<Vault>>,
    pub vault_path: PathBuf,
}

impl Default for AppState {
    fn default() -> Self {
        let vault_path = dirs::data_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("zeno")
            .join("vault.enc");

        Self {
            key: Mutex::new(None),
            vault: Mutex::new(None),
            vault_path,
        }
    }
}

pub fn derive_key(password: &str, salt: &[u8]) -> [u8; 32] {
    let mut key = [0u8; 32];
    let argon2 = Argon2::default();
    argon2.hash_password_into(password.as_bytes(), salt, &mut key).unwrap();
    key
}

pub fn encrypt(data: &[u8], key: &[u8; 32]) -> Vec<u8> {
    let cipher = Aes256Gcm::new_from_slice(key).unwrap();
    let mut nonce_bytes = [0u8; 12];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);
    
    let mut result = nonce_bytes.to_vec();
    let encrypted = cipher.encrypt(nonce, data).unwrap();
    result.extend(encrypted);
    result
}

pub fn decrypt(data: &[u8], key: &[u8; 32]) -> Result<Vec<u8>, String> {
    if data.len() < 12 {
        return Err("Invalid data".to_string());
    }
    
    let cipher = Aes256Gcm::new_from_slice(key).unwrap();
    let nonce = Nonce::from_slice(&data[..12]);
    let ciphertext = &data[12..];
    
    cipher.decrypt(nonce, ciphertext).map_err(|e| e.to_string())
}

pub fn save_vault(path: &PathBuf, vault: &Vault, key: &[u8; 32]) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    
    let plaintext = serde_json::to_vec(vault).map_err(|e| e.to_string())?;
    let encrypted = encrypt(&plaintext, key);
    fs::write(path, encrypted).map_err(|e| e.to_string())
}
