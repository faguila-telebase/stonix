use stonix::Stonix;

fn main() {
    // Escenario: El administrador asigna solo 100 bytes a este entorno
    let quota = 100;
    let db = Stonix::new("./data", quota);
    let user = "admin_curro";

    println!("🛡️ Stonix iniciado con cuota de {} bytes.", quota);

    // Este texto tiene unos 115 bytes aprox, debería ser bloqueado
    let big_data = "Este es un texto que pretende ser bastante largo para superar el limite de cien bytes que hemos puesto arriba.";
    
    println!("尝试写入 (Intentando escribir)...");
    match db.put("over_limit_test", big_data, user) {
        Ok(_) => println!("✅ Escrito con éxito (¡Algo falló, no debería dejar!)"),
        Err(e) => println!("❌ Bloqueo de seguridad: {}", e),
    }
}
