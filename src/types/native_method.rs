use crate::ffi;
use std::intrinsics::transmute;
use std::ffi::c_void;

pub trait JavaType {}

impl JavaType for ffi::JBoolean {}
impl JavaType for ffi::JByte {}
impl JavaType for ffi::JChar {}
impl JavaType for ffi::JShort {}
impl JavaType for ffi::JInt {}
impl JavaType for ffi::JLong {}
impl JavaType for ffi::JFloat {}
impl JavaType for ffi::JDouble {}
impl JavaType for *mut ffi::JObject {}

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