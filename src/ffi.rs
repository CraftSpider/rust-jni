
pub mod native_interface;
pub mod invoke_interface;
pub mod types;
pub mod link;
pub mod constants;

pub use native_interface::JNINativeInterface;
pub use invoke_interface::JNIInvokeInterface;
pub use types::*;
pub use link::*;


#[cfg(test)]
mod tests {
    use super::*;
    use std::ptr;
    use std::ffi::CString;

    #[test]
    #[ignore]
    fn test() {
        println!("Running test");
        let mut vm_ptr: *mut JavaVM = ptr::null_mut();
        let mut env_ptr: *mut JNIEnv = ptr::null_mut();

        println!("Entering unsafe block");
        unsafe {
            let mut init_args = JavaVMInitArgs::new(crate::ffi::constants::JNI_VERSION_1_8);

            let result = get_default_jvm_init_args(&mut init_args);
            println!("Get Default: {}", result);

            let result = create_jvm(&mut vm_ptr, &mut env_ptr, &mut init_args);
            println!("Create VM: {}", result);
            if result != 0 {
                assert!(false)
            }

            println!("VM: {:?}", vm_ptr);
            println!("Env: {:?}", env_ptr);

            let env = env_ptr.as_ref().unwrap();
            let version = env.get_version();
            println!("Version: {:X}", version);

            let name = CString::new("java/lang/Object").unwrap();
            let result = env.find_class(name.as_ptr());
            println!("String Class: {:?}", result);

            let vm = vm_ptr.as_mut().unwrap();
            vm.destroy_java_vm();
        }

        println!("Test complete");
    }

}
