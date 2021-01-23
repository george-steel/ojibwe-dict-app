use std::collections::HashMap;
use serde::{Serialize};
use crate::fiero::Fiero;

#[derive(Clone, PartialOrd, Ord, PartialEq, Eq, Serialize)]
pub struct OJWord {
    pub word: String,
    pub suffix: Option<String>,
    pub meta: Option<String>,
}

#[derive(PartialOrd, Ord, PartialEq, Eq, Serialize)]
pub struct DictEntry {
    #[serde(skip)]
    pub fiero: Vec<Fiero>,
    pub oj: OJWord,
    pub en: Vec<String>,
}

impl OJWord {
    fn parse(raw_word: &str, raw_meta: &str) -> Self{
        let mut iter = raw_word.split('+');
        let word = String::from(iter.next().unwrap());
        let suffix = iter.next().map(String::from);
        let meta = if raw_meta.is_empty() {None} else {Some(String::from(raw_meta))};
        OJWord{word, suffix, meta}
    }

    pub fn contains(&self, query: &str) -> bool {
        match &self.suffix {
            None => self.word.contains(query),
            Some(pl) => self.word.contains(query) || (self.word.clone() + &pl).contains(query),
        }
    }
}

impl DictEntry {
    pub fn en_contains(&self, query: &str) -> bool {
        self.en.iter().any(|s| {s.contains(query)})
    }
}

pub fn parse_dict(raw_dict: String) -> Vec<DictEntry> {
    let mut entries: HashMap<(&str, &str), Vec<String>> = HashMap::new();

    for line in raw_dict.lines() {
        let fields: Vec<&str> = line.split('\t').collect();
        if fields.len() != 3 {continue;}
        entries.entry((fields[1],fields[0])).or_insert(Vec::with_capacity(1)).push(String::from(fields[2]));
    }

    let mut dict: Vec<DictEntry> = entries.into_iter().map( |((oj, meta), en)| {
        let oj = OJWord::parse(oj, meta);
        let fiero = Fiero::parse(&oj.word);
        DictEntry {fiero, oj, en}
    }).collect();
    drop(raw_dict);
    dict.sort_unstable();
    dict
}