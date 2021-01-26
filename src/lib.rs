use wasm_bindgen::prelude::*;
use serde_wasm_bindgen::to_value;

mod dictentry;
mod fiero;
mod utils;

use dictentry::DictEntry;
use fiero::{Fiero, edit_distance};

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub struct Dictionary {
    entries: Vec<DictEntry>
}

#[wasm_bindgen]
pub fn parse_dict(rawfile: Box<[u8]>) -> Dictionary {
    let file = String::from_utf8(rawfile.into_vec()).expect("Invalid utf-8");
    let entries = dictentry::parse_dict(file);
    Dictionary {entries}
}

impl Dictionary {
    pub fn search_en(&self, query: &str) -> Vec<&DictEntry> {
        self.entries.iter().filter(|entry| entry.en_contains(query)).take(100).collect()
    }

    pub fn search_oj(&self, query: &str) -> Vec<&DictEntry> {
        let fiero_query = Fiero::parse(query);
        let edit_dist = |entry: &DictEntry| edit_distance(&entry.fiero, &fiero_query);

        let matches = utils::find_smallest(&self.entries, edit_dist, 50, 10);

        matches.iter().map(|x| x.1).collect()
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
