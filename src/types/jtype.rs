//!
//! Module containing enums used to represent possible java types in the JNI, or types of a
//! reference
//!

use crate::ffi;

///
/// A struct containing all the possible types recognized by the java JNI
///
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum JType {
    /// Object type
    Object,
    /// Primitive boolean type
    Boolean,
    /// Primitive byte type
    Byte,
    /// Primitive char type
    Char,
    /// Primitive short type
    Short,
    /// Primitive int type
    Int,
    /// Primitive long type
    Long,
    /// Primitive float type
    Float,
    /// Primitive double type
    Double,
    /// Primitive void type
    Void
}

impl JType {

    /// Get a JType from the name of a type
    pub fn from_name(name: &str) -> JType {
        match name {
            "void" => {
                JType::Void
            }
            "boolean" => {
                JType::Boolean
            }
            "byte" => {
                JType::Byte
            }
            "char" => {
                JType::Char
            }
            "short" => {
                JType::Short
            }
            "int" => {
                JType::Int
            }
            "long" => {
                JType::Long
            }
            "float" => {
                JType::Float
            }
            "double" => {
                JType::Double
            }
            _ => {
                JType::Object
            }
        }
    }

    /// Get a JNonVoidType from this JType, if this JType isn't Void
    pub fn as_nonvoid(&self) -> Option<JNonVoidType> {
        match self {
            JType::Object => {
                Some(JNonVoidType::Object)
            }
            JType::Boolean => {
                Some(JNonVoidType::Boolean)
            }
            JType::Byte => {
                Some(JNonVoidType::Byte)
            }
            JType::Char => {
                Some(JNonVoidType::Char)
            }
            JType::Short => {
                Some(JNonVoidType::Short)
            }
            JType::Int => {
                Some(JNonVoidType::Int)
            }
            JType::Long => {
                Some(JNonVoidType::Long)
            }
            JType::Float => {
                Some(JNonVoidType::Float)
            }
            JType::Double => {
                Some(JNonVoidType::Double)
            }
            JType::Void => {
                None
            }
        }
    }

    /// Get a JNativeType from this JType, if this JType is a native/primitive type
    pub fn as_native(&self) -> Option<JNativeType> {
        match self {
            JType::Object => {
                None
            }
            JType::Boolean => {
                Some(JNativeType::Boolean)
            }
            JType::Byte => {
                Some(JNativeType::Byte)
            }
            JType::Char => {
                Some(JNativeType::Char)
            }
            JType::Short => {
                Some(JNativeType::Short)
            }
            JType::Int => {
                Some(JNativeType::Int)
            }
            JType::Long => {
                Some(JNativeType::Long)
            }
            JType::Float => {
                Some(JNativeType::Float)
            }
            JType::Double => {
                Some(JNativeType::Double)
            }
            JType::Void => {
                None
            }
        }
    }
}

///
/// A struct representing all the possible non-void types recognized by the java JNI
///
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum JNonVoidType {
    /// Object type
    Object,
    /// Primitive boolean type
    Boolean,
    /// Primitive byte type
    Byte,
    /// Primitive char type
    Char,
    /// Primitive short type
    Short,
    /// Primitive int type
    Int,
    /// Primitive long type
    Long,
    /// Primitive float type
    Float,
    /// Primitive double type
    Double
}

///
/// A struct representing all the possible native/primitive types recognized by the java JNI
///
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum JNativeType {
    /// Primitive boolean type
    Boolean,
    /// Primitive byte type
    Byte,
    /// Primitive char type
    Char,
    /// Primitive short type
    Short,
    /// Primitive int type
    Int,
    /// Primitive long type
    Long,
    /// Primitive float type
    Float,
    /// Primitive double type
    Double
}

///
/// A struct representing all the possible JVM reference types
///
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum JRefType {
    /// An invalid reference
    Invalid,
    /// A reference that lives till the end of the current scope
    Local,
    /// A reference that lives forever
    Global,
    /// A reference that lives as long as other references exist
    WeakGlobal
}

impl JRefType {

    /// Convert this JRefType to the ffi-safe equivalent enum
    fn as_ffi(&self) -> ffi::JObjectRefType {
        match self {
            JRefType::Invalid => {
                ffi::JObjectRefType::JNIInvalidRefType
            }
            JRefType::Local => {
                ffi::JObjectRefType::JNILocalRefType
            }
            JRefType::Global => {
                ffi::JObjectRefType::JNIGlobalRefType
            }
            JRefType::WeakGlobal => {
                ffi::JObjectRefType::JNIWeakGlobalRefType
            }
        }
    }
    
}

impl From<ffi::JObjectRefType> for JRefType {
    fn from(val: ffi::JObjectRefType) -> Self {
        match val {
            ffi::JObjectRefType::JNIInvalidRefType => {
                JRefType::Invalid
            }
            ffi::JObjectRefType::JNILocalRefType => {
                JRefType::Local
            }
            ffi::JObjectRefType::JNIGlobalRefType => {
                JRefType::Global
            }
            ffi::JObjectRefType::JNIWeakGlobalRefType => {
                JRefType::WeakGlobal
            }
        }
    }
}
