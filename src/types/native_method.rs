//!
//! Module containing a struct and methods related to the RegisterNatives functionality
//!

use crate::{ffi, JavaType};
use std::ffi::{c_void, CString};

///
/// A struct representing a single native method. Contains the name of the method, the signature
/// of the method, and the pointer to the actual Rust function to be invoked when the method is
/// called in the JVM
///
pub struct JNINativeMethod {
    name: CString,
    signature: CString,
    ptr: *mut c_void
}

impl JNINativeMethod {

    /// Create a new NativeMethod from a name, signature, and function to be called when the method
    /// is invoked
    ///
    /// TODO: Replace with extern "system" fn once varargs are a thing
    pub fn new<U: JavaType>(name: &str, signature: &str, fn_ptr: *mut c_void) -> JNINativeMethod {
        // SAFETY: As long as signature is correct, JNI doesn't actually use this internal type
        JNINativeMethod {
            name: CString::new(name).expect("Expected valid CString"),
            signature: CString::new(signature).expect("Expected valid CString"),
            ptr: fn_ptr
        }
    }

    /// Create a vector of the FFI-safe JNINativeMethod type from a slice of JNINativeMethods
    pub fn make_ffi_vec(slice: &[JNINativeMethod]) -> Vec<ffi::JNINativeMethod> {
        let mut out = Vec::new();

        for i in 0..slice.len() {
            unsafe {
                out.push(slice[i].as_ffi())
            }
        }

        out
    }

    /// Get this JNINativeMethod as the FFI-safe JNINativeMethod struct
    pub unsafe fn as_ffi(&self) -> ffi::JNINativeMethod {
        ffi::JNINativeMethod::new(
            self.name.as_ptr(),
            self.signature.as_ptr(),
            self.ptr
        )
    }
}