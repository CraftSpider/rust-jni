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
    ($x:ident, $y:literal) => {

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
                    Err(Error::new_null(&format!("{} Constructor", stringify!($x))))
                } else {
                    Ok($x {
                        backing_ptr: ptr,
                        phantom: PhantomData
                    })
                }
            }

            /// Get the java name associated with this type, if one exists
            pub fn get_java_name() -> &'static str {
                stringify!($y)
            }

            /// Get the backing pointer of this object. Unsafe, as this pointer may be used without
            /// the safety provided by this object
            pub unsafe fn borrow_ptr(&self) -> *mut ffi::$x {
                self.backing_ptr
            }

        }

    }
}


macro_rules! extends {
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
            fn upcast(self, env: &JNIEnv) -> $crate::error::Result<$y<'a>> {
                let self_name = Self::get_java_name();
                let cast_cls = env.find_class(self_name)?;
                let cls = env.get_object_class(&self)?;

                if !env.is_assignable_from(&cls, &cast_cls) {
                    Err(
                        $crate::error::Error::new(&format!("Can't assign to type {}", self_name), -1)
                    )
                } else {
                    $y::new(self.backing_ptr as *mut ffi::$y)
                }
            }

            unsafe fn upcast_raw(self) -> $y<'a> {
                $y::new(self.backing_ptr as *mut ffi::$y).unwrap()
            }
        }

        impl<'a, 'b> JavaUpCast<&'b $y<'a>> for &'b $x<'a> {
            fn upcast(self, _env: &JNIEnv) -> $crate::error::Result<&'b $y<'a>> {
                // TODO: Do env check
                unsafe {
                    Ok(&*(self as *const $x as *const $y))
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
            Err(Error::new_null("JMethodID Constructor"))
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
            Err(Error::new_null("JFieldID Constructor"))
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

smart_obj!(JObject, "[Ljava/lang/Object;");

smart_obj!(JThrowable, "[Ljava/lang/Throwable;");
smart_obj!(JClass, "[Ljava/lang/Class;");
smart_obj!(JString, "[Ljava/lang/String;");
smart_obj!(JWeak, "[Ljava/lang/ref/WeakReference;");
smart_obj!(JArray, "");

smart_obj!(JObjectArray, "[Ljava/lang/Object;");
smart_obj!(JBooleanArray, "[Z");
smart_obj!(JByteArray, "[B");
smart_obj!(JCharArray, "[C");
smart_obj!(JShortArray, "[S");
smart_obj!(JIntArray, "[I");
smart_obj!(JLongArray, "[J");
smart_obj!(JFloatArray, "[F");
smart_obj!(JDoubleArray, "[D");

upcast!(JObject, JThrowable);
upcast!(JObject, JClass);
upcast!(JObject, JString);
upcast!(JObject, JWeak);
upcast!(JObject, JArray);

extends!(JThrowable, JObject);

extends!(JClass, JObject);

extends!(JString, JObject);

extends!(JArray, JObject);

extends!(JObjectArray, JObject);
extends!(JObjectArray, JArray);

extends!(JBooleanArray, JObject);
extends!(JBooleanArray, JArray);

extends!(JByteArray, JObject);
extends!(JByteArray, JArray);

extends!(JCharArray, JObject);
extends!(JCharArray, JArray);

extends!(JShortArray, JObject);
extends!(JShortArray, JArray);

extends!(JIntArray, JObject);
extends!(JIntArray, JArray);

extends!(JLongArray, JObject);
extends!(JLongArray, JArray);

extends!(JFloatArray, JObject);
extends!(JFloatArray, JArray);

extends!(JDoubleArray, JObject);
extends!(JDoubleArray, JArray);
