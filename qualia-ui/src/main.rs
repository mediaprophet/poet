//! Binary entry point for WASM build (trunk builds --bin).
//!
//! Copyright (c) 2026 Timothy Charles Holborn. All rights reserved.

fn main() {
    // The #[wasm_bindgen(start)] attribute in browser::start()
    // is the actual WASM entry point. This binary target exists
    // so that trunk can build it with `cargo build --bin qualia-ui`.
    qualia_ui::browser::start();
}
