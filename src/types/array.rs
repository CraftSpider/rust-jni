
use crate::{ffi, JavaDownCast};
use crate::error::Result;
use crate::types::{
    JBooleanArray, JByteArray, JCharArray, JShortArray, JIntArray, JLongArray, JFloatArray,
    JDoubleArray, JArray,
    JBoolean, JByte, JChar, JShort, JInt, JLong, JFloat, JDouble
};
use crate::JNativeType;

pub enum JNativeArray<'a> {
    Boolean(JBooleanArray<'a>),
    Byte(JByteArray<'a>),
    Char(JCharArray<'a>),
    Short(JShortArray<'a>),
    Int(JIntArray<'a>),
    Long(JLongArray<'a>),
    Float(JFloatArray<'a>),
    Double(JDoubleArray<'a>)
}

impl JNativeArray<'_> {

    pub unsafe fn new_raw<'a>(arr: *mut ffi::JArray, ty: JNativeType) -> Result<JNativeArray<'a>> {
        match ty {
            JNativeType::Boolean => {
                Ok(JNativeArray::Boolean(JBooleanArray::new(arr as *mut ffi::JBooleanArray)?))
            }
            JNativeType::Byte => {
                Ok(JNativeArray::Byte(JByteArray::new(arr as *mut ffi::JByteArray)?))
            }
            JNativeType::Char => {
                Ok(JNativeArray::Char(JCharArray::new(arr as *mut ffi::JCharArray)?))
            }
            JNativeType::Short => {
                Ok(JNativeArray::Short(JShortArray::new(arr as *mut ffi::JShortArray)?))
            }
            JNativeType::Int => {
                Ok(JNativeArray::Int(JIntArray::new(arr as *mut ffi::JIntArray)?))
            }
            JNativeType::Long => {
                Ok(JNativeArray::Long(JLongArray::new(arr as *mut ffi::JLongArray)?))
            }
            JNativeType::Float => {
                Ok(JNativeArray::Float(JFloatArray::new(arr as *mut ffi::JFloatArray)?))
            }
            JNativeType::Double => {
                Ok(JNativeArray::Double(JDoubleArray::new(arr as *mut ffi::JDoubleArray)?))
            }
        }
    }

    pub fn as_jarray(&self) -> &JArray {
        match self {
            JNativeArray::Boolean(arr) => {
                arr.downcast()
            }
            JNativeArray::Byte(arr) => {
                arr.downcast()
            }
            JNativeArray::Char(arr) => {
                arr.downcast()
            }
            JNativeArray::Short(arr) => {
                arr.downcast()
            }
            JNativeArray::Int(arr) => {
                arr.downcast()
            }
            JNativeArray::Long(arr) => {
                arr.downcast()
            }
            JNativeArray::Float(arr) => {
                arr.downcast()
            }
            JNativeArray::Double(arr) => {
                arr.downcast()
            }
        }
    }

}

pub enum JNativeSlice<'a> {
    Boolean(&'a mut [JBoolean]),
    Byte(&'a mut [JByte]),
    Char(&'a mut [JChar]),
    Short(&'a mut [JShort]),
    Int(&'a mut [JInt]),
    Long(&'a mut [JLong]),
    Float(&'a mut [JFloat]),
    Double(&'a mut [JDouble])
}

impl<'a> JNativeSlice<'a> {
    pub unsafe fn borrow_ptr(&self) -> *mut std::ffi::c_void {
        match self {
            JNativeSlice::Boolean(slice) =>
                slice.as_ptr() as _,
            JNativeSlice::Byte(slice) =>
                slice.as_ptr() as _,
            JNativeSlice::Char(slice) =>
                slice.as_ptr() as _,
            JNativeSlice::Short(slice) =>
                slice.as_ptr() as _,
            JNativeSlice::Int(slice) =>
                slice.as_ptr() as _,
            JNativeSlice::Long(slice) =>
                slice.as_ptr() as _,
            JNativeSlice::Float(slice) =>
                slice.as_ptr() as _,
            JNativeSlice::Double(slice) =>
                slice.as_ptr() as _
        }
    }
}

pub enum JNativeVec {
    Boolean(Vec<bool>),
    Byte(Vec<i8>),
    Char(Vec<char>),
    Short(Vec<i16>),
    Int(Vec<i32>),
    Long(Vec<i64>),
    Float(Vec<f32>),
    Double(Vec<f64>)
}

pub enum ReleaseMode {
    CopyFree,
    Commit,
    Abort
}

impl Into<JInt> for ReleaseMode {
    fn into(self) -> JInt {
        match self {
            ReleaseMode::CopyFree => {
                0
            }
            ReleaseMode::Commit => {
                crate::ffi::constants::JNI_COMMIT
            }
            ReleaseMode::Abort => {
                crate::ffi::constants::JNI_ABORT
            }
        }
    }
}
