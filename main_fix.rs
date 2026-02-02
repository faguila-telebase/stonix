fn main() -> anyhow::Result<()> {
    // Eliminamos el '?' porque tu método 'new' devuelve el objeto directamente
    let mut db = Stonix::new("telarix.db");
    
    db.insert("empresa", "Cosentino")?;
    
    if let Some(val) = db.get("empresa")? {
        println!("🚀 Recuperado de índice: {}", val);
    }
    Ok(())
}
