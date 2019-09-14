// lib.rs
pub mod wasm;
pub mod ios;
pub mod android;

#[cfg(not(target_arch = "wasm32"))]
use std::os::raw::{c_char};

#[cfg(not(target_arch = "wasm32"))]
use std::ffi::{CStr};

pub fn rust_muliplatforms(to: &str) -> String {
    format!("Hello {}", to)
}

#[cfg(not(target_arch = "wasm32"))]
fn char_str(pattern: *const c_char) -> &'static str {

    let c_str = unsafe { CStr::from_ptr(pattern) };
    let string_ptr = match c_str.to_str() {
        Err(_) => "there",
        Ok(string) => string,
    };
    string_ptr
}