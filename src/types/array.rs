//!
//! Module containing types relevant to java arrays
//!


use crate::{ffi, JavaDownCast};
use crate::error::Result;
use crate::types::{
    JBooleanArray, JByteArray, JCharArray, JShortArray, JIntArray, JLongArray, JFloatArray,
    JDoubleArray, JArray,
    JBoolean, JByte, JChar, JShort, JInt, JLong, JFloat, JDouble
};
use crate::JNativeType;

///
/// An enum representing any java primitive array type. Can be converted into its backing reference
/// through [`as_jarray`][Self::as_jarray]
///
pub enum JNativeArray<'a> {
    /// Java primitive boolean array
    Boolean(JBooleanArray<'a>),
    /// Java primitive byte array
    Byte(JByteArray<'a>),
    /// Java primitive char array
    Char(JCharArray<'a>),
    /// Java primitive short array
    Short(JShortArray<'a>),
    /// Java primitive int array
    Int(JIntArray<'a>),
    /// Java primitive long array
    Long(JLongArray<'a>),
    /// Java primitive float array
    Float(JFloatArray<'a>),
    /// Java primitive double array
    Double(JDoubleArray<'a>)
}

impl JNativeArray<'_> {

    /// Create a new JNativeArray from a backing pointer and array type. Unsafe, as we trust that
    /// the native type is correct.
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

    /// Get the backing reference of this object as a generic JArray reference
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

    /// Get the JNativeType associated with this Array
    pub fn jtype(&self) -> JNativeType {
        match self {
            JNativeArray::Boolean(_) =>
                JNativeType::Boolean,
            JNativeArray::Byte(_) =>
                JNativeType::Byte,
            JNativeArray::Char(_) =>
                JNativeType::Char,
            JNativeArray::Short(_) =>
                JNativeType::Short,
            JNativeArray::Int(_) =>
                JNativeType::Int,
            JNativeArray::Long(_) =>
                JNativeType::Long,
            JNativeArray::Float(_) =>
                JNativeType::Float,
            JNativeArray::Double(_) =>
                JNativeType::Double,
        }
    }

}

///
/// An enum representing a slice of a java primitive array
///
pub enum JNativeSlice<'a> {
    /// Java primitive boolean slice
    Boolean(&'a mut [JBoolean]),
    /// Java primitive byte slice
    Byte(&'a mut [JByte]),
    /// Java primitive char slice
    Char(&'a mut [JChar]),
    /// Java primitive short slice
    Short(&'a mut [JShort]),
    /// Java primitive int slice
    Int(&'a mut [JInt]),
    /// Java primitive long slice
    Long(&'a mut [JLong]),
    /// Java primitive float slice
    Float(&'a mut [JFloat]),
    /// Java primitive double slice
    Double(&'a mut [JDouble])
}

impl<'a> JNativeSlice<'a> {

    /// Get the backing pointer of this object. Unsafe, as this pointer may be used without
    /// the safety provided by this object
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

    /// Get the JNativeType associated with this Slice
    pub fn jtype(&self) -> JNativeType {
        match self {
            JNativeSlice::Boolean(_) =>
                JNativeType::Boolean,
            JNativeSlice::Byte(_) =>
                JNativeType::Byte,
            JNativeSlice::Char(_) =>
                JNativeType::Char,
            JNativeSlice::Short(_) =>
                JNativeType::Short,
            JNativeSlice::Int(_) =>
                JNativeType::Int,
            JNativeSlice::Long(_) =>
                JNativeType::Long,
            JNativeSlice::Float(_) =>
                JNativeType::Float,
            JNativeSlice::Double(_) =>
                JNativeType::Double,
        }
    }
}

///
/// An enum representing a vector containing elements of one of the java primitive types
///
pub enum JNativeVec {
    /// Vector of boolean values
    Boolean(Vec<bool>),
    /// Vector of byte values
    Byte(Vec<i8>),
    /// Vector of char values
    Char(Vec<char>),
    /// Vector of short values
    Short(Vec<i16>),
    /// Vector of int values
    Int(Vec<i32>),
    /// Vector of long values
    Long(Vec<i64>),
    /// Vector of float values
    Float(Vec<f32>),
    /// Vector of double values
    Double(Vec<f64>)
}

impl JNativeVec {

    /// Get the JNativeType associated with this Vec
    pub fn jtype(&self) -> JNativeType {
        match self {
            JNativeVec::Boolean(_) =>
                JNativeType::Boolean,
            JNativeVec::Byte(_) =>
                JNativeType::Byte,
            JNativeVec::Char(_) =>
                JNativeType::Char,
            JNativeVec::Short(_) =>
                JNativeType::Short,
            JNativeVec::Int(_) =>
                JNativeType::Int,
            JNativeVec::Long(_) =>
                JNativeType::Long,
            JNativeVec::Float(_) =>
                JNativeType::Float,
            JNativeVec::Double(_) =>
                JNativeType::Double,
        }
    }

}

///
/// An enum representing the various modes used in releasing a java array region
///
pub enum ReleaseMode {
    /// Copy the values back to the array, then free the slice
    CopyFree,
    /// Copy the values back to the array, but don't free the slice
    Commit,
    /// Don't copy the values back to the array, free the slice
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
