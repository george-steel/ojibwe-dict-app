use wasm_bindgen::prelude::*;
use serde::{Serialize};
use serde_wasm_bindgen::to_value;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[derive(Clone, Serialize)]
pub struct DictEntry {
    pub meta: String,
    pub oj: String,
    pub en: String,
}

#[wasm_bindgen]
pub struct Dictionary {
    entries: Vec<DictEntry>
}

#[wasm_bindgen]
pub fn parse_dict(rawfile: Box<[u8]>) -> Dictionary {
    let file = String::from_utf8(rawfile.into_vec()).expect("Invalid utf-8");
    let mut entries = Vec::new();

    for line in file.lines() {
        let fields: Vec<&str> = line.split('\t').collect();
        let entry = DictEntry{
            meta: String::from(fields[0]),
            oj: String::from(fields[1]),
            en: String::from(fields[2]) };
        entries.push(entry);
    }

    Dictionary {entries}
}

impl Dictionary {
    pub fn search_en(&self, query: &str) -> Vec<&DictEntry> {
        self.entries.iter().filter(|entry| entry.en.contains(query)).take(50).collect()
    }

    pub fn search_oj(&self, query: &str) -> Vec<&DictEntry> {
        self.entries.iter().filter(|entry| entry.oj.contains(query)).take(50).collect()
    }
}

#[wasm_bindgen]
impl Dictionary {
    pub fn search_en_js(&self, query: &str) -> JsValue {
        to_value(&self.search_en(query)).unwrap()
    }
    pub fn search_oj_js(&self, query: &str) -> JsValue {
        to_value(&self.search_oj(query)).unwrap()
    }
}



// Called when the wasm module is instantiated
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    log("Hello from Rust");
    Ok(())
}
