//!
//! A module containing macro helpers, for more efficient usage of the JNI in rust code. Handles
//! some of the possible pitfalls for you as much as possible.
//!
//! - Global references are valid on any attached thread
//! - Returns a &JObject instead of a JObject, as deleting the global reference would lead to UB
//!


#[macro_export]
macro_rules! get_cls {
    ($env:ident, $cls:literal) => {
        {
            use $crate::types::*;

            static mut CLS: Option<JClass<'static>> = None;
            unsafe {
                if let None = CLS {
                    let cls = $env.new_global_ref(&$env.find_class($cls).expect(&format!("Couldn't find class {}", $cls)).downcast()).unwrap().upcast_raw();
                    CLS = Some(cls);
                }
                CLS.as_ref().unwrap()
            }
        }
    }
}


#[macro_export]
macro_rules! get_method_id {
    ($env:ident, $cls:ident, $name:literal, $sig:literal) => {
        {
            use $crate::types::*;

            static mut ID: Option<JMethodID> = None;
            unsafe {
                if let None = ID {
                    let id = $env.get_method_id($cls, $name, $sig).expect(&format!("Couldn't find method {} with signature {}", $name, $sig));
                    ID = Some(id)
                }
                ID.as_ref().unwrap()
            }
        }
    }
}


#[macro_export]
macro_rules! get_static_method_id {
    ($env:ident, $cls:ident, $name:literal, $sig:literal) => {
        {
            use $crate::types::*;

            static mut ID: Option<JMethodID> = None;
            unsafe {
                if let None = ID {
                    let id = $env.get_static_method_id($cls, $name, $sig).expect(&format!("Couldn't find method {} with signature {}", $name, $sig));
                    ID = Some(id)
                }
                ID.as_ref().unwrap()
            }
        }
    }
}


#[macro_export]
macro_rules! get_field_id {
    ($env:ident, $cls:ident, $name:literal, $ty:literal) => {
        {
            use $crate::types::*;

            static mut ID: Option<JFieldID> = None;
            unsafe {
                if let None = ID {
                    let id = $env.get_field_id($cls, $name, $ty).expect(&format!("Couldn't find method {} with signature {}", $name, $ty));
                    ID = Some(id)
                }
                ID.as_ref().unwrap()
            }
        }
    }
}


#[macro_export]
macro_rules! get_static_field_id {
    ($env:ident, $cls:ident, $name:literal, $ty:literal) => {
        {
            use $crate::types::*;

            static mut ID: Option<JFieldID> = None;
            unsafe {
                if let None = ID {
                    let id = $env.get_static_field_id($cls, $name, $ty).expect(&format!("Couldn't find method {} with signature {}", $name, $ty));
                    ID = Some(id)
                }
                ID.as_ref().unwrap()
            }
        }
    }
}
