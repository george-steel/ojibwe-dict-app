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
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum SearchMode {
    WholeWord,
    QueryInWord,
    WordInQuery,
}

#[wasm_bindgen]
pub fn parse_dict(rawfile: Box<[u8]>) -> Dictionary {
    let file = String::from_utf8(rawfile.into_vec()).expect("Invalid utf-8");
    let entries = dictentry::parse_dict(file);
    Dictionary {entries}
}

impl Dictionary {
    pub fn search_en(&self, query: &str) -> Vec<&DictEntry> {
        let ciquery = query.to_lowercase();
        self.entries.iter().filter(|entry| entry.en_contains(&ciquery)).take(100).collect()
    }

    pub fn search_oj(&self, query: &str, search_mode: SearchMode) -> Vec<&DictEntry> {
        let fiero_query = Fiero::parse(query);
        let edit_dist = |entry: &DictEntry| match search_mode {
            SearchMode::WholeWord => edit_distance(&fiero_query, &entry.fiero, None),
            SearchMode::QueryInWord => edit_distance(&fiero_query, &entry.fiero, Some(1)),
            SearchMode::WordInQuery => edit_distance(&entry.fiero, &fiero_query, Some(100)),
        };

        let matches = utils::find_smallest(&self.entries, edit_dist, 50, 0);

        matches.iter().map(|x| x.1).collect()
    }
}

#[wasm_bindgen]
impl Dictionary {
    pub fn search_en_js(&self, query: &str) -> JsValue {
        to_value(&self.search_en(query)).unwrap()
    }
    pub fn search_oj_js(&self, query: &str, search_mode: SearchMode) -> JsValue {
        to_value(&self.search_oj(query, search_mode)).unwrap()
    }
    pub fn size(&self) -> usize {
        self.entries.len()
    }
    pub fn to_tsv(&self) -> String {
        dictentry::dict_to_tsv(&self.entries)
    }
}



// Called when the wasm module is instantiated
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    log("Hello from Rust 2");
    Ok(())
}
