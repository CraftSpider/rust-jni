use crate::{ffi, JavaType};
use std::intrinsics::transmute;
use std::ffi::c_void;

pub struct JNINativeMethod {
    name: String,
    signature: String,
    ptr: *mut c_void
}


impl JNINativeMethod {
    // TODO: Replace with extern "system" fn once varargs are a thing
    pub fn new<U: JavaType>(name: &str, signature: &str, fn_ptr: *mut c_void) -> JNINativeMethod {
        // SAFETY: As long as signature is correct, JNI doesn't actually use this internal type
        JNINativeMethod {
            name: String::from(name),
            signature: String::from(signature),
            ptr: fn_ptr
        }
    }

    pub fn make_ffi_vec(slice: &[JNINativeMethod]) -> Vec<ffi::JNINativeMethod> {
        unimplemented!()
    }

    pub unsafe fn as_ffi(&self) -> ffi::JNINativeMethod {
        unimplemented!()
    }
}