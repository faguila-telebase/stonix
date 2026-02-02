use aes_gcm::{Aes256Gcm, Key, KeyInit, aead::{Aead, OsRng, AeadCore}};
use sha2::{Sha256, Digest};
use std::env;
use std::fs::{self, OpenOptions};
use std::io::{Write, Read};
use std::path::Path;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    let version = "1.2.0-Stonix-GCM";
    let db_path = "stonix.db";
    
    // --- PROTOCOLO DE AUTENTICACIÓN AUTOMÁTICA (Sello Maestro) ---
    // El Sello Maestro se genera a partir del hash del propio binario en ejecución.
    let path = env::current_exe()?;
    let exe_bytes = fs::read(path)?;
    let hash = Sha256::digest(&exe_bytes);
    let key = Key::<Aes256Gcm>::from_slice(&hash[..32]);
    let cipher = Aes256Gcm::new(key);

    println!("--- STONIX {} | ARCH: {} ---", version, env::consts::ARCH);

    if args.len() < 2 {
        println!("Uso: ./stonix [put <clave> <valor> | get | del]");
        return Ok(());
    }

    match args[1].as_str() {
        "put" => {
            if args.len() < 4 { 
                return Err(anyhow::anyhow!("Error: Se requiere clave y valor.")); 
            }
            // Sanitización básica de tokens para blindaje contra inyección
            let k = args[2].chars().filter(|c| c.is_alphanumeric() || *c == '_').collect::<String>();
            let v = args[3].chars().filter(|c| c.is_alphanumeric() || " .-_".contains(*c)).collect::<String>();
            
            let entry = format!("{}:{}\n", k, v);
            let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
            let ciphertext = cipher.encrypt(&nonce, entry.as_bytes())
                .map_err(|_| anyhow::anyhow!("Fallo crítico en cifrado GCM"))?;

            let mut file = OpenOptions::new().create(true).append(true).open(db_path)?;
            
            // Estructura del registro: [Tamaño Ciphertext (u64)] + [Nonce (12b)] + [Ciphertext]
            let len = ciphertext.len() as u64;
            file.write_all(&len.to_le_bytes())?;
            file.write_all(&nonce)?;
            file.write_all(&ciphertext)?;
            
            println!("[ GUARDADO ] Registro '{}' cifrado y sanitizado.", k);
        },

        "get" => {
            if !Path::new(db_path).exists() {
                println!("Bóveda vacía.");
                return Ok(());
            }

            let mut file = fs::File::open(db_path)?;
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer)?;

            println!("--- BÓVEDA TELEBASE ---");
            let mut cursor = 0;
            while cursor < buffer.len() {
                // Leer tamaño del ciphertext
                let mut len_bytes = [0u8; 8];
                len_bytes.copy_from_slice(&buffer[cursor..cursor+8]);
                let len = u64::from_le_bytes(len_bytes) as usize;
                cursor += 8;

                // Leer Nonce
                let nonce = aes_gcm::Nonce::from_slice(&buffer[cursor..cursor+12]);
                cursor += 12;

                // Leer Ciphertext
                let ciphertext = &buffer[cursor..cursor+len];
                cursor += len;

                // Descifrar con el Sello Maestro
                if let Ok(plaintext) = cipher.decrypt(nonce, ciphertext) {
                    print!("{}", String::from_utf8_lossy(&plaintext));
                } else {
                    println!("[ BLOQUEADO ] Sello Maestro alterado o datos corruptos.");
                    break;
                }
            }
        },

        "del" => {
            if Path::new(db_path).exists() {
                fs::remove_file(db_path)?;
                println!("[ RESET ] Bóveda eliminada físicamente.");
            }
        },

        _ => println!("Comando no reconocido. Use put, get o del."),
    }

    Ok(())
}
