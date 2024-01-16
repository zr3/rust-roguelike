use std::slice::Iter;
use wasm_bindgen::prelude::*;

pub struct GameLog {
    entries: Vec<String>,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

impl GameLog {
    pub fn new(entries: Vec<String>) -> GameLog {
        GameLog { entries }
    }
    pub fn log(&mut self, entry: String) {
        #[allow(unused_unsafe)]
        unsafe {
            log(&entry);
        }
        self.entries.push(entry.clone());
    }
    pub fn entries(&self) -> Iter<String> {
        self.entries.iter()
    }
}
