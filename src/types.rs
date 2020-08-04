
pub mod object;
pub mod array;
pub mod jtype;
pub mod value;
pub mod version;
pub mod cast;
pub mod native_method;

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

pub trait JavaType {}

impl JavaType for JBoolean {}
impl JavaType for JByte {}
impl JavaType for JChar {}
impl JavaType for JShort {}
impl JavaType for JInt {}
impl JavaType for JLong {}
impl JavaType for JFloat {}
impl JavaType for JDouble {}
impl JavaType for *mut super::ffi::JObject {}
