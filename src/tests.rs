use crate::{JNIEnv, JNIVersion, JavaVM};

/// Create and return a static reference to a JVM
fn create_leak_jvm() -> &'static mut JavaVM {
    let jvm = match JavaVM::create(JNIVersion::Ver18) {
        Ok(jvm) => jvm.0,
        Err(e) => panic!("{}", e)
    };

    Box::leak(Box::new(jvm))
}

/// Gets an existing JVM if one has been setup, otherwise creates a new one.
/// It then passes to the provided closure
pub fn with_vm<F>(f: F)
    where
        F: FnOnce(&mut JavaVM)
{
    let mut existing = JavaVM::get_existing(JNIVersion::Ver18).expect("Failed to get existing VMs");

    let jvm = if !existing.is_empty() {
        &mut existing[0]
    } else {
        create_leak_jvm()
    };

    f(jvm)
}

/// Gets an existing JVM if one has been setup, otherwise creates a new one.
/// It then ensures the current thread is attached, and passes the env from that attach
/// to the provided closure
pub fn with_env<F>(f: F)
    where
        F: FnOnce(&mut JNIEnv)
{
    let mut existing = JavaVM::get_existing(JNIVersion::Ver18).expect("Failed to get existing VMs");

    let jvm = if !existing.is_empty() {
        &mut existing[0]
    } else {
        create_leak_jvm()
    };

    let mut env = jvm.attach_current_thread().expect("Couldn't attach test thread");

    f(&mut env);
}
