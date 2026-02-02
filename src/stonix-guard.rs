use std::fs;
use std::path::Path;

const MAX_QUOTA_BYTES: u64 = 500 * 1024 * 1024;
const STORAGE_PATH: &str = "./data";

fn main() {
    println!("🛡️ Stonix Guard: Iniciando verificación...");
    if !Path::new(STORAGE_PATH).exists() {
        fs::create_dir_all(STORAGE_PATH).expect("Error");
    }
    match get_dir_size(STORAGE_PATH) {
        Ok(size) => {
            let usage_pct = (size as f64 / MAX_QUOTA_BYTES as f64) * 100.0;
            println!("📊 Uso: {} bytes ({:.2}%)", size, usage_pct);
            if size >= MAX_QUOTA_BYTES {
                eprintln!("⚠️ CUOTA EXCEDIDA");
                std::process::exit(1);
            }
            println!("✅ Estado: OPTIMAL.");
        }
        Err(e) => { eprintln!("❌ Error: {}", e); std::process::exit(1); }
    }
}
fn get_dir_size<P: AsRef<Path>>(path: P) -> std::io::Result<u64> {
    let mut size = 0;
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let metadata = entry.metadata().unwrap();
                size += if metadata.is_dir() { get_dir_size(entry.path()).unwrap_or(0) } else { metadata.len() };
            }
        }
    }
    Ok(size)
}
