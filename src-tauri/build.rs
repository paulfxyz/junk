// build.rs — Tauri build script
//
// This file runs at *compile time* (via Cargo's build script system) before
// the main crate is compiled. It does two things:
//
//   1. Reads `tauri.conf.json` and validates it against the Tauri v2 schema.
//   2. Generates Rust code that embeds the frontend assets (HTML/CSS/JS from
//      the `../src/` directory) into the binary at release time, and sets up
//      the `generate_context!()` macro used in `main.rs`.
//
// Why do we need this?
//
// Tauri needs to know the window configuration, bundle identifier, plugin
// capabilities, and asset paths at compile time so it can:
//   - Embed the frontend into the binary (no separate webserver in production)
//   - Generate capability permission types the Rust code can reference
//   - Bake in the app version so it appears in `About` dialogs
//
// You almost never need to modify this file. The only reason to touch it is
// if you are doing something exotic like a custom asset embedding strategy or
// compile-time feature detection beyond what Tauri supports.
//
// Reference: https://v2.tauri.app/concept/build-script/

fn main() {
    // `tauri_build::build()` performs the code generation described above.
    // It panics with a helpful message if tauri.conf.json is missing or
    // malformed — so build errors surface clearly rather than as cryptic
    // linker errors downstream.
    tauri_build::build()
}
