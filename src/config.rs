use serde::{Serialize, Deserialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
pub struct StonixConfig {
    pub storage_path: String,
    pub quota_bytes: u64,
    pub audit_enabled: bool,
    pub strict_permissions: bool,
}

impl StonixConfig {
    pub fn load(path: &str) -> Self {
        let config_str = fs::read_to_string(path).unwrap_or_else(|_| {
            r#"{
                "storage_path": "./data",
                "quota_bytes": 104857600,
                "audit_enabled": true,
                "strict_permissions": true
            }"#.to_string()
        });
        serde_json::from_str(&config_str).expect("Error al parsear config")
    }
}
