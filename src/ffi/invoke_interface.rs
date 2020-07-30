
use std::ffi::c_void;
use crate::ffi::types::*;

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

    pub fn destroy_java_vm(&self) -> JInt {
        (self.get_functions().destroy_java_vm)(self)
    }

    pub fn attach_current_thread(&self, env: *mut *mut JNIEnv, args: *const JavaVMAttachArgs) -> JInt {
        (self.get_functions().attach_current_thread)(self, env, args)
    }

    pub fn detach_current_thread(&self) -> JInt {
        (self.get_functions().detach_current_thread)(self)
    }

    pub fn get_env(&self, env: *mut *mut JNIEnv, version: JInt) -> JInt {
        (self.get_functions().get_env)(self, env, version)
    }

    pub fn attach_current_thread_as_daemon(&self, env: *mut *mut JNIEnv, args: *const JavaVMAttachArgs) -> JInt {
        (self.get_functions().attach_current_thread_as_daemon)(self, env, args)
    }
}
