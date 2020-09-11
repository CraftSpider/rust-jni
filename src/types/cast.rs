//!
//! Module containing traits used for up/down-casting JObject types safely
//!

use crate::env::JNIEnv;

///
/// Trait representing the ability for this reference to be 'downcast', and treated as a reference
/// to a parent type
///
pub trait JavaDownCast<T> {
    /// Does a safe cast to a Java type that this type inherits from
    fn downcast(self) -> T;
}


///
/// Trait representing the ability for this reference to be 'upcast', and treated as a reference
/// to a child type
///
pub trait JavaUpCast<T> {
    /// Does a safe cast to a Java type that inherits from this type
    fn upcast(self, env: &JNIEnv) -> T;

    /// Does an unsafe cast to a Java type that inherits from this type
    unsafe fn upcast_raw(self) -> T;
}
