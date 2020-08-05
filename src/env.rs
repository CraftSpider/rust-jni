
use std::ffi::CString;
use std::slice;

use crate::{ffi, JByte, JNativeType, JNativeArray, JNativeSlice, ReleaseMode, JNativeVec};
use crate::types::{JNIVersion, JType, JValue, JObject, JClass, JMethodID, JFieldID, JThrowable, JString, JArray, JObjectArray, JavaDownCast, JNonVoidType, JNINativeMethod, JavaUpCast};
use crate::error::{Error, Result};
use crate::mangling::{mangle_class, TypeSignature};
use crate::vm::JavaVM;
use crate::types::jtype::JRefType;
use std::collections::HashMap;
use std::cell::RefCell;
use crate::types::object::JWeak;
use crate::object::JByteBuffer;


/// Handy utility for converting a `&str` into a `CString`, returning a rust_jni error on failure
fn cstr_from_str(str: &str) -> Result<CString> {
    CString::new(str)
        .map_err(|err| {
            Error::from(Box::new(err))
        })
}


/// Higher-level construct representing a JNIEnv
pub struct JNIEnv {
    version: JNIVersion,
    backing_ptr: *mut ffi::JNIEnv
}

impl JNIEnv {

    /// Create a new JNIEnv from a pointer to an [ffi::JNIEnv]. This environment will
    /// live as long as the current thread, generally. Thus this type is not marked Send or Sync.
    pub fn new(env: *mut ffi::JNIEnv) -> Result<JNIEnv> {
        if env.is_null() {
            Err(Error::new("JNIEnv must be constructed from non-null pointer", ffi::constants::JNI_ERR))
        } else {
            // SAFETY: Pointer is definitely not null here
            let version;
            unsafe {
                version = <*mut ffi::JNIEnv>::as_ref(env)
                    .expect("Couldn't get ref to checked pionter")
                    .get_version()
                    .into();
            }
            Ok(JNIEnv {
                version,
                backing_ptr: env
            })
        }
    }

    /// Get the backing environment pointer
    pub unsafe fn borrow_ptr(&self) -> *mut ffi::JNIEnv {
        self.backing_ptr
    }

    /// Non public way to get a reference to the internal environment. Not unsafe only because
    /// it's not public.
    fn internal_env(&self) -> &ffi::JNIEnv {
        // SAFETY: The real_env pointer is private, and only set to non-null values in checked locations
        unsafe {
            if let Some(env) = self.backing_ptr.as_ref() {
                env
            } else {
                panic!("Invalid JNIEnv")
            }
        }
    }

    /// Get the version of the associated JVM
    pub fn get_version(&self) -> JNIVersion {
        let env = self.internal_env();
        JNIVersion::from(env.get_version())
    }

    /// Define a new JVM class. The class will have the given name and be owned by the given loader,
    /// created from the passed byte buffer.
    pub fn define_class(&self, name: &str, loader: &JObject, buffer: &[u8]) -> Result<JClass> {
        let env = self.internal_env();
        let name = cstr_from_str(name)?;

        // SAFETY: Internal pointer use
        unsafe {
            let new_cls = env.define_class(name.as_ptr(), loader.borrow_ptr(), buffer.as_ptr() as _, buffer.len() as i32);
            if new_cls.is_null() {
                Err(Error::new("Could not define new Java Class", ffi::constants::JNI_ERR))
            } else {
                Ok(JClass::new(new_cls)?)
            }
        }
    }

    /// Find an existing class by name. The passed name should consist only of ASCII characters
    pub fn find_class(&self, name: &str) -> Result<JClass> {
        let env = self.internal_env();
        let c_name = cstr_from_str(&mangle_class(name).mangled())?;

        let new_cls = env.find_class(c_name.as_ptr());
        if new_cls.is_null() {
            Err(Error::new(&format!("Could not find Java Class {}", name), ffi::constants::JNI_ERR))
        } else {
            Ok(JClass::new(new_cls)?)
        }
    }

    /// Convert a reflected method object into an associated method ID
    pub fn from_reflected_method(&self, method: &JObject) -> Result<JMethodID> {
        let env = self.internal_env();
        let meth_cls = self.find_class("java.lang.reflect.Method").unwrap();
        let cls_cls = self.find_class("java.lang.Class").unwrap();
        let get_ret = self.get_method_id(&meth_cls, "getReturnType", "() -> java.lang.Class").unwrap();
        let get_num_args = self.get_method_id(&meth_cls, "getParameterCount", "() -> int").unwrap();
        let get_name = self.get_method_id(&cls_cls, "getName", "() -> java.lang.String").unwrap();

        // SAFETY: Internal pointer use
        unsafe {
            let id = env.from_reflected_method(method.borrow_ptr());
            let ret_cls = self.call_method(method, &get_ret, &vec![])?
                .expect("Unexpected void result")
                .into_obj()?
                .expect("Unexpected null result");
            let ret_name = self.call_method(&ret_cls, &get_name, &vec![])?
                .expect("Unexpected void result")
                .into_obj()?
                .expect("Unexpected null result");
            let num_args = self.call_method(method, &get_num_args, &vec![])?
                .expect("Unexpected void result")
                .into_int()? as usize;

            let chars = self.get_string_chars(&ret_name.upcast_raw());
            let chars: String = chars.into_iter().collect();
            let ret_type = JType::from_name(&chars);

            if id.is_null() {
                Err(Error::new("Could not find method ID", ffi::constants::JNI_ERR))
            } else {
                Ok(JMethodID::new(id, ret_type, num_args )?)
            }
        }
    }

    /// Convert a reflected field object into an associated field ID
    pub fn from_reflected_field(&self, field: &JObject) -> Result<JFieldID> {
        let env = self.internal_env();
        let field_cls = self.find_class("java.lang.reflect.Field").unwrap();
        let cls_cls = self.find_class("java.lang.Class").unwrap();
        let get_ty = self.get_method_id(&field_cls, "getType", "() -> java.lang.Class").unwrap();
        let get_name = self.get_method_id(&cls_cls, "getName", "() -> java.lang.String").unwrap();

        // SAFETY: Internal pointer use
        unsafe {
            let id = env.from_reflected_field(field.borrow_ptr());
            let ty_cls = self.call_method(field, &get_ty, &vec![])?
                .expect("Unexpected void result")
                .into_obj()?
                .expect("Unexpected null result");
            let ty_name = self.call_method(&ty_cls, &get_name, &vec![])?
                .expect("Unexpected void result")
                .into_obj()?
                .expect("Unexpected null result");

            let chars = self.get_string_chars(&ty_name.upcast_raw());
            let chars: String = chars.into_iter().collect();
            let ty = JType::from_name(&chars).as_nonvoid().unwrap();

            if id.is_null() {
                Err(Error::new("Could not find field ID", ffi::constants::JNI_ERR))
            } else {
                Ok(JFieldID::new(id, ty)?)
            }
        }
    }

    /// TODO: Maybe make is_static part of IDs?
    /// Build a reflected Method object from a class, method ID, and static-ness
    pub fn to_reflected_method(&self, cls: &JClass, id: &JMethodID, is_static: bool) -> Result<JObject> {
        let env = self.internal_env();

        // SAFETY: Internal pointer use
        unsafe {
            let obj = env.to_reflected_method(cls.borrow_ptr(), id.borrow_ptr(), is_static.into());
            if obj.is_null() {
                Err(Error::new("Could not find reflected method", ffi::constants::JNI_ERR))
            } else {
                Ok(JObject::new(obj)?)
            }
        }
    }

    /// Build a reflected Field object from a class, field ID, and static-ness
    pub fn to_reflected_field(&self, cls: &JClass, id: &JFieldID, is_static: bool) -> Result<JObject> {
        let env = self.internal_env();

        unsafe {
            let obj = env.to_reflected_field(cls.borrow_ptr(), id.borrow_ptr(), is_static.into());
            if obj.is_null() {
                Err(Error::new("Could not find reflected field", ffi::constants::JNI_ERR))
            } else {
                Ok(JObject::new(obj)?)
            }
        }
    }

    /// Get the superclass of a given class. Will return an error if the class is Object or other
    /// class with no superclass.
    pub fn get_superclass(&self, cls: &JClass) -> Result<JClass> {
        let env = self.internal_env();

        // SAFETY: Internal pointer use
        unsafe {
            let obj = env.get_superclass(cls.borrow_ptr());
            if obj.is_null() {
                Err(Error::new("Could not get object superclass", ffi::constants::JNI_ERR))
            } else {
                Ok(JClass::new(obj)?)
            }
        }
    }

    /// Checks whether an object with the type of the first argument can be safely cast to an object
    /// with the type of the second object
    pub fn is_assignable_from(&self, cls1: &JClass, cls2: &JClass) -> bool {
        let env = self.internal_env();

        // SAFETY: Internal pointer use
        unsafe {
            env.is_assignable_from(cls1.borrow_ptr(), cls2.borrow_ptr()) != 0
        }
    }

    /// Start throwing an exception on the JVM. Result is Ok if exception *is* thrown, Err if no
    /// exception is thrown.
    pub fn throw(&self, exception: JThrowable) -> Result<()> {
        let env = self.internal_env();

        // SAFETY: Internal pointer use
        unsafe {
            let result = env.throw(exception.borrow_ptr());
            if result != 0 {
                Err(Error::new("Could not throw exception", ffi::constants::JNI_ERR))
            } else {
                Ok(())
            }
        }
    }

    /// Start throwing a new instance of an exception on the JVM. Result is Ok if exception *is*
    /// thrown, Err if no exception is thrown.
    pub fn throw_new(&self, cls: JClass, msg: &str) -> Result<()> {
        let env = self.internal_env();
        let c_msg = cstr_from_str(msg)?;

        // SAFETY: Internal pointer use
        unsafe {
            let result = env.throw_new(cls.borrow_ptr(), c_msg.as_ptr());
            if result != 0 {
                Err(Error::new("Could not throw exception", ffi::constants::JNI_ERR))
            } else {
                Ok(())
            }
        }
    }

    /// Check whether an exception is currently occuring on the JVM
    pub fn exception_check(&self) -> bool {
        let env = self.internal_env();
        env.exception_check() != 0
    }

    /// Get the current exception being thrown, or Err
    pub fn exception_occurred(&self) -> Result<JThrowable> {
        let env = self.internal_env();

        let exc = env.exception_occurred();
        if exc.is_null() {
            Err(Error::new("No active exception to retrieve", ffi::constants::JNI_ERR))
        } else {
            Ok(JThrowable::new(exc)?)
        }
    }

    /// Helper to print the current exception being thrown, or Err
    pub fn exception_describe(&self) -> Result<()> {
        let env = self.internal_env();

        if self.exception_check() {
            env.exception_describe();
            Ok(())
        } else {
            Err(Error::new("No active exception to describe", ffi::constants::JNI_ERR))
        }
    }

    /// If an exception is being thrown, clear it. Otherwise Err
    pub fn exception_clear(&self) -> Result<()> {
        let env = self.internal_env();

        if self.exception_check() {
            env.exception_clear();
            Ok(())
        } else {
            Err(Error::new("No active error to clear", ffi::constants::JNI_ERR))
        }
    }

    /// Raise a fatal error, and don't expect the JVM to continue.
    pub fn fatal_error(&self, msg: &str) -> ! {
        let env = self.internal_env();
        let c_msg = cstr_from_str(msg)?;

        env.fatal_error(c_msg.as_ptr())
    }

    pub fn ensure_local_capacity(&self, capacity: i32) -> Result<()> {
        let env = self.internal_env();

        let result = env.ensure_local_capacity(capacity);
        if result != 0 {
            Err(Error::new(&format!("Couldn't ensure local capacity of at least {}", capacity), result))
        } else {
            Ok(())
        }
    }

    // TODO: Maybe make this return a new 'environment' so all refs don't outlive it?
    pub fn push_local_frame(&self, capacity: i32) -> Result<()> {
        let env = self.internal_env();

        let result = env.push_local_frame(capacity);
        if result != 0 {
            Err(Error::new(&format!("Couldn't push local from with capacity {}", capacity), result))
        } else {
            Ok(())
        }
    }

    pub fn pop_local_frame<'a>(&self, obj: Option<JObject<'a>>) -> Option<JObject<'a>> {
        let env = self.internal_env();

        // SAFETY: Internal pointer use
        unsafe {
            let ptr;
            if let Some(obj) = obj {
                ptr = obj.borrow_ptr();
            } else {
                ptr = std::ptr::null_mut();
            }

            let out = env.pop_local_frame(ptr);

            if out.is_null() {
                None
            } else {
                Some(JObject::new(out).expect("Null pointer in `pop_local_frame` despite null check"))
                // Some(self.local_ref(out))
            }
        }
    }

    pub fn new_global_ref(&self, obj: &JObject) -> Result<JObject<'static>> {
        let env = self.internal_env();

        // SAFETY: Internal pointer use
        unsafe {
            let obj = env.new_global_ref(obj.borrow_ptr());
            if obj.is_null() {
                Err(Error::new("Couldn't create new globabl reference", ffi::constants::JNI_ERR))
            } else {
                Ok(JObject::new(obj)?)
            }
        }
    }

    pub fn delete_global_ref(&self, obj: JObject<'static>) {
        let env = self.internal_env();

        // SAFETY: Internal pointer use
        unsafe {
            env.delete_global_ref(obj.borrow_ptr())
        }
    }

    pub fn new_local_ref(&self, obj: &JObject) -> Result<JObject> {
        let env = self.internal_env();

        // SAFETY: Internal pointer use
        unsafe {
            let obj = env.new_local_ref(obj.borrow_ptr());
            if obj.is_null() {
                Err(Error::new("Couldn't create new local reference", ffi::constants::JNI_ERR))
            } else {
                Ok(JObject::new(obj)?)
            }
        }
    }

    /// Deletes a local reference on the JVM. If you use this, you must ensure that all
    /// other references to the passed JObject are not used. Any use of them past here
    /// will cause undefined behavior.
    pub fn delete_local_ref(&self, obj: JObject) {
        let env = self.internal_env();

        // SAFETY: Internal pointer use
        unsafe {
            env.delete_local_ref(obj.borrow_ptr());
        }
    }

    pub fn is_same_object(&self, obj1: &JObject, obj2: &JObject) -> bool {
        let env = self.internal_env();

        // SAFETY: Internal pointer use
        unsafe {
            env.is_same_object(obj1.borrow_ptr(), obj2.borrow_ptr()) != 0
        }
    }

    pub fn alloc_object(&self, cls: &JClass) -> Result<JObject> {
        let env = self.internal_env();

        // SAFETY: Internal pointer use
        unsafe {
            let obj = env.alloc_object(cls.borrow_ptr());
            if obj.is_null() {
                Err(Error::new("Couldn't allocate object", ffi::constants::JNI_ERR))
            } else {
                Ok(JObject::new(obj)?)
            }
        }
    }

    pub fn new_object(&self, cls: &JClass, id: &JMethodID, args: &[JValue]) -> Result<JObject> {
        let env = self.internal_env();

        // SAFETY: Internal pointer use
        unsafe {
            let c_args = JValue::make_ffi_vec(args);
            let obj = env.new_object(cls.borrow_ptr(), id.borrow_ptr(), c_args.as_ptr());
            if obj.is_null() {
                Err(Error::new("Couldn't create new object", ffi::constants::JNI_ERR))
            } else {
                Ok(JObject::new(obj)?)
            }
        }
    }

    pub fn get_object_class(&self, obj: &JObject) -> Result<JClass> {
        let env = self.internal_env();

        // SAFETY: Internal pointer use
        unsafe {
            let cls = env.get_object_class(obj.borrow_ptr());
            if cls.is_null() {
                Err(Error::new("Couldn't get object class", ffi::constants::JNI_ERR))
            } else {
                Ok(JClass::new(cls)?)
            }
        }
    }

    pub fn is_instance_of(&self, obj: &JObject, cls: &JClass) -> bool {
        let env = self.internal_env();

        // SAFETY: Internal pointer use
        unsafe {
            env.is_instance_of(obj.borrow_ptr(), cls.borrow_ptr()) != 0
        }
    }

    pub fn get_method_id(&self, cls: &JClass, name: &str, sig: &str) -> Result<JMethodID> {
        let env = self.internal_env();
        let c_name = cstr_from_str(name)?;

        let sig = mangle_class(sig);
        let num_args;
        let ret_ty;

        if let TypeSignature::Method(args, ret) = &sig {
            num_args = args.len();
            ret_ty = ret.java_type();
        } else {
            return Err(Error::new("Expected method signature", ffi::constants::JNI_ERR));
        }

        let c_sig = cstr_from_str(&sig.mangled())?;

        // SAFETY: Internal pointer use
        unsafe {
            let id = env.get_method_id(cls.borrow_ptr(), c_name.as_ptr(), c_sig.as_ptr());
            if id.is_null() {
                Err(Error::new(&format!("Couldn't get method id of {}", name), ffi::constants::JNI_ERR))
            } else {
                Ok(JMethodID::new(id, ret_ty, num_args)?)
            }
        }
    }

    pub fn call_method(&self, obj: &JObject, id: &JMethodID, args: &[JValue]) -> Result<Option<JValue>> {
        if args.len() != id.num_args() {
            return Err(Error::new("Invalid number of arguement for method", ffi::constants::JNI_ERR))
        }

        let env = self.internal_env();
        let args = JValue::make_ffi_vec(args);

        // SAFETY: Internal pointer use
        let raw_obj;
        let raw_id;
        unsafe {
            raw_obj = obj.borrow_ptr();
            raw_id = id.borrow_ptr();
        }

        match id.ret_ty() { // TODO: Add error check after calls?
            JType::Object => {
                let result = env.call_object_method(raw_obj, raw_id, args.as_ptr());
                if result.is_null() {
                    Ok(Some(JValue::Object(None)))
                } else {
                    Ok(Some(JValue::Object(Some(JObject::new(result)?))))
                }
            }
            JType::Boolean => {
                let result = env.call_boolean_method(raw_obj, raw_id, args.as_ptr());
                Ok(Some(JValue::Bool(result != 0)))
            }
            JType::Byte => {
                let result = env.call_byte_method(raw_obj, raw_id, args.as_ptr());
                Ok(Some(JValue::Byte(result)))
            }
            JType::Char => {
                let result = env.call_char_method(raw_obj, raw_id, args.as_ptr());
                Ok(Some(JValue::Char(
                    std::char::from_u32(result as u32).expect("Java returned bad char")
                )))
            }
            JType::Short => {
                let result = env.call_short_method(raw_obj, raw_id, args.as_ptr());
                Ok(Some(JValue::Short(result)))
            }
            JType::Int => {
                let result = env.call_int_method(raw_obj, raw_id, args.as_ptr());
                Ok(Some(JValue::Int(result)))
            }
            JType::Long => {
                let result = env.call_long_method(raw_obj, raw_id, args.as_ptr());
                Ok(Some(JValue::Long(result)))
            }
            JType::Float => {
                let result = env.call_float_method(raw_obj, raw_id, args.as_ptr());
                Ok(Some(JValue::Float(result)))
            }
            JType::Double => {
                let result = env.call_double_method(raw_obj, raw_id, args.as_ptr());
                Ok(Some(JValue::Double(result)))
            }
            JType::Void => {
                env.call_void_method(raw_obj, raw_id, args.as_ptr());
                Ok(None)
            }
        }
    }

    pub fn call_nonvirtual_method(&self, obj: &JObject, cls: &JClass, id: &JMethodID, args: &[JValue]) -> Result<Option<JValue>> {
        if args.len() != id.num_args() {
            return Err(Error::new("Invalid number of arguement for method", ffi::constants::JNI_ERR))
        }

        let env = self.internal_env();
        let args = JValue::make_ffi_vec(args);

        // SAFETY: Internal pointer use
        let raw_obj;
        let raw_cls;
        let raw_id;
        unsafe {
            raw_obj = obj.borrow_ptr();
            raw_cls = cls.borrow_ptr();
            raw_id = id.borrow_ptr();
        }

        match id.ret_ty() { // TODO: Add error check for non-object calls?
            JType::Object => {
                let result = env.call_nonvirtual_object_method(raw_obj, raw_cls, raw_id, args.as_ptr());
                if result.is_null() {
                    Ok(Some(JValue::Object(None)))
                } else {
                    Ok(Some(JValue::Object(Some(JObject::new(result)?))))
                }
            }
            JType::Boolean => {
                let result = env.call_nonvirtual_boolean_method(raw_obj, raw_cls, raw_id, args.as_ptr());
                Ok(Some(JValue::Bool(result != 0)))
            }
            JType::Byte => {
                let result = env.call_nonvirtual_byte_method(raw_obj, raw_cls, raw_id, args.as_ptr());
                Ok(Some(JValue::Byte(result)))
            }
            JType::Char => {
                let result = env.call_nonvirtual_char_method(raw_obj, raw_cls, raw_id, args.as_ptr());
                Ok(Some(JValue::Char(
                    std::char::from_u32(result as u32).expect("Java returned bad char")
                )))
            }
            JType::Short => {
                let result = env.call_nonvirtual_short_method(raw_obj, raw_cls, raw_id, args.as_ptr());
                Ok(Some(JValue::Short(result)))
            }
            JType::Int => {
                let result = env.call_nonvirtual_int_method(raw_obj, raw_cls, raw_id, args.as_ptr());
                Ok(Some(JValue::Int(result)))
            }
            JType::Long => {
                let result = env.call_nonvirtual_long_method(raw_obj, raw_cls, raw_id, args.as_ptr());
                Ok(Some(JValue::Long(result)))
            }
            JType::Float => {
                let result = env.call_nonvirtual_float_method(raw_obj, raw_cls, raw_id, args.as_ptr());
                Ok(Some(JValue::Float(result)))
            }
            JType::Double => {
                let result = env.call_nonvirtual_double_method(raw_obj, raw_cls, raw_id, args.as_ptr());
                Ok(Some(JValue::Double(result)))
            }
            JType::Void => {
                env.call_nonvirtual_void_method(raw_obj, raw_cls, raw_id, args.as_ptr());
                Ok(None)
            }
        }
    }

    pub fn get_field_id(&self, cls: &JClass, name: &str, sig: &str) -> Result<JFieldID> {
        let env = self.internal_env();
        let c_name = cstr_from_str(name)?;

        let sig = mangle_class(sig);
        let ty= sig.java_type().as_nonvoid().expect("Expected field type to be non-void");

        let c_sig = cstr_from_str(&sig.mangled())?;

        // SAFETY: Internal pointer use
        unsafe {
            let id = env.get_field_id(cls.borrow_ptr(), c_name.as_ptr(), c_sig.as_ptr());
            if id.is_null() {
                Err(Error::new(&format!("Couldn't get field id of {}", name), ffi::constants::JNI_ERR))
            } else {
                Ok(JFieldID::new(id, ty)?)
            }
        }
    }

    pub fn get_field(&self, obj: &JObject, id: &JFieldID) -> Result<JValue> {
        let env = self.internal_env();

        let raw_obj;
        let raw_id;
        // SAFETY: Internal pointer use
        unsafe {
            raw_obj = obj.borrow_ptr();
            raw_id = id.borrow_ptr();
        }

        match id.ty() {
            JNonVoidType::Object => {
                let result = env.get_object_field(raw_obj, raw_id);
                if result.is_null() {
                    Ok(JValue::Object(None))
                } else {
                    Ok(JValue::Object(Some(JObject::new(result)?)))
                }
            }
            JNonVoidType::Boolean => {
                let result = env.get_boolean_field(raw_obj, raw_id);
                Ok(JValue::Bool(result != 0))
            }
            JNonVoidType::Byte => {
                let result = env.get_byte_field(raw_obj, raw_id);
                Ok(JValue::Byte(result))
            }
            JNonVoidType::Char => {
                let result = env.get_char_field(raw_obj, raw_id);
                Ok(JValue::Char(std::char::from_u32(result as u32).expect("Java returned bad char")))
            }
            JNonVoidType::Short => {
                let result = env.get_short_field(raw_obj, raw_id);
                Ok(JValue::Short(result))
            }
            JNonVoidType::Int => {
                let result = env.get_int_field(raw_obj, raw_id);
                Ok(JValue::Int(result))
            }
            JNonVoidType::Long => {
                let result = env.get_long_field(raw_obj, raw_id);
                Ok(JValue::Long(result))
            }
            JNonVoidType::Float => {
                let result = env.get_float_field(raw_obj, raw_id);
                Ok(JValue::Float(result))
            }
            JNonVoidType::Double => {
                let result = env.get_double_field(raw_obj, raw_id);
                Ok(JValue::Double(result))
            }
        }
    }

    pub fn set_field(&self, obj: &JObject, id: &JFieldID, val: JValue) -> Result<()> {
        let env = self.internal_env();

        let raw_obj;
        let raw_id;
        // SAFETY: Internal pointer use
        unsafe {
            raw_obj = obj.borrow_ptr();
            raw_id = id.borrow_ptr();
        }

        match id.ty() {
            JNonVoidType::Object => {
                // SAFETY: Internal pointer use
                unsafe {
                    let obj = val
                        .into_obj()?
                        .map(|obj| {obj.borrow_ptr()})
                        .unwrap_or(std::ptr::null_mut());

                    env.set_object_field(raw_obj, raw_id, obj);
                }
                Ok(())
            }
            JNonVoidType::Boolean => {
                env.set_boolean_field(raw_obj, raw_id, val.into_bool()? as ffi::JBoolean);
                Ok(())
            }
            JNonVoidType::Byte => {
                env.set_byte_field(raw_obj, raw_id, val.into_byte()? as ffi::JByte);
                Ok(())
            }
            JNonVoidType::Char => {
                env.set_char_field(raw_obj, raw_id, val.into_char()? as ffi::JChar);
                Ok(())
            }
            JNonVoidType::Short => {
                env.set_short_field(raw_obj, raw_id, val.into_short()? as ffi::JShort);
                Ok(())
            }
            JNonVoidType::Int => {
                env.set_int_field(raw_obj, raw_id, val.into_int()? as ffi::JInt);
                Ok(())
            }
            JNonVoidType::Long => {
                env.set_long_field(raw_obj, raw_id, val.into_long()? as ffi::JLong);
                Ok(())
            }
            JNonVoidType::Float => {
                env.set_float_field(raw_obj, raw_id, val.into_float()? as ffi::JFloat);
                Ok(())
            }
            JNonVoidType::Double => {
                env.set_double_field(raw_obj, raw_id, val.into_double()? as ffi::JDouble);
                Ok(())
            }
        }
    }

    pub fn get_static_method_id(&self, cls: &JClass, name: &str, sig: &str) -> Result<JMethodID> {
        let env = self.internal_env();
        let c_name = cstr_from_str(name)?;

        let sig = mangle_class(sig);
        let num_args;
        let ret_ty;

        if let TypeSignature::Method(args, ret) = &sig {
            num_args = args.len();
            ret_ty = ret.java_type();
        } else {
            return Err(Error::new("Expected method signature", ffi::constants::JNI_ERR));
        }

        let c_sig = cstr_from_str(&sig.mangled())?;

        // SAFETY: Internal pointer use
        unsafe {
            let id = env.get_static_method_id(cls.borrow_ptr(), c_name.as_ptr(), c_sig.as_ptr());
            if id.is_null() {
                Err(Error::new(&format!("Couldn't get static method id of {}", name), ffi::constants::JNI_ERR))
            } else {
                Ok(JMethodID::new(id, ret_ty, num_args)?)
            }
        }
    }

    pub fn call_static_method(&self, cls: &JClass, id: &JMethodID, args: &[JValue]) -> Result<Option<JValue>> {
        if args.len() != id.num_args() {
            return Err(Error::new("Invalid number of arguement for method", ffi::constants::JNI_ERR))
        }

        let env = self.internal_env();
        let args = JValue::make_ffi_vec(args);

        // SAFETY: Internal pointer use
        let raw_cls;
        let raw_id;
        unsafe {
            raw_cls = cls.borrow_ptr();
            raw_id = id.borrow_ptr();
        }

        match id.ret_ty() { // TODO: Add error check for calls?
            JType::Object => {
                let result = env.call_static_object_method(raw_cls, raw_id, args.as_ptr());
                if result.is_null() {
                    Ok(Some(JValue::Object(None)))
                } else {
                    Ok(Some(JValue::Object(Some(JObject::new(result)?))))
                }
            }
            JType::Boolean => {
                let result = env.call_static_boolean_method(raw_cls, raw_id, args.as_ptr());
                Ok(Some(JValue::Bool(result != 0)))
            }
            JType::Byte => {
                let result = env.call_static_byte_method(raw_cls, raw_id, args.as_ptr());
                Ok(Some(JValue::Byte(result)))
            }
            JType::Char => {
                let result = env.call_static_char_method(raw_cls, raw_id, args.as_ptr());
                Ok(Some(JValue::Char(
                    std::char::from_u32(result as u32).expect("Java returned bad char")
                )))
            }
            JType::Short => {
                let result = env.call_static_short_method(raw_cls, raw_id, args.as_ptr());
                Ok(Some(JValue::Short(result)))
            }
            JType::Int => {
                let result = env.call_static_int_method(raw_cls, raw_id, args.as_ptr());
                Ok(Some(JValue::Int(result)))
            }
            JType::Long => {
                let result = env.call_static_long_method(raw_cls, raw_id, args.as_ptr());
                Ok(Some(JValue::Long(result)))
            }
            JType::Float => {
                let result = env.call_static_float_method(raw_cls, raw_id, args.as_ptr());
                Ok(Some(JValue::Float(result)))
            }
            JType::Double => {
                let result = env.call_static_double_method(raw_cls, raw_id, args.as_ptr());
                Ok(Some(JValue::Double(result)))
            }
            JType::Void => {
                env.call_static_void_method(raw_cls, raw_id, args.as_ptr());
                Ok(None)
            }
        }
    }

    pub fn get_static_field_id(&self, cls: &JClass, name: &str, sig: &str) -> Result<JFieldID> {
        let env = self.internal_env();
        let c_name = cstr_from_str(name)?;

        let sig = mangle_class(sig);
        let ty= sig.java_type().as_nonvoid().expect("Expected field type to be non-void");

        let c_sig = cstr_from_str(&sig.mangled())?;

        // SAFETY: Internal pointer use
        unsafe {
            let id = env.get_static_field_id(cls.borrow_ptr(), c_name.as_ptr(), c_sig.as_ptr());
            if id.is_null() {
                Err(Error::new(&format!("Couldn't get static field id of {}", name), ffi::constants::JNI_ERR))
            } else {
                Ok(JFieldID::new(id, ty)?)
            }
        }
    }

    pub fn get_static_field(&self, cls: &JClass, id: &JFieldID) -> Result<JValue> {
        let env = self.internal_env();

        let raw_cls;
        let raw_id;
        // SAFETY: Internal pointer use
        unsafe {
            raw_cls = cls.borrow_ptr();
            raw_id = id.borrow_ptr();
        }

        match id.ty() {
            JNonVoidType::Object => {
                let result = env.get_static_object_field(raw_cls, raw_id);
                if result.is_null() {
                    Ok(JValue::Object(None))
                } else {
                    Ok(JValue::Object(Some(JObject::new(result)?)))
                }
            }
            JNonVoidType::Boolean => {
                let result = env.get_static_boolean_field(raw_cls, raw_id);
                Ok(JValue::Bool(result != 0))
            }
            JNonVoidType::Byte => {
                let result = env.get_static_byte_field(raw_cls, raw_id);
                Ok(JValue::Byte(result))
            }
            JNonVoidType::Char => {
                let result = env.get_static_char_field(raw_cls, raw_id);
                Ok(JValue::Char(std::char::from_u32(result as u32).expect("Java returned bad char")))
            }
            JNonVoidType::Short => {
                let result = env.get_static_short_field(raw_cls, raw_id);
                Ok(JValue::Short(result))
            }
            JNonVoidType::Int => {
                let result = env.get_static_int_field(raw_cls, raw_id);
                Ok(JValue::Int(result))
            }
            JNonVoidType::Long => {
                let result = env.get_static_long_field(raw_cls, raw_id);
                Ok(JValue::Long(result))
            }
            JNonVoidType::Float => {
                let result = env.get_static_float_field(raw_cls, raw_id);
                Ok(JValue::Float(result))
            }
            JNonVoidType::Double => {
                let result = env.get_static_double_field(raw_cls, raw_id);
                Ok(JValue::Double(result))
            }
        }
    }

    pub fn set_static_field(&self, cls: &JClass, id: &JFieldID, val: JValue) -> Result<()> {
        let env = self.internal_env();

        let raw_cls;
        let raw_id;
        // SAFETY: Internal pointer use
        unsafe {
            raw_cls = cls.borrow_ptr();
            raw_id = id.borrow_ptr();
        }

        match id.ty() {
            JNonVoidType::Object => {
                // SAFETY: Internal pointer use
                unsafe {
                    let obj = val
                        .into_obj()?
                        .map(|obj| {obj.borrow_ptr()})
                        .unwrap_or(std::ptr::null_mut());

                    env.set_static_object_field(raw_cls, raw_id, obj);
                }
                Ok(())
            }
            JNonVoidType::Boolean => {
                env.set_static_boolean_field(raw_cls, raw_id, val.into_bool()? as ffi::JBoolean);
                Ok(())
            }
            JNonVoidType::Byte => {
                env.set_static_byte_field(raw_cls, raw_id, val.into_byte()? as ffi::JByte);
                Ok(())
            }
            JNonVoidType::Char => {
                env.set_static_char_field(raw_cls, raw_id, val.into_char()? as ffi::JChar);
                Ok(())
            }
            JNonVoidType::Short => {
                env.set_static_short_field(raw_cls, raw_id, val.into_short()? as ffi::JShort);
                Ok(())
            }
            JNonVoidType::Int => {
                env.set_static_int_field(raw_cls, raw_id, val.into_int()? as ffi::JInt);
                Ok(())
            }
            JNonVoidType::Long => {
                env.set_static_long_field(raw_cls, raw_id, val.into_long()? as ffi::JLong);
                Ok(())
            }
            JNonVoidType::Float => {
                env.set_static_float_field(raw_cls, raw_id, val.into_float()? as ffi::JFloat);
                Ok(())
            }
            JNonVoidType::Double => {
                env.set_static_double_field(raw_cls, raw_id, val.into_double()? as ffi::JDouble);
                Ok(())
            }
        }
    }

    pub fn new_string(&self, chars: &[char]) -> Result<JString> {
        let env = self.internal_env();

        let chars: Vec<u16> = chars.iter().map(|c| {*c as u16}).collect();

        let result = env.new_string(chars.as_ptr(), chars.len() as i32);
        if result.is_null() {
            Err(Error::new("Couldn't create new string", ffi::constants::JNI_ERR))
        } else {
            Ok(JString::new(result)?)
        }
    }

    pub fn get_string_length(&self, str: &JString) -> usize {
        let env = self.internal_env();

        // SAFETY: Internal pointer use
        unsafe {
            env.get_string_length(str.borrow_ptr()) as usize
        }
    }

    pub fn get_string_chars(&self, str: &JString) -> Vec<char> {
        let env = self.internal_env();
        let mut is_copy = 0;

        // SAFETY: Internal pointer use, Java verifies returned pointer will be valid
        unsafe {
            let chars = env.get_string_chars(str.borrow_ptr(), &mut is_copy);

            let raw_slice = slice::from_raw_parts(chars, self.get_string_length(str));

            let out = Vec::from(raw_slice)
                .into_iter()
                .map(|c| {
                    std::char::from_u32(c as u32).expect("Java returned bad char")
                })
                .collect();

            env.release_string_chars(str.borrow_ptr(), chars);

            out
        }
    }

    pub fn new_string_utf(&self, str: &str) -> Result<JString> {
        let env = self.internal_env();
        let c_str = cstr_from_str(str)?;

        let new_str = env.new_string_utf(c_str.as_ptr());
        if new_str.is_null() {
            Err(Error::new("Couldn't create string from UTF", ffi::constants::JNI_ERR))
        } else {
            Ok(JString::new(new_str)?)
        }
    }

    pub fn get_string_utf_length(&self, str: &JString) -> usize {
        let env = self.internal_env();

        // SAFETY: Internal pointer use
        unsafe {
            env.get_string_utf_length(str.borrow_ptr()) as usize
        }
    }

    pub fn get_string_utf_chars(&self, str: &JString) -> &[u8] {
        let env = self.internal_env();
        let mut is_copy = 0;

        // SAFETY: Internal pointer use, Java verifies returned pointer will be valid
        unsafe {
            let chars = env.get_string_utf_chars(str.borrow_ptr(), &mut is_copy) as *const u8;
            let raw_slice = slice::from_raw_parts(chars, self.get_string_utf_length(str));
            raw_slice
        }
    }

    pub fn release_string_utf_chars(&self, str: &JString, chars: &[i8]) {
        let env = self.internal_env();

        // SAFETY: Internal pointer use
        unsafe {
            env.release_string_utf_chars(str.borrow_ptr(), chars.as_ptr())
        }
    }

    pub fn get_array_length(&self, array: &JArray) -> usize {
        let env = self.internal_env();

        // SAFETY: Internal pointer use
        unsafe {
            env.get_array_length(array.borrow_ptr()) as usize
        }
    }

    pub fn new_object_array(&self, len: usize, cls: &JClass, init: Option<&JObject>) -> Result<JObjectArray> {
        let env = self.internal_env();

        // SAFETY: Internal pointer use
        unsafe {
            let raw_init;
            if let Some(obj) = init {
                raw_init = obj.borrow_ptr();
            } else {
                raw_init = std::ptr::null_mut();
            }

            let result = env.new_object_array(len as i32, cls.borrow_ptr(), raw_init);

            if result.is_null() {
                Err(Error::new("Couldn't create new object array", ffi::constants::JNI_ERR))
            } else {
                Ok(JObjectArray::new(result)?)
            }
        }
    }

    pub fn get_object_array_element(&self, array: &JObjectArray, idx: usize) -> Result<JObject> {
        let env = self.internal_env();

        if idx >= self.get_array_length(array.downcast()) {
            return Err(Error::new("Index outside array bounds", ffi::constants::JNI_ERR));
        }

        // SAFETY: Internal pointer use
        unsafe {
            let result = env.get_object_array_element(array.borrow_ptr(), idx as i32);
            if result.is_null() {
                Err(Error::new("Failed to get array element", ffi::constants::JNI_ERR))
            } else {
                Ok(JObject::new(result)?)
            }
        }
    }

    pub fn set_object_array_element(&self, array: &JObjectArray, idx: usize, val: &JObject) -> Result<()> {
        let env = self.internal_env();

        if idx >= self.get_array_length(array.downcast()) {
            return Err(Error::new("Index outside array bounds", ffi::constants::JNI_ERR))
        }

        // SAFETY: Internal pointer use
        unsafe {
            env.set_object_array_element(array.borrow_ptr(), idx as i32, val.borrow_ptr());
            Ok(())
        }
    }

    pub fn new_native_array(&self, len: usize, ty: JNativeType) -> Result<JNativeArray> {
        let len = len as i32;
        let env = self.internal_env();

        let result;
        match ty {
            JNativeType::Boolean => {
                result = env.new_boolean_array(len) as *mut ffi::JArray;
            }
            JNativeType::Byte => {
                result = env.new_byte_array(len) as *mut ffi::JArray;
            }
            JNativeType::Char => {
                result = env.new_char_array(len) as *mut ffi::JArray;
            }
            JNativeType::Short => {
                result = env.new_short_array(len) as *mut ffi::JArray;
            }
            JNativeType::Int => {
                result = env.new_int_array(len) as *mut ffi::JArray;
            }
            JNativeType::Long => {
                result = env.new_long_array(len) as *mut ffi::JArray;
            }
            JNativeType::Float => {
                result = env.new_float_array(len) as *mut ffi::JArray;
            }
            JNativeType::Double => {
                result = env.new_double_array(len) as *mut ffi::JArray;
            }
        }

        if result.is_null() {
            Err(Error::new("Couldn't create new native array", ffi::constants::JNI_ERR))
        } else {
            // SAFETY: Types must match do to above match statement
            unsafe {
                Ok(JNativeArray::new_raw(result, ty)?)
            }
        }
    }

    pub fn get_native_array_elements<'a>(&self, arr: &'a JNativeArray ) -> Result<JNativeSlice<'a>> {
        let env = self.internal_env();
        let jarr = arr.as_jarray();

        // SAFETY: Internal pointer use
        unsafe {
            let mut is_copy = 0;
            let len = self.get_array_length(jarr);
            let ptr: *mut std::ffi::c_void;

            match arr {
                JNativeArray::Boolean(arr) => {
                    ptr = env.get_boolean_array_elements(arr.borrow_ptr(), &mut is_copy) as _;
                }
                JNativeArray::Byte(arr) => {
                    ptr = env.get_byte_array_elements(arr.borrow_ptr(), &mut is_copy) as _;
                }
                JNativeArray::Char(arr) => {
                    ptr = env.get_char_array_elements(arr.borrow_ptr(), &mut is_copy) as _;
                }
                JNativeArray::Short(arr) => {
                    ptr = env.get_short_array_elements(arr.borrow_ptr(), &mut is_copy) as _;
                }
                JNativeArray::Int(arr) => {
                    ptr = env.get_int_array_elements(arr.borrow_ptr(), &mut is_copy) as _;
                }
                JNativeArray::Long(arr) => {
                    ptr = env.get_long_array_elements(arr.borrow_ptr(), &mut is_copy) as _;
                }
                JNativeArray::Float(arr) => {
                    ptr = env.get_float_array_elements(arr.borrow_ptr(), &mut is_copy) as _;
                }
                JNativeArray::Double(arr) => {
                    ptr = env.get_double_array_elements(arr.borrow_ptr(), &mut is_copy) as _;
                }
            }

            if ptr.is_null() {
                Err(Error::new("Couldn't get array elements", ffi::constants::JNI_ERR))
            } else {
                match arr {
                    JNativeArray::Boolean(_) => {
                        Ok(JNativeSlice::Boolean(slice::from_raw_parts_mut(ptr as _, len)))
                    }
                    JNativeArray::Byte(_) => {
                        Ok(JNativeSlice::Byte(slice::from_raw_parts_mut(ptr as _, len)))
                    }
                    JNativeArray::Char(_) => {
                        Ok(JNativeSlice::Char(slice::from_raw_parts_mut(ptr as _, len)))
                    }
                    JNativeArray::Short(_) => {
                        Ok(JNativeSlice::Short(slice::from_raw_parts_mut(ptr as _, len)))
                    }
                    JNativeArray::Int(_) => {
                        Ok(JNativeSlice::Int(slice::from_raw_parts_mut(ptr as _, len)))
                    }
                    JNativeArray::Long(_) => {
                        Ok(JNativeSlice::Long(slice::from_raw_parts_mut(ptr as _, len)))
                    }
                    JNativeArray::Float(_) => {
                        Ok(JNativeSlice::Float(slice::from_raw_parts_mut(ptr as _, len)))
                    }
                    JNativeArray::Double(_) => {
                        Ok(JNativeSlice::Double(slice::from_raw_parts_mut(ptr as _, len)))
                    }
                }
            }
        }
    }

    // TODO: Check arr and slice are the same
    pub fn release_native_array_elements(&self, arr: &JNativeArray, slice: &JNativeSlice, mode: ReleaseMode) {
        let env = self.internal_env();
        let ptr: *mut std::ffi::c_void;
        let mode = mode.into();

        match slice {
            JNativeSlice::Boolean(slice) => {
                ptr = slice.as_ptr() as _
            }
            JNativeSlice::Byte(slice) => {
                ptr = slice.as_ptr() as _
            }
            JNativeSlice::Char(slice) => {
                ptr = slice.as_ptr() as _
            }
            JNativeSlice::Short(slice) => {
                ptr = slice.as_ptr() as _
            }
            JNativeSlice::Int(slice) => {
                ptr = slice.as_ptr() as _
            }
            JNativeSlice::Long(slice) => {
                ptr = slice.as_ptr() as _
            }
            JNativeSlice::Float(slice) => {
                ptr = slice.as_ptr() as _
            }
            JNativeSlice::Double(slice) => {
                ptr = slice.as_ptr() as _
            }
        }

        // SAFETY: Internal pointer use
        unsafe {
            match arr {
                JNativeArray::Boolean(arr) => {
                    env.release_boolean_array_elements(arr.borrow_ptr(), ptr as _, mode)
                }
                JNativeArray::Byte(arr) => {
                    env.release_byte_array_elements(arr.borrow_ptr(), ptr as _, mode)
                }
                JNativeArray::Char(arr) => {
                    env.release_char_array_elements(arr.borrow_ptr(), ptr as _, mode)
                }
                JNativeArray::Short(arr) => {
                    env.release_short_array_elements(arr.borrow_ptr(), ptr as _, mode)
                }
                JNativeArray::Int(arr) => {
                    env.release_int_array_elements(arr.borrow_ptr(), ptr as _, mode)
                }
                JNativeArray::Long(arr) => {
                    env.release_long_array_elements(arr.borrow_ptr(), ptr as _, mode)
                }
                JNativeArray::Float(arr) => {
                    env.release_float_array_elements(arr.borrow_ptr(), ptr as _, mode)
                }
                JNativeArray::Double(arr) => {
                    env.release_double_array_elements(arr.borrow_ptr(), ptr as _, mode)
                }
            }
        }
    }

    pub fn get_native_array_region(&self, arr: &JNativeArray, start: usize, len: usize) -> Result<JNativeVec> {
        let env = self.internal_env();

        unsafe {
            match arr {
                JNativeArray::Boolean(arr) => {
                    let mut out = Vec::with_capacity(len);
                    env.get_boolean_array_region(arr.borrow_ptr(), start as i32, len as i32, out.as_mut_ptr());
                    Ok(JNativeVec::Boolean(out.into_iter().map(|b| {b != 0}).collect()))
                }
                JNativeArray::Byte(arr) => {
                    let mut out = Vec::with_capacity(len);
                    env.get_byte_array_region(arr.borrow_ptr(), start as i32, len as i32, out.as_mut_ptr());
                    Ok(JNativeVec::Byte(out))
                }
                JNativeArray::Char(arr) => {
                    let mut out = Vec::with_capacity(len);
                    env.get_char_array_region(arr.borrow_ptr(), start as i32, len as i32, out.as_mut_ptr());
                    Ok(JNativeVec::Char(out.into_iter().map(|c| {std::char::from_u32(c as u32).expect("Java returned bad char")}).collect()))
                }
                JNativeArray::Short(arr) => {
                    let mut out = Vec::with_capacity(len);
                    env.get_short_array_region(arr.borrow_ptr(), start as i32, len as i32, out.as_mut_ptr());
                    Ok(JNativeVec::Short(out))
                }
                JNativeArray::Int(arr) => {
                    let mut out = Vec::with_capacity(len);
                    env.get_int_array_region(arr.borrow_ptr(), start as i32, len as i32, out.as_mut_ptr());
                    Ok(JNativeVec::Int(out))
                }
                JNativeArray::Long(arr) => {
                    let mut out = Vec::with_capacity(len);
                    env.get_long_array_region(arr.borrow_ptr(), start as i32, len as i32, out.as_mut_ptr());
                    Ok(JNativeVec::Long(out))
                }
                JNativeArray::Float(arr) => {
                    let mut out = Vec::with_capacity(len);
                    env.get_float_array_region(arr.borrow_ptr(), start as i32, len as i32, out.as_mut_ptr());
                    Ok(JNativeVec::Float(out))
                }
                JNativeArray::Double(arr) => {
                    let mut out = Vec::with_capacity(len);
                    env.get_double_array_region(arr.borrow_ptr(), start as i32, len as i32, out.as_mut_ptr());
                    Ok(JNativeVec::Double(out))
                }
            }
        }
    }

    // TODO: Check arr and vec match
    pub fn set_native_array_region(&self, arr: &JNativeArray, start: usize, len: usize, slice: &JNativeVec) -> Result<()> {
        let env = self.internal_env();
        let start = start as i32;
        let len = len as i32;

        // SAFETY: Internal pointer use
        unsafe {
            match arr {
                JNativeArray::Boolean(arr) => {
                    let temp: Vec<_>;
                    if let JNativeVec::Boolean(vec) = slice {
                        temp = vec.iter().map(|b| {*b as u8}).collect()
                    } else {
                        unreachable!()
                    }

                    env.set_boolean_array_region(arr.borrow_ptr(), start, len, temp.as_ptr());
                }
                JNativeArray::Byte(arr) => {
                    let temp: &Vec<_>;
                    if let JNativeVec::Byte(vec) = slice {
                        temp = vec;
                    } else {
                        unreachable!()
                    }

                    env.set_byte_array_region(arr.borrow_ptr(), start, len, temp.as_ptr());
                }
                JNativeArray::Char(arr) => {
                    let temp: Vec<_>;
                    if let JNativeVec::Char(vec) = slice {
                        temp = vec.iter().map(|c| {*c as u16}).collect()
                    } else {
                        unreachable!()
                    }

                    env.set_char_array_region(arr.borrow_ptr(), start, len, temp.as_ptr());
                }
                JNativeArray::Short(arr) => {
                    let temp: &Vec<_>;
                    if let JNativeVec::Short(vec) = slice {
                        temp = vec;
                    } else {
                        unreachable!()
                    }

                    env.set_short_array_region(arr.borrow_ptr(), start, len, temp.as_ptr());
                }
                JNativeArray::Int(arr) => {
                    let temp: &Vec<_>;
                    if let JNativeVec::Int(vec) = slice {
                        temp = vec;
                    } else {
                        unreachable!()
                    }

                    env.set_int_array_region(arr.borrow_ptr(), start, len, temp.as_ptr());
                }
                JNativeArray::Long(arr) => {
                    let temp: &Vec<_>;
                    if let JNativeVec::Long(vec) = slice {
                        temp = vec;
                    } else {
                        unreachable!()
                    }

                    env.set_long_array_region(arr.borrow_ptr(), start, len, temp.as_ptr());
                }
                JNativeArray::Float(arr) => {
                    let temp: &Vec<_>;
                    if let JNativeVec::Float(vec) = slice {
                        temp = vec;
                    } else {
                        unreachable!()
                    }

                    env.set_float_array_region(arr.borrow_ptr(), start, len, temp.as_ptr());
                }
                JNativeArray::Double(arr) => {
                    let temp: &Vec<_>;
                    if let JNativeVec::Double(vec) = slice {
                        temp = vec;
                    } else {
                        unreachable!()
                    }

                    env.set_double_array_region(arr.borrow_ptr(), start, len, temp.as_ptr());
                }
            }

            Ok(())
        }
    }

    pub fn register_natives(&self, cls: &JClass, methods: &[JNINativeMethod]) -> Result<()> {
        let env = self.internal_env();

        // SAFETY: Internal pointer use
        unsafe {
            let methods = JNINativeMethod::make_ffi_vec(methods);
            let result = env.register_natives(cls.borrow_ptr(), methods.as_ptr(), methods.len() as i32);
            if result != 0 {
                Err(Error::new("Couldn't register native methods", result))
            } else {
                Ok(())
            }
        }
    }

    pub fn unregister_natives(&self, cls: &JClass) -> Result<()> {
        let env = self.internal_env();

        // SAFETY: Internal pointer use
        unsafe {
            let result = env.unregister_natives(cls.borrow_ptr());
            if result != 0 {
                Err(Error::new("Couldn't unregister native methods", result))
            } else {
                Ok(())
            }
        }
    }

    pub fn monitor_enter(&self, obj: &JObject) -> Result<()> {
        let env = self.internal_env();

        // SAFETY: Internal pointer use
        unsafe {
            let result = env.monitor_enter(obj.borrow_ptr());
            if result != 0 {
                Err(Error::new("Couldn't enter monitor", result))
            } else {
                Ok(())
            }
        }
    }

    pub fn monitor_exit(&self, obj: &JObject) -> Result<()> {
        let env = self.internal_env();

        // SAFETY: Internal pointer use
        unsafe {
            let result = env.monitor_exit(obj.borrow_ptr());
            if result != 0 {
                Err(Error::new("Couldn't exit monitor", result))
            } else {
                Ok(())
            }
        }
    }

    pub fn get_jvm(&self) -> Result<JavaVM> {
        let env = self.internal_env();
        let mut vm = std::ptr::null_mut();
        env.get_java_vm(&mut vm);

        JavaVM::new(self.version, vm)
    }

    pub fn get_string_region(&self, str: JString, start: usize, len: usize) -> Result<Vec<char>> {
        let env = self.internal_env();
        let mut buffer = Vec::with_capacity(len);

        // SAFETY: Internal pointer use
        unsafe {
            env.get_string_region(str.borrow_ptr(), start as i32, len as i32, buffer.as_mut_ptr());
        }

        Ok(buffer.into_iter().map(|c| {std::char::from_u32(c as u32).expect("Java returned bad char")}).collect())
    }

    pub fn get_string_utf_region(&self, str: JString, start: usize, len: usize) -> Result<Vec<u8>> {
        let env = self.internal_env();
        let mut buffer = Vec::with_capacity(len);

        // SAFETY: Internal pointer use
        unsafe {
            env.get_string_utf_region(str.borrow_ptr(), start as i32, len as i32, buffer.as_mut_ptr());
        }

        Ok(buffer.into_iter().map(|c| {c as u8}).collect())
    }

    pub fn get_primitive_array_critical<'a>(&self, arr: &'a JNativeArray) -> Result<JNativeSlice<'a>> {
        let env = self.internal_env();
        let jarr = arr.as_jarray();

        // SAFETY: Internal pointer use
        unsafe {
            let mut is_copy = 0;
            let len = self.get_array_length(jarr);
            let ptr: *mut std::ffi::c_void;

            ptr = env.get_primitive_array_critical(jarr.borrow_ptr() as _, &mut is_copy) as _;

            if ptr.is_null() {
                Err(Error::new("Couldn't get array elements", ffi::constants::JNI_ERR))
            } else {
                match arr {
                    JNativeArray::Boolean(_) => {
                        Ok(JNativeSlice::Boolean(slice::from_raw_parts_mut(ptr as _, len)))
                    }
                    JNativeArray::Byte(_) => {
                        Ok(JNativeSlice::Byte(slice::from_raw_parts_mut(ptr as _, len)))
                    }
                    JNativeArray::Char(_) => {
                        Ok(JNativeSlice::Char(slice::from_raw_parts_mut(ptr as _, len)))
                    }
                    JNativeArray::Short(_) => {
                        Ok(JNativeSlice::Short(slice::from_raw_parts_mut(ptr as _, len)))
                    }
                    JNativeArray::Int(_) => {
                        Ok(JNativeSlice::Int(slice::from_raw_parts_mut(ptr as _, len)))
                    }
                    JNativeArray::Long(_) => {
                        Ok(JNativeSlice::Long(slice::from_raw_parts_mut(ptr as _, len)))
                    }
                    JNativeArray::Float(_) => {
                        Ok(JNativeSlice::Float(slice::from_raw_parts_mut(ptr as _, len)))
                    }
                    JNativeArray::Double(_) => {
                        Ok(JNativeSlice::Double(slice::from_raw_parts_mut(ptr as _, len)))
                    }
                }
            }
        }
    }

    // TODO: Check arr and slice are the same
    pub fn release_primitive_array_critical(&self, arr: &JNativeArray, slice: &JNativeSlice, mode: ReleaseMode) {
        let env = self.internal_env();
        let ptr: *mut std::ffi::c_void;
        let mode = mode.into();
        let jarr = arr.as_jarray();

        match slice {
            JNativeSlice::Boolean(slice) => {
                ptr = slice.as_ptr() as _
            }
            JNativeSlice::Byte(slice) => {
                ptr = slice.as_ptr() as _
            }
            JNativeSlice::Char(slice) => {
                ptr = slice.as_ptr() as _
            }
            JNativeSlice::Short(slice) => {
                ptr = slice.as_ptr() as _
            }
            JNativeSlice::Int(slice) => {
                ptr = slice.as_ptr() as _
            }
            JNativeSlice::Long(slice) => {
                ptr = slice.as_ptr() as _
            }
            JNativeSlice::Float(slice) => {
                ptr = slice.as_ptr() as _
            }
            JNativeSlice::Double(slice) => {
                ptr = slice.as_ptr() as _
            }
        }

        // SAFETY: Internal pointer use
        unsafe {
            env.release_primitive_array_critical(jarr.borrow_ptr(), ptr as _, mode)
        }
    }

    pub fn new_weak_global_ref(&self, obj: &JObject) -> Result<JWeak<'static>> {
        let env = self.internal_env();

        unsafe {
            let weak = env.new_weak_global_ref(obj.borrow_ptr());
            if weak.is_null() {
                Err(Error::new("Couldn't create weak global reference", ffi::constants::JNI_ERR))
            } else {
                Ok(JWeak::new(weak)?)
            }
        }
    }

    pub fn delete_weak_global_ref(&self, weak: JWeak<'static>) {
        let env = self.internal_env();

        unsafe {
            env.delete_weak_global_ref(weak.borrow_ptr())
        }
    }

    pub fn new_direct_byte_buffer<'a>(&self, buff: &'a mut [u8]) -> Result<JObject<'a>> {
        let env = self.internal_env();

        let obj = env.new_direct_byte_buffer(
            buff.as_mut_ptr() as *mut std::ffi::c_void,
            buff.len() as i64
        );

        if obj.is_null() {
            Err(Error::new("Couldn't create direct byte buffer", ffi::constants::JNI_ERR))
        } else {
            Ok(JObject::new(obj)?)
        }
    }

    pub fn get_direct_buffer_slice<'a>(&self, buff: &JObject<'a>) -> Result<&'a mut [u8]> {
        let env = self.internal_env();

        // SAFETY: Internal pointer use
        unsafe {
            let pos = env.get_direct_buffer_address(buff.borrow_ptr());
            let len = env.get_direct_buffer_capacity(buff.borrow_ptr());

            if pos.is_null() {
                Err(Error::new("Couldn't get buffer from object", ffi::constants::JNI_ERR))
            } else {
                Ok(slice::from_raw_parts_mut(pos as *mut u8, len as usize))
            }
        }
    }

    pub fn get_object_ref_type(&self, obj: &JObject) -> JRefType {
        let env = self.internal_env();

        // SAFETY: Internal pointer use
        unsafe {
            let result = env.get_object_ref_type(obj.borrow_ptr());
            result.into()
        }
    }

    pub fn get_module(&self, cls: &JClass) -> Result<JObject> {
        let env = self.internal_env();

        // SAFETY: Internal pointer use
        unsafe {
            let result = env.get_module(cls.borrow_ptr());
            if result.is_null() {
                Err(Error::new("Couldn't get module for class", ffi::constants::JNI_ERR))
            } else {
                Ok(JObject::new(result)?)
            }
        }
    }
}
