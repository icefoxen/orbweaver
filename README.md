# orbweaver

A compiler to turn WebIDL specifications into Rust crates.

Specifically, the goal is to be able to suck in a WebIDL specification describing a browser's Javascript API, and output a `-jsys` FFI crate that exposes the browser's API to Rust, probably through `wasm_bindgen`

Current status: Fiddling around to see what's possible.

# Odd things found so far

## Firefox

Has non-standard `#ifdef`'s and `#endif`'s, ugh

Uses `implements` rather than `includes`