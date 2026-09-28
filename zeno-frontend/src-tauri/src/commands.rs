use crate::crypto::*;
use tauri::{Manager, State};
use std::cmp::Reverse;
use std::collections::HashMap;
use chrono::Utc;
use uuid::Uuid;
use rand::Rng;
use base64::Engine;
use base64::engine::general_purpose::STANDARD;

// 隐藏快速搜索窗口（自定义命令，绕过核心窗口 API 的 ACL 授权）
#[tauri::command]
pub fn hide_quick(app: tauri::AppHandle) {
    if let Some(win) = app.get_webview_window("quick") {
        let _ = win.hide();
    }
}

#[tauri::command]
pub fn check_vault_exists(state: State<'_, AppState>) -> bool {
    state.vault_path.exists()
}

#[tauri::command]
pub fn setup_master_password(state: State<'_, AppState>, password: String) -> Result<(), String> {
    let salt: [u8; 16] = rand::random();
    let key = derive_key(&password, &salt);
    
    let config_path = state.vault_path.parent().unwrap().join("config.json");
    std::fs::create_dir_all(config_path.parent().unwrap()).map_err(|e| e.to_string())?;
    let config = serde_json::json!({
        "salt": STANDARD.encode(salt),
        "key": STANDARD.encode(key),
    });
    std::fs::write(&config_path, serde_json::to_string_pretty(&config).unwrap()).map_err(|e| e.to_string())?;
    
    let vault = Vault { records: vec![] };
    save_vault(&state.vault_path, &vault, &key)?;
    
    *state.key.lock().unwrap() = Some(key);
    *state.vault.lock().unwrap() = Some(vault);
    
    Ok(())
}

#[tauri::command]
pub fn auto_unlock(state: State<'_, AppState>) -> Result<(), String> {
    let config_path = state.vault_path.parent().unwrap().join("config.json");
    if !config_path.exists() {
        return Err("保险库未初始化".to_string());
    }
    
    let config_data = std::fs::read_to_string(&config_path).map_err(|e| e.to_string())?;
    let config: serde_json::Value = serde_json::from_str(&config_data).map_err(|e| e.to_string())?;
    let key_bytes = STANDARD.decode(config["key"].as_str().unwrap()).map_err(|e| e.to_string())?;
    
    let mut key = [0u8; 32];
    key.copy_from_slice(&key_bytes);
    
    let encrypted_data = std::fs::read(&state.vault_path).map_err(|e| e.to_string())?;
    let decrypted = decrypt(&encrypted_data, &key).map_err(|_| "解密失败".to_string())?;
    let vault: Vault = serde_json::from_slice(&decrypted).map_err(|e| e.to_string())?;
    
    *state.key.lock().unwrap() = Some(key);
    *state.vault.lock().unwrap() = Some(vault);
    
    Ok(())
}

#[tauri::command]
pub fn get_records(state: State<'_, AppState>) -> Result<Vec<Record>, String> {
    let vault = state.vault.lock().unwrap();
    match &*vault {
        Some(v) => {
            let mut records = v.records.clone();
            records.sort_by_key(|r| Reverse(r.updated_at));
            Ok(records)
        }
        None => Err("保险库未解锁".to_string()),
    }
}

#[tauri::command]
pub fn add_record(
    state: State<'_, AppState>,
    category: String,
    title: String,
    fields: HashMap<String, String>,
) -> Result<Record, String> {
    let record = Record {
        id: Uuid::new_v4().to_string(),
        category,
        title,
        fields,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };
    
    let key = state.key.lock().unwrap().ok_or("保险库未解锁")?;
    let mut vault = state.vault.lock().unwrap();
    if let Some(ref mut v) = *vault {
        v.records.push(record.clone());
        save_vault(&state.vault_path, v, &key)?;
    } else {
        return Err("保险库未解锁".to_string());
    }
    
    Ok(record)
}

#[tauri::command]
pub fn update_record(
    state: State<'_, AppState>,
    id: String,
    title: String,
    fields: HashMap<String, String>,
) -> Result<Record, String> {
    let key = state.key.lock().unwrap().ok_or("保险库未解锁")?;
    let mut vault = state.vault.lock().unwrap();
    if let Some(ref mut v) = *vault {
        if let Some(record) = v.records.iter_mut().find(|r| r.id == id) {
            record.title = title;
            record.fields = fields;
            record.updated_at = Utc::now();
            let updated = record.clone();
            save_vault(&state.vault_path, v, &key)?;
            return Ok(updated);
        }
        Err("记录不存在".to_string())
    } else {
        Err("保险库未解锁".to_string())
    }
}

#[tauri::command]
pub fn delete_record(state: State<'_, AppState>, id: String) -> Result<(), String> {
    let key = state.key.lock().unwrap().ok_or("保险库未解锁")?;
    let mut vault = state.vault.lock().unwrap();
    if let Some(ref mut v) = *vault {
        v.records.retain(|r| r.id != id);
        save_vault(&state.vault_path, v, &key)?;
        Ok(())
    } else {
        Err("保险库未解锁".to_string())
    }
}

#[tauri::command]
pub fn search_records(state: State<'_, AppState>, query: String) -> Result<Vec<Record>, String> {
    let vault = state.vault.lock().unwrap();
    match &*vault {
        Some(v) => {
            let query_lower = query.to_lowercase();
            let mut filtered: Vec<Record> = v.records.iter()
                .filter(|r| {
                    r.title.to_lowercase().contains(&query_lower) ||
                    r.fields.iter().any(|(k, val)| {
                        k != "密码" && k.to_lowercase() != "password" 
                            && val.to_lowercase().contains(&query_lower)
                    })
                })
                .cloned()
                .collect();
            filtered.sort_by_key(|r| Reverse(r.updated_at));
            Ok(filtered)
        }
        None => Err("保险库未解锁".to_string()),
    }
}

#[tauri::command]
pub fn generate_password(
    length: u32,
    uppercase: bool,
    lowercase: bool,
    numbers: bool,
    symbols: bool,
) -> String {
    let mut charset = String::new();
    if uppercase { charset.push_str("ABCDEFGHIJKLMNOPQRSTUVWXYZ"); }
    if lowercase { charset.push_str("abcdefghijklmnopqrstuvwxyz"); }
    if numbers { charset.push_str("0123456789"); }
    if symbols { charset.push_str("!@#$%^&*()_+-=[]{}|;:,.<>?"); }
    
    if charset.is_empty() {
        charset = "abcdefghijklmnopqrstuvwxyz".to_string();
    }
    
    let mut password = String::new();
    let mut rng = rand::thread_rng();
    for _ in 0..length {
        let idx = rng.gen_range(0..charset.len());
        password.push(charset.chars().nth(idx).unwrap());
    }
    password
}
