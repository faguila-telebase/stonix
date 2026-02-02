use stonix::Stonix;

fn main() {
    // Cargamos Stonix usando el archivo de configuración externo
    let db = Stonix::new("stonix_config.json");
    let user = "admin_curro";

    println!("🚀 Stonix iniciado mediante manifiesto de configuración.");

    let data = r#"{"msg": "Configuración dinámica activa", "status": "OK"}"#;

    match db.put("config_test", data, user) {
        Ok(_) => {
            println!("✅ Datos guardados respetando las políticas del administrador.");
            if let Ok(content) = db.get("config_test", user) {
                println!("🔍 Contenido verificado: {}", content);
            }
        },
        Err(e) => println!("❌ Error de política: {}", e),
    }
}
