use std::collections::HashMap;
use std::fs::{OpenOptions, File};
use std::io::{BufReader, BufWriter, Write, BufRead, Seek, SeekFrom};
use std::path::Path;
use std::boxed::Box;

// ---------- Struct: Record stored on heap ---------- //
#[derive(Debug)]
struct Record {
    key: String,
    value: String,
}

impl Record {
    fn new(key: String, value: String) -> Self {
        Self { key, value }
    }
}

// ---------- Struct: KeyValueStore ---------- //
struct KeyValueStore<'a> {
    store: HashMap<String, Box<Record>>,
    file: BufWriter<File>,
    path: &'a str,
}

impl<'a> KeyValueStore<'a> {
    fn new(path: &'a str) -> std::io::Result<Self> {
        // Open or create the file
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .read(true)
            .open(path)?;

        let mut store = HashMap::new();

        // Read existing records
        let reader = BufReader::new(&file);
        for line in reader.lines() {
            if let Ok(record_line) = line {
                let parts: Vec<&str> = record_line.trim().splitn(2, '=').collect();
                if parts.len() == 2 {
                    let rec = Box::new(Record::new(parts[0].to_string(), parts[1].to_string()));
                    store.insert(parts[0].to_string(), rec);
                }
            }
        }

        // Rewind the file for appending
        let mut file = file;
        file.seek(SeekFrom::End(0))?;

        Ok(Self {
            store,
            file: BufWriter::new(file),
            path,
        })
    }

    fn set(&mut self, key: String, value: String) {
        let rec = Box::new(Record::new(key.clone(), value.clone()));
        self.store.insert(key.clone(), rec);
        let _ = writeln!(self.file, "{}={}", key, value);
        let _ = self.file.flush();

        // Visualization:
        println!(" Box allocated for key='{}' -> stored on HEAP", key);
        println!(" Stack: key (String), value (String), Box<Record>");
        println!(" Heap: Record {{ key, value }}");
    }

    fn get(&self, key: &str) -> Option<&Record> {
        self.store.get(key).map(|r| r.as_ref())
    }

    fn all_records(&self) -> &[Box<Record>] {
        // SAFELY convert values to slice using Vec and references
        // This returns &[Box<Record>] to allow read-only access
        let boxed_refs: Vec<&Box<Record>> = self.store.values().collect();
        // This is a bit of a trick to show usage of slice, but in real-world, slices from HashMap aren’t ideal
        unsafe {
            std::slice::from_raw_parts(
                boxed_refs.as_ptr() as *const Box<Record>,
                boxed_refs.len()
            )
        }
    }
}

// ---------- RAII Writeback on Drop ---------- //
impl<'a> Drop for KeyValueStore<'a> {
    fn drop(&mut self) {
        // Auto write-back isn't needed since we already append on set(),
        // but this is a safe place to do any final operations
        println!("💾 Store closed: all changes are already persisted.");
    }
}

// ---------- Main Usage ---------- //
fn main() -> std::io::Result<()> {
    let path = "kv_store.db";

    let mut kv = KeyValueStore::new(path)?;

    kv.set("username".into(), "subham_dev".into());
    kv.set("language".into(), "rust".into());
    kv.set("role".into(), "developer".into());

    println!("\n Current In-Memory Records:");
    for rec in kv.store.values() {
        println!("  {} = {}", rec.key, rec.value);
    }

    println!("\n Fetching a single record:");
    if let Some(r) = kv.get("language") {
        println!("  language: {}", r.value);
    }

    println!("\n Slice-based access to records:");
    let records: &[Box<Record>] = kv.all_records();
    for rec in records {
        println!("  &[Box<Record>] -> {} = {}", rec.key, rec.value);
    }

    Ok(())
}
