use stonix::Stonix;

fn main() {
    let db = Stonix::new("./data");
    
    println!("🚀 Insertando registro en Stonix...");
    // Usamos r#"..."# para evitar problemas con las comillas del JSON
    let json_data = r#"{"name": "Francisco", "role": "Architect", "status": "Active"}"#;
    
    db.put("user_01", json_data).expect("Error al escribir");

    match db.get("user_01") {
        Ok(data) => println!("✅ Dato recuperado con éxito: {}", data),
        Err(e) => println!("❌ Error al leer los datos: {}", e),
    }
}
