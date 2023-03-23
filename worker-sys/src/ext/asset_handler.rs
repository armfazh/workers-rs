use std::collections::HashMap;

use once_cell::sync::Lazy;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "__STATIC_CONTENT_MANIFEST")]
extern "C" {
    #[wasm_bindgen(js_name = "default")]
    static MANIFEST_STR: String;
}

pub static ASSET_MANIFEST: Lazy<HashMap<&str, &str>> =
    Lazy::new(|| serde_json::from_str(&MANIFEST_STR).unwrap_or_default());
