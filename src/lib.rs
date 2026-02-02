pub mod config;
use config::StonixConfig;
use std::fs::{self, File, OpenOptions};
use std::io::{Write, Read};
use std::path::Path;
use std::os::unix::fs::PermissionsExt;

pub struct Stonix {
    config: StonixConfig,
}

impl Stonix {
    pub fn new(config_path: &str) -> Self {
        let config = StonixConfig::load(config_path);
        if !Path::new(&config.storage_path).exists() {
            fs::create_dir_all(&config.storage_path).expect("Error creando directorio");
        }
        Stonix { config }
    }

    fn get_storage_size(&self) -> u64 {
        fs::read_dir(&self.config.storage_path).unwrap().filter_map(|entry| {
            entry.ok().and_then(|e| e.metadata().ok()).map(|m| m.len())
        }).sum()
    }

    pub fn put(&self, key: &str, value: &str, user: &str) -> std::io::Result<()> {
        let current_size = self.get_storage_size();
        if current_size + value.len() as u64 > self.config.quota_bytes {
            if self.config.audit_enabled {
                println!("[ALERT] Cuota excedida para {}. Capacidad: {} bytes", user, self.config.quota_bytes);
            }
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Cuota de almacenamiento excedida"));
        }

        let file_path = format!("{}/{}.db", self.config.storage_path, key);
        let mut file = OpenOptions::new().create(true).write(true).truncate(true).open(&file_path)?;

        if self.config.strict_permissions {
            let mut perms = file.metadata()?.permissions();
            perms.set_mode(0o600);
            fs::set_permissions(&file_path, perms)?;
        }

        file.write_all(value.as_bytes())?;
        if self.config.audit_enabled {
            println!("[AUDIT] User {} escribió en {} (Uso: {}/{})", user, key, current_size + value.len() as u64, self.config.quota_bytes);
        }
        Ok(())
    }

    pub fn get(&self, key: &str, user: &str) -> std::io::Result<String> {
        if self.config.audit_enabled {
            println!("[AUDIT] User {} accedió a {}", user, key);
        }
        let mut file = File::open(format!("{}/{}.db", self.config.storage_path, key))?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(contents)
    }
}
