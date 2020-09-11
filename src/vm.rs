//!
//! Module containing a higher-level wrapper over a raw JVM. This higher-level implementation
//! can handle automatic creation/destruction, all the Invoke Interface functions, and tries to
//! ensure safety while doing so.
//!

use crate::{env, ffi};
use crate::error::Error;
use crate::ffi::{JavaVMInitArgs, JavaVMAttachArgs};
use crate::types::JNIVersion;
use crate::env::JNIEnv;

/// Higher-level construct representing a JVM
pub struct JavaVM {
    version: JNIVersion,
    main_vm: *mut ffi::JavaVM,
    owned: bool
}

impl JavaVM {

    /// Build a JVM instance from a version and pointer
    pub fn new(version: JNIVersion, vm: *mut ffi::JavaVM, owned: bool) -> Result<JavaVM, Error> {
        if vm.is_null() {
            Err(Error::new("JavaVM must be constructed from non-null pointer", ffi::constants::JNI_ERR))
        } else {
            Ok(JavaVM {
                version,
                main_vm: vm,
                owned
            })
        }
    }

    /// Create a new JVM. Initializes an entirely new JVM, with the current thread
    /// as the main thread. This object will call the JVM destroy function when it is dropped
    pub fn create(version: JNIVersion) -> Result<(JavaVM, JNIEnv), Error> {
        let mut main_vm = std::ptr::null_mut();
        let mut main_env = std::ptr::null_mut();
        let mut args = JavaVMInitArgs::new(version.into());

        // SAFETY: The FFI functions called here only rely on user input in checked cases, and
        //         will return error codes if the input provided here isn't right, which will be
        //         propagated as Err results.
        unsafe {
            let result = ffi::get_default_jvm_init_args(&mut args);
            if result != 0 {
                return Err(Error::new("Couldn't get default JVM args", result))
            }

            let result = ffi::create_jvm(&mut main_vm, &mut main_env, &mut args);
            if result != 0 {
                return Err(Error::new("Couldn't create JVM", result))
            }
        }

        if main_vm.is_null() || main_env.is_null() {
            Err(Error::new("Main VM or Global Environment null, despite successful JVM creation", ffi::constants::JNI_ERR))
        } else {
            let main_env = env::JNIEnv::new(main_env)?;
            Ok((JavaVM { version, main_vm, owned: true }, main_env))
        }
    }

    /// Get a list of existing JVMs. May error if the JNI returns an error code
    pub fn get_existing(version: JNIVersion) -> Result<Vec<JavaVM>, Error> {
        const BASE_QUANT: usize = 8;

        let mut main_vms: [*mut ffi::JavaVM; BASE_QUANT] = [std::ptr::null_mut(); BASE_QUANT];
        let mut total: i32 = 0;

        let result = unsafe {
            ffi::get_created_jvms(main_vms.as_mut_ptr(), BASE_QUANT as _, &mut total)
        };

        if result != 0 {
            Err(Error::new("Couldn't get list of existing JVM instances", result))
        } else {
            let mut out = Vec::new();
            for idx in 0..total as usize {
                out.push(JavaVM::new(version, main_vms[idx], false))
            }

            out.into_iter().collect()
        }
    }

    /// Non-public way to get a reference to the internal JVM pointer. Not unsafe only because it's
    /// not public.
    fn internal_vm(&self) -> &ffi::JavaVM {
        // SAFETY: The main_vm pointer is private, and only set to non-null values in checked locations
        unsafe {
            if let Some(vm) = self.main_vm.as_ref() {
                vm
            } else {
                panic!("Invalid JavaVM")
            }
        }
    }

    /// Get an owned object for the local thread's environment
    pub fn get_local_env(&self) -> Result<env::JNIEnv, Error> {
        let vm = self.internal_vm();

        let mut ffi_env = std::ptr::null_mut();
        let result = vm.get_env(&mut ffi_env, self.version.into());
        if result != 0 {
            return Err(Error::new("Couldn't get local environment", result))
        }

        env::JNIEnv::new(ffi_env)
    }

    /// Attach the current thread, and get an owned instance of the environment for it
    pub fn attach_current_thread(&self) -> Result<env::JNIEnv, Error> {
        let args = JavaVMAttachArgs::new(self.version.into());
        let vm = self.internal_vm();

        let mut ffi_env = std::ptr::null_mut();
        let result = vm.attach_current_thread(&mut ffi_env, &args);

        if result != 0 {
            Err(Error::new("Couldn't attach current thread to the JVM", result))
        } else {
            Ok(env::JNIEnv::new(ffi_env)?)
        }
    }

    /// Attach the current thread as a daemon, and get an owned instance of the environment for it
    pub fn attach_current_thread_daemon(&self) -> Result<env::JNIEnv, Error> {
        let args = JavaVMAttachArgs::new(self.version.into());
        let vm = self.internal_vm();

        let mut ffi_env = std::ptr::null_mut();
        let result = vm.attach_current_thread_as_daemon(&mut ffi_env, &args);

        if result != 0 {
            Err(Error::new("Couldn't attach current thread as daemon to the JVM", result))
        } else {
            Ok(env::JNIEnv::new(ffi_env)?)
        }
    }

    /// Detach the current thread, and give up the associated owned environment
    pub fn detach_current_thread(&self, _env: env::JNIEnv) -> Result<(), Error> {
        let vm = self.internal_vm();
        let result = vm.detach_current_thread();

        if result != 0 {
            Err(Error::new("Couldn't detach current thread from JVM", result))
        } else {
            Ok(())
        }
    }
}

impl Drop for JavaVM {
    fn drop(&mut self) {
        if self.owned {
            let vm = self.internal_vm();
            let result = vm.destroy_java_vm();
            if result != 0 {
                panic!(format!("JVM failed to shut down: {}", result));
            }
        }
    }
}

// JavaVM is Sync but not Send. It is tied to thread it's created in, but it is valid in all.
unsafe impl Sync for JavaVM {}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_jni_proc::java;
    use crate::tests::with_vm;
    use crate::types::{JavaDownCast, JObject, JClass, JString};
    use crate::env::JNIEnv;
    use crate::JThrowable;

    // Compile tests

    #[java(class = "java.lang.Foo", name = "Bar")]
    fn func(env: &JNIEnv) {

    }

    #[java(class = "java.lang.Foo", name = "Foo")]
    fn func(env: &JNIEnv, _this: JObject, _arg1: JClass, _arg2: Option<JString>) -> JObject {
        JObject::new(std::ptr::null_mut()).unwrap()
    }

    #[java(class = "java.lang.Foo")]
    fn TestName(env: &JNIEnv) -> Option<JObject> {
        None
    }

    #[java(class = "temp.foo.Foo")]
    fn func(env: &JNIEnv, _this: JObject) -> Result<JObject, Error> {
        Err(Error::new("", -1))
    }

    #[java(class = "temp.bar.Bar")]
    fn func(env: &JNIEnv, _this: JObject) -> Result<JObject, JThrowable> {
        Ok(JObject::new(std::ptr::null_mut()).unwrap())
    }

    fn check_cls(env: &JNIEnv, name: &str) {
        let cls = env.find_class(name);
        println!("{:?}", cls);
        cls.unwrap();
    }

    #[test]
    fn test() {
        with_vm(|vm| {
            let env = vm.attach_current_thread().expect("Couldn't attach test thread");

            let cls = env.find_class("java.lang.Class").unwrap();
            let method_id = env.get_static_method_id(&cls, "getPrimitiveClass", "(java.lang.String) -> java.lang.Class").unwrap();

            let str = env.new_string_utf("void").unwrap();
            let args = vec![str.downcast().into()];
            env.call_static_method(
                &cls,
                &method_id,
                &args
            ).unwrap().unwrap();

            use crate::{get_cls, get_method_id};

            let cls = get_cls!(env, "java.lang.String");
            let _id = get_method_id!(env, cls, "length", "() -> int");
            // let _id = get_static_method_id!(env, cls, "blah", "() -> void");
            // let _id = get_field_id!(env, cls, "field", "java.lang.String");
            // let _id = get_static_field_id!(env, cls, "field", "java.lang.String");

            let obj_buff;
            {
                let mut buff = [0x1, 0x2, 0x3];
                obj_buff = env.new_direct_byte_buffer(&mut buff).unwrap();
                env.get_direct_buffer_slice(&obj_buff).unwrap();
            }
        });
    }
}
