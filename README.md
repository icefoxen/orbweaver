# orbweaver

A compiler to turn WebIDL specifications into Rust crates.

Specifically, the goal is to be able to suck in a WebIDL specification describing a browser's Javascript API, and output a `-jsys` FFI crate that exposes the browser's API to Rust, probably through `wasm_bindgen`

Current status: Fiddling around to see what's possible.

# Platform support

The `build.rs` uses Unix tools to fetch and extract webidl files from browser source code repos, so it only runs on Unix systems.

# WebIDL compatability issues.

## Firefox

webidl-rs issue: https://github.com/sgodwincs/webidl-rs/issues/14

Has non-standard `#ifdef`'s and `#endif`'s, ugh.  Easy to remove though.

Uses `implements` rather than `includes`

## Chrome/Webkit

https://trac.webkit.org/wiki/WebKitIDL

Also uses v1 webidl, with some extensions (sigh) which I haven't explored yet.

## Servo

Parses perfectly with `webidl`, it's what the crate maintainer has been testing against.

Its corpus of webidl files is about 1/4 the size of Firefox's though, in kilobytes at least, so there may be some missing
stuff in there.

## Chrome

???

## Edge

???
