use stonix::Stonix;

fn main() {
    let db = Stonix::new("./data");
    let current_user = "admin_curro";

    println!("🛡️ Iniciando Stonix con control de acceso...");
    
    let json_data = r#"{"secret": "Clave-Privada-Telebase-2026", "audit": "LOPD-Ready"}"#;
    
    // Ahora pasamos el usuario para el registro de auditoría
    db.put("secret_data", json_data, current_user).expect("Error al escribir");

    match db.get("secret_data", current_user) {
        Ok(data) => println!("✅ Acceso autorizado. Contenido: {}", data),
        Err(e) => println!("❌ Acceso denegado o error: {}", e),
    }
}
