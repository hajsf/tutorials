// android/mod.rs
#[cfg(target_os="android")]
#[allow(non_snake_case)]
pub mod android {
    use crate::char_str;
 //   extern crate jni;
 use crate::rust_muliplatforms;

    use self::jni::JNIEnv;
    use self::jni::objects::{JClass, JString};
    use self::jni::sys::{jstring};

    #[no_mangle]   // Hasan.RustMuliplatforms
    pub unsafe extern fn hasan_rustmuliplatforms(env: JNIEnv, _: JClass, java_pattern: JString) -> jstring {

        let jvm_input = env.get_string(java_pattern)
                                        .expect("invalid pattern string").as_ptr();

        let input = rust_muliplatforms(char_str(jvm_input));

        let output = env.new_string(input)
                                        .expect("Couldn't create java string!");
        output.into_inner()
    }
}