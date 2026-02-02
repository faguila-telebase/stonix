use std::collections::HashMap;
use std::fs::{OpenOptions, File};
use std::io::{Read, Write, Seek, SeekFrom};
use anyhow::Result;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Record { k: String, v: String }

pub struct Stonix { path: String, index: HashMap<String, u64> }

impl Stonix {
    pub fn new(p: &str) -> Self {
        let mut db = Self { path: p.to_string(), index: HashMap::new() };
        let _ = db.load_index();
        db
    }
    fn load_index(&mut self) -> Result<()> {
        let f_res = File::open(&self.path);
        if f_res.is_err() { return Ok(()); }
        let mut f = f_res?;
        let mut o = 0; let fl = f.metadata()?.len();
        while o < fl {
            f.seek(SeekFrom::Start(o))?;
            let mut lb = [0u8; 4];
            if f.read_exact(&mut lb).is_err() { break; }
            let l = u32::from_le_bytes(lb) as u64;
            let mut rb = vec![0u8; l as usize];
            f.read_exact(&mut rb)?;
            let r: Record = serde_json::from_slice(&rb)?;
            self.index.insert(r.k, o);
            o += 4 + l;
        }
        Ok(())
    }

    pub fn get(&mut self, k: &str) -> Result<Option<String>> {
        if let Some(&o) = self.index.get(k) {
            let mut f = File::open(&self.path)?;
            f.seek(SeekFrom::Start(o))?;
            let mut lb = [0u8; 4];
            f.read_exact(&mut lb)?;
            let l = u32::from_le_bytes(lb) as usize;
            let mut rb = vec![0u8; l];
            f.read_exact(&mut rb)?;
            let r: Record = serde_json::from_slice(&rb)?;
            return Ok(Some(r.v));
        }
        Ok(None)
    }

    pub fn insert(&mut self, k: &str, v: &str) -> Result<()> {
        let mut f = OpenOptions::new().append(true).create(true).open(&self.path)?;
        let o = f.metadata()?.len();
        let d = serde_json::to_vec(&Record { k: k.to_string(), v: v.to_string() })?;
        f.write_all(&(d.len() as u32).to_le_bytes())?;
        f.write_all(&d)?;
        self.index.insert(k.to_string(), o);
        Ok(())
    }
}

fn main() -> Result<()> {
    let mut db = Stonix::new("telarix.db");
    db.insert("infraestructura", "Vectorware")?;
    if let Some(v) = db.get("infraestructura")? {
        println!("🚀 Stonix Engine [O(1)] -> Recuperado: {}", v);
    }
    Ok(())
}
