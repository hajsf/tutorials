// wasm/mod.rs
#[cfg(target_arch = "wasm32")]
pub mod wasm {
    //use wasm_bindgen::prelude::*;
    use crate::rust_muliplatforms;

   // #[wasm_bindgen]
    pub fn wasm_greetings(to: &str) -> String {
        rust_muliplatforms(to)
    }
}