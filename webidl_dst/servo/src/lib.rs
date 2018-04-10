#![feature(proc_macro, wasm_custom_section, wasm_import_module)]
extern crate wasm_bindgen;

mod window;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
