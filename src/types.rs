//!
//! Module containing higher-level wrapper JNI types, used by the safe abstractions provided by this
//! library
//!

// Public modules

pub mod object;
pub mod array;
pub mod jtype;
pub mod value;
pub mod version;
pub mod cast;
pub mod native_method;

// Public re-exports

pub use object::{
    JMethodID, JFieldID,
    JObject, JThrowable, JString, JClass, JArray, JObjectArray, JBooleanArray, JByteArray,
    JCharArray, JShortArray, JIntArray, JLongArray, JFloatArray, JDoubleArray
};

pub use array::{
    JNativeArray, JNativeSlice, JNativeVec, ReleaseMode
};

pub use jtype::{JType, JNonVoidType, JNativeType};

pub use value::JValue;

pub use version::JNIVersion;

pub use cast::{JavaUpCast, JavaDownCast};

pub use native_method::JNINativeMethod;

pub use super::ffi::{JBoolean, JByte, JChar, JShort, JInt, JLong, JFloat, JDouble};

// Marker trait for types that are valid for use in JNI functions

// TODO: Find a way to use this with proc macro, for compile-time type checking???
pub unsafe trait JavaType {}

unsafe impl JavaType for JBoolean {}
unsafe impl JavaType for JByte {}
unsafe impl JavaType for JChar {}
unsafe impl JavaType for JShort {}
unsafe impl JavaType for JInt {}
unsafe impl JavaType for JLong {}
unsafe impl JavaType for JFloat {}
unsafe impl JavaType for JDouble {}
unsafe impl JavaType for *mut super::ffi::JObject {}
