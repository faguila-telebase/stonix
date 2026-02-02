use aes_gcm::{Aes256Gcm, Key, KeyInit, aead::{Aead, OsRng, AeadCore}};
use sha2::{Sha256, Digest};
use std::fs::{self, OpenOptions};
use std::io::{Write, Read};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use sysinfo::{System, Disks};

fn get_cipher() -> Option<Aes256Gcm> {
 let mut s = System::new_all();
 s.refresh_all();
 let hw_id = format!("{:?}{:?}", s.cpus().first()?.vendor_id(), System::host_name());
 let exe_p = std::env::current_exe().ok()?;
 let bin_b = fs::read(exe_p).ok()?;
 let mut hasher = Sha256::new();
 hasher.update(&bin_b);
 hasher.update(hw_id.as_bytes());
 let h = hasher.finalize();
 Some(Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&h[..32])))
}

#[no_mangle]
pub extern "C" fn stonix_put(key: *const c_char, val: *const c_char) -> i32 {
 let disks = Disks::new_with_refreshed_list();
 let free_space = disks.iter().next().map(|d| d.available_space()).unwrap_or(0);
 if free_space < 100 * 1024 * 1024 { return 2; }
 let k = unsafe { CStr::from_ptr(key) }.to_string_lossy();
 let v = unsafe { CStr::from_ptr(val) }.to_string_lossy();
 if let Some(c) = get_cipher() {
 let entry = format!("{}:{}", k, v);
 let n = Aes256Gcm::generate_nonce(&mut OsRng);
 if let Ok(ct) = c.encrypt(&n, entry.as_bytes()) {
 if let Ok(mut f) = OpenOptions::new().create(true).append(true).open("stonix.db") {
 let _ = f.write_all(&(ct.len() as u64).to_le_bytes());
 let _ = f.write_all(&n);
 let _ = f.write_all(&ct);
 return 0;
 }
 }
 }
 1
}

#[no_mangle]
pub extern "C" fn stonix_get_all() -> *mut c_char {
 let mut res = String::new();
 if let (Some(c), Ok(mut f)) = (get_cipher(), std::fs::File::open("stonix.db")) {
 let mut b = Vec::new(); let _ = f.read_to_end(&mut b);
 let mut i = 0;
 while i + 20 <= b.len() {
 let mut lb = [0u8; 8]; lb.copy_from_slice(&b[i..i+8]);
 let l = u64::from_le_bytes(lb) as usize; i += 8;
 let n = aes_gcm::Nonce::from_slice(&b[i..i+12]); i += 12;
 if i + l > b.len() { break; }
 if let Ok(p) = c.decrypt(n, &b[i..i+l]) { res.push_str(&String::from_utf8_lossy(&p)); }
 i += l;
 }
 }
 CString::new(res).unwrap_or(CString::new("").unwrap()).into_raw()
}

#[no_mangle]
pub extern "C" fn stonix_get_stats() -> *mut c_char {
 let disks = Disks::new_with_refreshed_list();
 let db_size = std::fs::metadata("stonix.db").map(|m| m.len()).unwrap_or(0);
 let free_space = disks.iter().next().map(|d| d.available_space()).unwrap_or(0);
 let stats = format!("{{\"db_size_bytes\": {}, \"disk_free_bytes\": {}}}", db_size, free_space);
 CString::new(stats).unwrap_or(CString::new("{}").unwrap()).into_raw()
}

#[no_mangle]
pub extern "C" fn stonix_free_string(s: *mut c_char) {
 unsafe { if !s.is_null() { let _ = CString::from_raw(s); } }
}
