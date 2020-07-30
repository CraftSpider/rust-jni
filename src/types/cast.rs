
use crate::env::JNIEnv;

pub trait JavaDownCast<T> {
    /// Does a safe cast to a Java type that this type inherits from
    fn downcast(self) -> T;
}


pub trait JavaUpCast<T> {
    /// Does a safe cast to a Java type that inherits from this type
    fn upcast(self, env: &JNIEnv) -> T;

    /// Does an unsafe cast to a Java type that inherits from this type
    unsafe fn upcast_raw(self) -> T;
}
