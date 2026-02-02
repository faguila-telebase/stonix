use std::fs::{self, File};
use std::io::{Write, Read};
use std::path::Path;

pub struct Stonix {
    storage_path: String,
}

impl Stonix {
    pub fn new(path: &str) -> Self {
        if !Path::new(path).exists() {
            fs::create_dir_all(path).expect("No se pudo crear el directorio de datos");
        }
        Stonix {
            storage_path: path.to_string(),
        }
    }

    pub fn put(&self, key: &str, value: &str) -> std::io::Result<()> {
        let file_path = format!("{}/{}.db", self.storage_path, key);
        let mut file = File::create(file_path)?;
        file.write_all(value.as_bytes())?;
        Ok(())
    }

    pub fn get(&self, key: &str) -> std::io::Result<String> {
        let file_path = format!("{}/{}.db", self.storage_path, key);
        let mut file = File::open(file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(contents)
    }
}
