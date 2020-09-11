//!
//! Module containing smart references for common Object types.
//! They can be up- and down-cast to other Object types safely, and cannot outlive their intended
//! spans.
//!


use crate::ffi;
use crate::error::Error;
use crate::env::JNIEnv;
use crate::types::{JavaDownCast, JavaUpCast, JType, JNonVoidType};
use std::marker::PhantomData;


macro_rules! smart_obj {
    ($x:ident) => {

        ///
        /// A struct representing a pointer to the $x type, with added guarantee of memory-safety
        /// in usage.
        ///
        #[derive(Debug)]
        pub struct $x<'a> {
            backing_ptr: *mut ffi::$x,
            phantom: PhantomData<&'a ffi::$x>
        }

        impl $x<'_> {

            /// Create a new instance of this struct from a backing pointer
            pub fn new<'a>(ptr: *mut ffi::$x) -> Result<$x<'a>, Error> {
                if ptr.is_null() {
                    Err(Error::new(&format!("{} must be constructed from non-null pointer", stringify!($x)), ffi::constants::JNI_ERR))
                } else {
                    Ok($x {
                        backing_ptr: ptr,
                        phantom: PhantomData
                    })
                }
            }

            /// Get the backing pointer of this object. Unsafe, as this pointer may be used without
            /// the safety provided by this object
            pub unsafe fn borrow_ptr(&self) -> *mut ffi::$x {
                self.backing_ptr
            }

        }
    }
}


macro_rules! downcast {
    ($x:ident, $y:ident) => {
        impl<'a> JavaDownCast<$y<'a>> for $x<'a> {
            fn downcast(self) -> $y<'a> {
                $y::new(self.backing_ptr as *mut ffi::$y).unwrap()
            }
        }

        impl<'a, 'b> JavaDownCast<&'b $y<'a>> for &'b $x<'a> {
            fn downcast(self) -> &'b $y<'a> {
                // SAFETY: All the smart types have the same size + same backing pointer
                //         This is thus a safe cast
                unsafe {
                    &*(self as *const $x as *const $y)
                }
            }
        }
    }
}


macro_rules! upcast {
    ($x:ident, $y:ident) => {
        impl<'a> JavaUpCast<$y<'a>> for $x<'a> {
            fn upcast(self, _env: &JNIEnv) -> $y<'a> {
                // TODO: Do env check
                $y::new(self.backing_ptr as *mut ffi::$y).unwrap()
            }

            unsafe fn upcast_raw(self) -> $y<'a> {
                $y::new(self.backing_ptr as *mut ffi::$y).unwrap()
            }
        }

        impl<'a, 'b> JavaUpCast<&'b $y<'a>> for &'b $x<'a> {
            fn upcast(self, _env: &JNIEnv) -> &'b $y<'a> {
                // TODO: Do env check
                unsafe {
                    &*(self as *const $x as *const $y)
                }
            }

            unsafe fn upcast_raw(self) -> &'b $y<'a> {
                &*(self as *const $x as *const $y)
            }
        }
    }
}


///
/// A struct representing a Java Method ID. Knows its own return type and the number of args,
/// preventing memory unsafety while calling methods with it
///
/// TODO: Maybe preserve method name / staticness?
#[derive(Debug, PartialEq)]
pub struct JMethodID {
    real_id: *const ffi::JMethodID,
    ret_type: JType,
    num_args: usize
}

impl JMethodID {

    /// Create a new JMethodID from a raw MethodID, return type, and number of args
    pub fn new(id: *const ffi::JMethodID, ret: JType, num_args: usize) -> Result<JMethodID, Error> {
        if id.is_null() {
            Err(Error::new("JMethodID must be constructed from a non-null pointer", ffi::constants::JNI_ERR))
        } else {
            Ok(JMethodID {
                real_id: id,
                ret_type: ret,
                num_args
            })
        }
    }

    /// Get the return type of this method
    pub fn ret_ty(&self) -> JType {
        self.ret_type
    }

    /// Get the number of args in this method
    pub fn num_args(&self) -> usize {
        self.num_args
    }

    /// Get the backing pointer of this method. Unsafe, as this pointer may be used without the
    /// safety provided by this object
    pub unsafe fn borrow_ptr(&self) -> *const ffi::JMethodID {
        self.real_id
    }
}


///
/// A struct representing a Java Field ID. Knows its own type, preventing memory unsafety while
/// calling methods with it
///
/// TODO: Maybe preserve field name /  staticness?
#[derive(Debug, PartialEq)]
pub struct JFieldID {
    real_id: *const ffi::JFieldID,
    ty: JNonVoidType
}

impl JFieldID {

    /// Create a new JFieldID from a raw FieldID and type
    pub fn new(id: *const ffi::JFieldID, ty: JNonVoidType) -> Result<JFieldID, Error> {
        if id.is_null() {
            Err(Error::new("JFieldID must be constructed from a non-null pointer", ffi::constants::JNI_ERR))
        } else {
            Ok(JFieldID {
                real_id: id,
                ty
            })
        }
    }

    /// Get the type of this field
    pub fn ty(&self) -> JNonVoidType {
        self.ty
    }

    /// Get the backing pointer of this field. Unsafe, as this pointer may be used without the
    /// safety provided by this object
    pub unsafe fn borrow_ptr(&self) -> *const ffi::JFieldID {
        self.real_id
    }
}


smart_obj!(JObject);

smart_obj!(JThrowable);
smart_obj!(JClass);
smart_obj!(JString);
smart_obj!(JWeak);
smart_obj!(JArray);

smart_obj!(JObjectArray);
smart_obj!(JBooleanArray);
smart_obj!(JByteArray);
smart_obj!(JCharArray);
smart_obj!(JShortArray);
smart_obj!(JIntArray);
smart_obj!(JLongArray);
smart_obj!(JFloatArray);
smart_obj!(JDoubleArray);

upcast!(JObject, JThrowable);
upcast!(JObject, JClass);
upcast!(JObject, JString);
upcast!(JObject, JWeak);
upcast!(JObject, JArray);

downcast!(JThrowable, JObject);

downcast!(JClass, JObject);

downcast!(JString, JObject);

downcast!(JArray, JObject);

downcast!(JObjectArray, JObject);
downcast!(JObjectArray, JArray);

downcast!(JBooleanArray, JObject);
downcast!(JBooleanArray, JArray);

downcast!(JByteArray, JObject);
downcast!(JByteArray, JArray);

downcast!(JCharArray, JObject);
downcast!(JCharArray, JArray);

downcast!(JShortArray, JObject);
downcast!(JShortArray, JArray);

downcast!(JIntArray, JObject);
downcast!(JIntArray, JArray);

downcast!(JLongArray, JObject);
downcast!(JLongArray, JArray);

downcast!(JFloatArray, JObject);
downcast!(JFloatArray, JArray);

downcast!(JDoubleArray, JObject);
downcast!(JDoubleArray, JArray);
