use crate::ffi;

#[derive(Debug, Copy, Clone)]
pub enum JType {
    Object,
    Boolean,
    Byte,
    Char,
    Short,
    Int,
    Long,
    Float,
    Double,
    Void
}

impl JType {
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

#[derive(Debug, Copy, Clone)]
pub enum JNonVoidType {
    Object,
    Boolean,
    Byte,
    Char,
    Short,
    Int,
    Long,
    Float,
    Double
}

#[derive(Debug, Copy, Clone)]
pub enum JNativeType {
    Boolean,
    Byte,
    Char,
    Short,
    Int,
    Long,
    Float,
    Double
}

#[derive(Debug, Copy, Clone)]
pub enum JRefType {
    Invalid,
    Local,
    Global,
    WeakGlobal
}

impl JRefType {
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
