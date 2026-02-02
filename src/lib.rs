use std::fs::{self, File, OpenOptions};
use std::io::{Write, Read};
use std::path::Path;
use std::os::unix::fs::PermissionsExt;

pub struct Stonix {
    storage_path: String,
}

impl Stonix {
    pub fn new(path: &str) -> Self {
        if !Path::new(path).exists() {
            fs::create_dir_all(path).expect("Error creando directorio");
        }
        Stonix { storage_path: path.to_string() }
    }

    pub fn put(&self, key: &str, value: &str, user: &str) -> std::io::Result<()> {
        let file_path = format!("{}/{}.db", self.storage_path, key);
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&file_path)?;

        // 🛡️ LOPD: Solo el propietario tiene acceso (rw-------)
        let metadata = file.metadata()?;
        let mut perms = metadata.permissions();
        perms.set_mode(0o600);
        fs::set_permissions(&file_path, perms)?;

        file.write_all(value.as_bytes())?;
        
        // 📝 Registro de auditoría simple
        println!("[AUDIT] User {} escribió en {}", user, key);
        Ok(())
    }

    pub fn get(&self, key: &str, user: &str) -> std::io::Result<String> {
        println!("[AUDIT] User {} intentó acceder a {}", user, key);
        let file_path = format!("{}/{}.db", self.storage_path, key);
        let mut file = File::open(file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(contents)
    }
}
