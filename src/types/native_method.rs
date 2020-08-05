use crate::{ffi, JavaType};
use std::ffi::{c_void, CString};

pub struct JNINativeMethod {
    name: CString,
    signature: CString,
    ptr: *mut c_void
}


impl JNINativeMethod {

    /// TODO: Replace with extern "system" fn once varargs are a thing
    pub fn new<U: JavaType>(name: &str, signature: &str, fn_ptr: *mut c_void) -> JNINativeMethod {
        // SAFETY: As long as signature is correct, JNI doesn't actually use this internal type
        JNINativeMethod {
            name: CString::new(name).expect("Expected valid CString"),
            signature: CString::new(signature).expect("Expected valid CString"),
            ptr: fn_ptr
        }
    }

    pub fn make_ffi_vec(slice: &[JNINativeMethod]) -> Vec<ffi::JNINativeMethod> {
        let mut out = Vec::new();

        for i in 0..slice.len() {
            unsafe {
                out.push(slice[i].as_ffi())
            }
        }

        out
    }

    pub unsafe fn as_ffi(&self) -> ffi::JNINativeMethod {
        ffi::JNINativeMethod::new(
            self.name.as_ptr(),
            self.signature.as_ptr(),
            self.ptr
        )
    }
}