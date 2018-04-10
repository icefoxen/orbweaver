#![feature(proc_macro, wasm_custom_section, wasm_import_module)]
extern crate wasm_bindgen;

mod window;

use wasm_bindgen::prelude::*;

/*
#[wasm_bindgen]
pub fn greet() {
    window::Window::alert();
}
*/
