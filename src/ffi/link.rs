//!
//! Module containing extern definitions of JNI functions
//!

use crate::ffi::types::*;

#[link(name = "jvm")]
extern "stdcall" {
    #[link_name = "JNI_GetDefaultJavaVMInitArgs"]
    pub fn get_default_jvm_init_args(args: *mut JavaVMInitArgs) -> JInt;
    #[link_name = "JNI_CreateJavaVM"]
    pub fn create_jvm(vm_loc: *mut *mut JavaVM, env_loc: *mut *mut JNIEnv, args: *mut JavaVMInitArgs) -> JInt;
    #[link_name = "JNI_GetCreatedJavaVMs"]
    pub fn get_created_jvms(vm_buf: *mut *mut JavaVM, len: JSize, total: *mut JSize) -> JInt;
}
