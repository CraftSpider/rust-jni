//!
//! Module containing definitions of FFI-safe JNI types
//!

use std::slice;
use std::ffi::{c_void, CString};
use std::fmt::{Debug, Formatter};
use std::alloc::Layout;
use crate::ffi::{JNINativeInterface, JNIInvokeInterface, constants};
use crate::error::Error;

/// Semi-opaque struct for the JNIEnv variable in interfaces
#[repr(transparent)]
pub struct JNIEnv {
    /// Function dispatch table for JNIEnv instances
    pub functions: *const JNINativeInterface
}

/// Semi-opaque struct for the JavaVM variable in interfaces
#[repr(transparent)]
pub struct JavaVM {
    /// Function dispatch table for JavaVM instances
    pub functions: *const JNIInvokeInterface
}

/// Real type for JByte on the JVM
pub type JByte = i8;
/// Real type for JShort on the JVM
pub type JShort = i16;
/// Real type for JInt on the JVM
pub type JInt = i32;
/// Real type for JLong on the JVM
pub type JLong = i64;

/// Real type for JBoolean on the JVM
pub type JBoolean = bool;

/// Real type for JChar on the JVM
pub type JChar = u16;

/// Real type for JFloat on the JVM
pub type JFloat = f32;
/// Real type for JDouble on the JVM
pub type JDouble = f64;

/// Real type for JSize on the JVM
pub type JSize = JInt;

/// Opaque type for field IDs
#[repr(C)]
pub struct JFieldID { _priv: [u8; 0] }

/// Opaque type for method IDs
#[repr(C)]
pub struct JMethodID { _priv: [u8; 0] }

/// Opaque type for JVM objects
#[repr(C)]
pub struct JObject { _priv: [u8; 0] }

/// Opaque type for JVM weak references
#[repr(C)]
pub struct JWeak { _pric: [u8; 0] }

/// Opaque type for JVM class objects
#[repr(C)]
pub struct JClass { _priv: [u8; 0] }

/// Opaque type for JVM throwable objects
#[repr(C)]
pub struct JThrowable { _priv: [u8; 0] }

/// Opaque type for JVM string objects
#[repr(C)]
pub struct JString { _priv: [u8; 0] }

/// Opaque type for JVM array objects
#[repr(C)]
pub struct JArray { _priv: [u8; 0] }

/// Opaque type for JVM boolean arrays
#[repr(C)]
pub struct JBooleanArray { _priv: [u8; 0] }

/// Opaque type for JVM byte arrays
#[repr(C)]
pub struct JByteArray { _priv: [u8; 0] }

/// Opaque type for JVM char arrays
#[repr(C)]
pub struct JCharArray { _priv: [u8; 0] }

/// Opaque type for JVM short arrays
#[repr(C)]
pub struct JShortArray { _priv: [u8; 0] }

/// Opaque type for JVM int arrays
#[repr(C)]
pub struct JIntArray { _priv: [u8; 0] }

/// Opaque type for JVM long arrays
#[repr(C)]
pub struct JLongArray { _priv: [u8; 0] }

/// Opaque type for JVM float arrays
#[repr(C)]
pub struct JFloatArray { _priv: [u8; 0] }

/// Opaque type for JVM double arrays
#[repr(C)]
pub struct JDoubleArray { _priv: [u8; 0] }

/// Opaque type for JVM object arrays
#[repr(C)]
pub struct JObjectArray { _priv: [u8; 0] }

/// An FFI-safe union of valid argument types
#[repr(C)]
pub union JValue {
    /// Java primitive boolean value
    pub z: JBoolean,
    /// Java primitive byte value
    pub b: JByte,
    /// Java primitive char value
    pub c: JChar,
    /// Java primitive short value
    pub s: JShort,
    /// Java primitive int value
    pub i: JInt,
    /// Java primitive long value
    pub j: JLong,
    /// Java primitive float value
    pub f: JFloat,
    /// Java primitive double value
    pub d: JDouble,
    /// Java Object value
    pub l: *mut JObject
}

/// Possible JVM reference types
#[repr(C)]
pub enum JObjectRefType {
    /// Reference is no longer valid
    JNIInvalidRefType = 0,
    /// Reference is valid for the life of the current scope
    JNILocalRefType = 1,
    /// Reference is valid forever
    JNIGlobalRefType = 2,
    /// Reference is valid as long as non-weak references exist
    JNIWeakGlobalRefType = 3
}

/// Data for registering a native method
#[repr(C)]
pub struct JNINativeMethod {
    name: *const i8,
    signature: *const i8,
    ptr: *mut c_void
}

impl JNINativeMethod {

    /// Create new JNINativeMethod struct out of parts
    pub fn new(name: *const i8, signature: *const i8, ptr: *mut c_void) -> JNINativeMethod {
        JNINativeMethod {
            name,
            signature,
            ptr
        }
    }

}

/// Data for attaching a thread to the JVM
#[repr(C)]
pub struct JavaVMAttachArgs {
    version: JInt,
    name: *mut i8,
    group: *mut JObject
}

impl JavaVMAttachArgs {

    /// Create new JavaVMAttachArgs from a JNI version
    pub fn new(version: JInt) -> JavaVMAttachArgs {
        JavaVMAttachArgs {
            version,
            name: std::ptr::null_mut(),
            group: std::ptr::null_mut()
        }
    }

}

/// Data for JVM startup options
#[repr(C)]
pub struct JavaVMOption {
    option_string: *mut i8,
    extra_info: *mut c_void
}

impl Debug for JavaVMOption {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let option_string = unsafe {
            CString::from_raw(self.option_string)
        };

        write!(
            f,
            "JavaVMOption {{ option_string: {:?}, extra_info: {:?} }}",
            option_string,
            self.extra_info
        )
    }
}

/// Data for JVM initialization arguments
#[repr(C)]
pub struct JavaVMInitArgs {
    version: JInt,
    num_options: JInt,
    options: *mut JavaVMOption,
    ignore_unrecognized: JBoolean
}

impl JavaVMInitArgs {

    /// Create new JavaVMInitArgs from a JNI version
    pub fn new(version: JInt) -> JavaVMInitArgs {
        JavaVMInitArgs {
            version,
            num_options: 0,
            options: std::ptr::null_mut(),
            ignore_unrecognized: false
        }
    }

    /// Add a startup option to these initialization args
    pub fn add_option(&mut self, option: JavaVMOption) {
        let layout = Layout::new::<JavaVMOption>();

        self.num_options += 1;
        if self.options == std::ptr::null_mut() {
            // SAFETY: Full size of allocation will be initialized by the set later
            unsafe {
                self.options = std::alloc::alloc(layout).cast();
            }
        } else {
            // SAFETY: Full size of allocation is initialized, or will be by the set later
            unsafe {
                self.options = std::alloc::realloc(
                    self.options.cast(),
                    layout,
                    std::mem::size_of::<JavaVMOption>() * self.num_options as usize
                ).cast()
            }
        }

        // SAFETY: Initializes any possibly uninit memory. Offset will always be less than array size
        unsafe {
            *self.options.offset(self.num_options as isize - 1) = option;
        }
    }

    /// Remove a startup option from these initialization args by index
    pub fn remove_option(&mut self, idx: i32) -> Result<(), Error>{
        if idx >= self.num_options || idx < 0 {
            return Err(Error::new(
                &format!("Index {} out of range for option removal", idx),
                constants::JNI_ERR
            ));
        }

        let layout = Layout::new::<JavaVMOption>();

        self.num_options -= 1;
        if self.num_options == 0 {
            // SAFETY: If num_options is zero, this array will never be accessed
            unsafe {
                std::alloc::dealloc(self.options.cast(), layout);
            }
        } else {
            // SAFETY: This will never overflow the end of the array, so will always be copying
            //         initialized values
            unsafe {
                self.options
                    .offset((idx + 1) as isize)
                    .copy_to(self.options.offset(idx as isize), (self.num_options - idx) as usize);
            }

            // SAFETY: Shrinks past the now discarded end of the array.
            unsafe {
                self.options = std::alloc::realloc(
                    self.options.cast(),
                    layout,
                    std::mem::size_of::<JavaVMOption>() * self.num_options as usize
                ).cast();
            }
        }

        Ok(())
    }
}

impl Debug for JavaVMInitArgs {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let options;
        unsafe {
            options = slice::from_raw_parts(self.options, self.num_options as usize);
        }
        write!(
            f,
            "JavaVMInitArgs {{ version: {:?}, num_options: {:?}, options: {:?}, ignore_unrecognized: {:?} }}",
            self.version,
            self.num_options,
            options,
            self.ignore_unrecognized
        )
    }
}

// Marker Trait implementations

/// Marker for all types that can be transmuted safely into a JObject without checks
pub unsafe trait IsObject {}

unsafe impl IsObject for JObject {}
unsafe impl IsObject for JWeak {}
unsafe impl IsObject for JClass {}
unsafe impl IsObject for JThrowable {}
unsafe impl IsObject for JString {}
unsafe impl IsObject for JArray {}
unsafe impl IsObject for JBooleanArray {}
unsafe impl IsObject for JByteArray {}
unsafe impl IsObject for JCharArray {}
unsafe impl IsObject for JShortArray {}
unsafe impl IsObject for JIntArray {}
unsafe impl IsObject for JLongArray {}
unsafe impl IsObject for JFloatArray {}
unsafe impl IsObject for JDoubleArray {}
unsafe impl IsObject for JObjectArray {}

/// Marker for all types that can be transmuted safely into a JArray without checks
pub unsafe trait IsArray {}

unsafe impl IsArray for JArray {}
unsafe impl IsArray for JBooleanArray {}
unsafe impl IsArray for JByteArray {}
unsafe impl IsArray for JCharArray {}
unsafe impl IsArray for JShortArray {}
unsafe impl IsArray for JIntArray {}
unsafe impl IsArray for JLongArray {}
unsafe impl IsArray for JFloatArray {}
unsafe impl IsArray for JDoubleArray {}
unsafe impl IsArray for JObjectArray {}
