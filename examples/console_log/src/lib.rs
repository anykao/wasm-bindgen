#![feature(proc_macro)]

extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn run() {
    console::log("Hello from Rust!");
}
