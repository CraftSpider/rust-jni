
use crate::ffi;
use crate::types::JObject;
use crate::error::Error;

#[derive(Debug)]
pub enum JValue<'a> {
    Bool(bool),
    Byte(i8),
    Char(char),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    Object(Option<JObject<'a>>)  // Option because null exists, and must be handled
}

impl<'a> JValue<'a> {

    pub fn make_ffi_vec(slice: &[JValue]) -> Vec<ffi::JValue> {
        let mut out = Vec::new();

        for i in 0..slice.len() {
            unsafe {
                out.push(slice[i].as_ffi())
            }
        }

        out
    }

    pub fn into_obj(self) -> Result<Option<JObject<'a>>, Error> {
        if let JValue::Object(obj) = self {
            Ok(obj)
        } else {
            Err(Error::new("JValue isn't an object", ffi::constants::JNI_ERR))
        }
    }

    pub fn into_bool(self) -> Result<bool, Error> {
        if let JValue::Bool(b) = self {
            Ok(b)
        } else {
            Err(Error::new("JValue isn't a boolean", ffi::constants::JNI_ERR))
        }
    }

    pub fn into_byte(self) -> Result<i8, Error> {
        if let JValue::Byte(b) = self {
            Ok(b)
        } else {
            Err(Error::new("JValue isn't a byte", ffi::constants::JNI_ERR))
        }
    }

    pub fn into_char(self) -> Result<char, Error> {
        if let JValue::Char(c) = self {
            Ok(c)
        } else {
            Err(Error::new("JValue isn't a char", ffi::constants::JNI_ERR))
        }
    }

    pub fn into_short(self) -> Result<i16, Error> {
        if let JValue::Short(s) = self {
            Ok(s)
        } else {
            Err(Error::new("JValue isn't a short", ffi::constants::JNI_ERR))
        }
    }

    pub fn into_int(self) -> Result<i32, Error> {
        if let JValue::Int(i) = self {
            Ok(i)
        } else {
            Err(Error::new("JValue isn't an integer", ffi::constants::JNI_ERR))
        }
    }

    pub fn into_long(self) -> Result<i64, Error> {
        if let JValue::Long(l) = self {
            Ok(l)
        } else {
            Err(Error::new("JValue isn't a long", ffi::constants::JNI_ERR))
        }
    }

    pub fn into_float(self) -> Result<f32, Error> {
        if let JValue::Float(f) = self {
            Ok(f)
        } else {
            Err(Error::new("JValue isn't a float", ffi::constants::JNI_ERR))
        }
    }

    pub fn into_double(self) -> Result<f64, Error> {
        if let JValue::Double(d) = self {
            Ok(d)
        } else {
            Err(Error::new("JValue isn't a double", ffi::constants::JNI_ERR))
        }
    }

    pub unsafe fn as_ffi(&self) -> ffi::JValue {
        match self {
            JValue::Bool(bool) => {
                ffi::JValue { z: *bool as ffi::JBoolean }
            }
            JValue::Byte(byte) => {
                ffi::JValue { b: *byte as ffi::JByte }
            }
            JValue::Char(char) => {
                ffi::JValue { c: *char as ffi::JChar }
            }
            JValue::Short(short) => {
                ffi::JValue { s: *short as ffi::JShort }
            }
            JValue::Int(int) => {
                ffi::JValue { i: *int as ffi::JInt }
            }
            JValue::Long(long) => {
                ffi::JValue { j: *long as ffi::JLong }
            }
            JValue::Float(float) => {
                ffi::JValue { f: *float as ffi::JFloat }
            }
            JValue::Double(double) => {
                ffi::JValue { d: *double as ffi::JDouble }
            }
            JValue::Object(obj) => {
                // SAFETY: Internal pointer use
                ffi::JValue { l: obj.as_ref().map(|obj| {obj.borrow_ptr()}).unwrap_or(std::ptr::null_mut()) }
            }
        }
    }
}

impl From<bool> for JValue<'_> {
    fn from(val: bool) -> Self {
        return JValue::Bool(val)
    }
}

impl From<i8> for JValue<'_> {
    fn from(val: i8) -> Self {
        return JValue::Byte(val)
    }
}

impl From<char> for JValue<'_> {
    fn from(val: char) -> Self {
        return JValue::Char(val)
    }
}

impl From<i16> for JValue<'_> {
    fn from(val: i16) -> Self {
        return JValue::Short(val)
    }
}

impl From<i32> for JValue<'_> {
    fn from(val: i32) -> Self {
        return JValue::Int(val)
    }
}

impl From<i64> for JValue<'_> {
    fn from(val: i64) -> Self {
        return JValue::Long(val)
    }
}

impl From<f32> for JValue<'_> {
    fn from(val: f32) -> Self {
        return JValue::Float(val)
    }
}

impl From<f64> for JValue<'_> {
    fn from(val: f64) -> Self {
        return JValue::Double(val)
    }
}

impl<'a> From<JObject<'a>> for JValue<'a> {
    fn from(val: JObject<'a>) -> Self {
        return JValue::Object(Some(val))
    }
}

impl<'a> From<Option<JObject<'a>>> for JValue<'a> {
    fn from(val: Option<JObject<'a>>) -> Self {
        return JValue::Object(val)
    }
}
