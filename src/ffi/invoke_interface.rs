//!
//! Module containing the implementation of the JNIInvokeInterface method table,
//! as well as the JavaVM functions that rely on it
//!

use std::ffi::c_void;
use crate::ffi::types::*;

///
/// A struct representing the method table backing the JVM, the only part of the VM which isn't
/// opaque to the user
///
#[repr(C)]
pub struct JNIInvokeInterface {
    reserved0: *const c_void,
    reserved1: *const c_void,
    reserved2: *const c_void,

    destroy_java_vm: extern "system" fn(*const JavaVM) -> JInt,
    attach_current_thread: extern "system" fn(*const JavaVM, *mut *mut JNIEnv, *const JavaVMAttachArgs) -> JInt,
    detach_current_thread: extern "system" fn(*const JavaVM) -> JInt,

    get_env: extern "system" fn(*const JavaVM, *mut *mut JNIEnv, JInt) -> JInt,

    attach_current_thread_as_daemon: extern "system" fn(*const JavaVM, *mut *mut JNIEnv, *const JavaVMAttachArgs) -> JInt,
}

impl JavaVM {

    fn get_functions(&self) -> &JNIInvokeInterface {
        unsafe {
            self.functions.as_ref().expect("Invalid JavaVM")
        }
    }

    /// Wrapper for vm->DestroyJavaVM()
    pub fn destroy_java_vm(&self) -> JInt {
        (self.get_functions().destroy_java_vm)(self)
    }

    /// Wrapper for vm->AttachCurrentThread(...)
    pub fn attach_current_thread(&self, env: *mut *mut JNIEnv, args: *const JavaVMAttachArgs) -> JInt {
        (self.get_functions().attach_current_thread)(self, env, args)
    }

    /// Wrapper for vm->DetachCurrentThread()
    pub fn detach_current_thread(&self) -> JInt {
        (self.get_functions().detach_current_thread)(self)
    }

    /// Wrapper for vm->GetEnv(...)
    pub fn get_env(&self, env: *mut *mut JNIEnv, version: JInt) -> JInt {
        (self.get_functions().get_env)(self, env, version)
    }

    /// Wrapper for vm->AttachCurrentThreadAsDaemon(...)
    pub fn attach_current_thread_as_daemon(&self, env: *mut *mut JNIEnv, args: *const JavaVMAttachArgs) -> JInt {
        (self.get_functions().attach_current_thread_as_daemon)(self, env, args)
    }

}
