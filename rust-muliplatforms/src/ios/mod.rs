// ios/mod.rs
#[cfg(target_os="ios")]
#[no_mangle]
pub mod ios {
  //  use crate::char_str;
  use crate::rust_muliplatforms;

    use std::ffi::{CString, CStr};
    use std::os::raw::{c_char};

    #[no_mangle]
    pub extern fn ios_greetings(to: *const c_char) -> *mut c_char {

        let input = rust_muliplatforms(char_str(to));

        CString::new(input).unwrap().into_raw()
    }

    pub extern fn iso_greeting_free(s: *mut c_char) {
        unsafe {
            if s.is_null() { return }
            CString::from_raw(s)
        };
    }
}