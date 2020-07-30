
use std::ffi::CString;
use std::slice;

use crate::{ffi, JByte};
use crate::types::{JNIVersion, JType, JValue, JObject, JClass, JMethodID, JFieldID, JThrowable, JString, JArray, JObjectArray, JavaDownCast, JNonVoidType, JNINativeMethod, JavaUpCast};
use crate::error::Error;
use crate::mangling::{mangle_class, TypeSignature};
use crate::vm::JavaVM;
use crate::types::jtype::JRefType;
use std::collections::HashMap;
use std::cell::RefCell;
use crate::types::object::JWeak;
use crate::object::JByteBuffer;


/// Handy utility for converting a `&str` into a `CString`, returning a rust_jni error on failure
fn cstr_from_str(str: &str) -> Result<CString, Error> {
    CString::new(str)
        .map_err(|err| {
            Error::from(Box::new(err))
        })
}


/// Higher-level construct representing a JNIEnv
pub struct JNIEnv {
    version: JNIVersion,
    backing_ptr: *mut ffi::JNIEnv,
    obj_refs: RefCell<HashMap<*mut ffi::JObject, JObject>>
}

impl JNIEnv {

    /// Create a new JNIEnv from a pointer to an [ffi::JNIEnv]. This environment will
    /// live as long as the current thread, generally. Thus this type is not marked Send or Sync.
    pub fn new(env: *mut ffi::JNIEnv) -> Result<JNIEnv, Error> {
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
                backing_ptr: env,
                obj_refs: RefCell::new(HashMap::new())
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

    /// Non-public, because it would be incredibly unsafe. Stores a pointer as a key and a JObject
    /// constructed from that pointer as a value, returning a reference to that JObject cast
    /// as any other smart-object.
    fn local_ref<T, U>(&self, ptr: *mut T) -> &U {
        let ptr = ptr as *mut ffi::JObject;
        let mut obj_refs = self.obj_refs.borrow_mut();

        if !obj_refs.contains_key(&ptr) {
            let obj = JObject::new(ptr).expect("Built local reference from invalid pointer");
            obj_refs.insert(ptr, obj);
        }

        // SAFETY: Converting between 'smart' objects, which have the same size and backing
        unsafe {
            &*(&obj_refs[&ptr] as *const JObject as *const U)
        }
    }

    fn drop_local_ref<T>(&self, ptr: *mut T) {
        let ptr = ptr as *mut ffi::JObject;
        let mut obj_refs = self.obj_refs.borrow_mut();
        obj_refs.remove(&ptr);
    }

    pub fn get_version(&self) -> JNIVersion {
        let env = self.internal_env();
        JNIVersion::from(env.get_version())
    }

    pub fn define_class(&self, name: &str, loader: &JObject, buffer: &[i8]) -> Result<&JClass, Error> {
        let env = self.internal_env();
        let name = cstr_from_str(name)?;

        // SAFETY: Internal pointer use
        unsafe {
            let new_cls = env.define_class(name.as_ptr(), loader.borrow_ptr(), buffer.as_ptr(), buffer.len() as i32);
            if new_cls.is_null() {
                Err(Error::new("Could not define new Java Class", ffi::constants::JNI_ERR))
            } else {
                Ok(self.local_ref(new_cls))
            }
        }
    }

    pub fn find_class(&self, name: &str) -> Result<&JClass, Error> {
        let env = self.internal_env();
        let c_name = cstr_from_str(&mangle_class(name).mangled())?;

        let new_cls = env.find_class(c_name.as_ptr());
        if new_cls.is_null() {
            Err(Error::new(&format!("Could not find Java Class {}", name), ffi::constants::JNI_ERR))
        } else {
            Ok(self.local_ref(new_cls))
        }
    }

    pub fn from_reflected_method(&self, method: &JObject) -> Result<JMethodID, Error> {
        let env = self.internal_env();
        let meth_cls = self.find_class("java.lang.reflect.Method").unwrap();
        let cls_cls = self.find_class("java.lang.Class").unwrap();
        let get_ret = self.get_method_id(meth_cls, "getReturnType", "() -> java.lang.Class").unwrap();
        let get_num_args = self.get_method_id(meth_cls, "getParameterCount", "() -> int").unwrap();
        let get_name = self.get_method_id(cls_cls, "getName", "() -> java.lang.String").unwrap();

        // SAFETY: Internal pointer use
        unsafe {
            let id = env.from_reflected_method(method.borrow_ptr());
            let ret_cls = self.call_method(method, &get_ret, &vec![])
                .unwrap()
                .unwrap()
                .into_obj()
                .unwrap()
                .unwrap();
            let ret_name = self.call_method(ret_cls, &get_name, &vec![])
                .unwrap()
                .unwrap()
                .into_obj()
                .unwrap()
                .unwrap();
            let num_args = self.call_method(method, &get_num_args, &vec![])
                .unwrap()
                .unwrap()
                .into_int()
                .unwrap() as usize;

            let chars = self.get_string_utf_chars(ret_name.upcast_raw());
            let chars = CString::new(chars).unwrap();
            let ret_type = JType::from_name(&chars.into_string().unwrap());

            if id.is_null() {
                Err(Error::new("Could not find method ID", ffi::constants::JNI_ERR))
            } else {
                Ok(JMethodID::new(id, ret_type, num_args )?)
            }
        }
    }

    pub fn from_reflected_field(&self, method: &JObject) -> Result<JFieldID, Error> {
        let env = self.internal_env();

        // TODO
        unimplemented!();
        // SAFETY: Internal pointer use
        // unsafe {
        //     let id = env.from_reflected_field(method.borrow_ptr());
        //     if id.is_null() {
        //         Err(Error::new("Could not find field ID", ffi::constants::JNI_ERR))
        //     } else {
        //         Ok(JFieldID::new(id)?)
        //     }
        // }
    }

    /// TODO: Maybe make is_static part of IDs?
    pub fn to_reflected_method(&self, cls: &JClass, id: &JMethodID, is_static: bool) -> Result<&JObject, Error> {
        let env = self.internal_env();

        // SAFETY: Internal pointer use
        unsafe {
            let obj = env.to_reflected_method(cls.borrow_ptr(), id.borrow_ptr(), is_static.into());
            if obj.is_null() {
                Err(Error::new("Could not find reflected method", ffi::constants::JNI_ERR))
            } else {
                Ok(self.local_ref(obj))
            }
        }
    }

    pub fn to_reflected_field(&self, cls: &JClass, id: &JFieldID, is_static: bool) -> Result<&JObject, Error> {
        let env = self.internal_env();

        unsafe {
            let obj = env.to_reflected_field(cls.borrow_ptr(), id.borrow_ptr(), is_static.into());
            if obj.is_null() {
                Err(Error::new("Could not find reflected field", ffi::constants::JNI_ERR))
            } else {
                Ok(self.local_ref(obj))
            }
        }
    }

    pub fn get_superclass(&self, cls: &JClass) -> Result<&JClass, Error> {
        let env = self.internal_env();

        // SAFETY: Internal pointer use
        unsafe {
            let obj = env.get_superclass(cls.borrow_ptr());
            if obj.is_null() {
                Err(Error::new("Could not get object superclass", ffi::constants::JNI_ERR))
            } else {
                Ok(self.local_ref(obj))
            }
        }
    }

    pub fn is_assignable_from(&self, cls1: &JClass, cls2: &JClass) -> bool {
        let env = self.internal_env();

        // SAFETY: Internal pointer use
        unsafe {
            env.is_assignable_from(cls1.borrow_ptr(), cls2.borrow_ptr()) != 0
        }
    }

    pub fn throw(&self, exception: JThrowable) -> Result<(), Error> {
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

    pub fn throw_new(&self, cls: JClass, msg: &str) -> Result<(), Error> {
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

    pub fn exception_check(&self) -> bool {
        let env = self.internal_env();
        env.exception_check() != 0
    }

    pub fn exception_occurred(&self) -> Result<Option<&JThrowable>, Error> {
        let env = self.internal_env();

        let exc = env.exception_occurred();
        if exc.is_null() {
            Ok(None)
        } else {
            let throwable = self.local_ref(exc);

            Ok(Some(throwable))
        }
    }

    pub fn exception_describe(&self) -> Result<(), ()> {
        let env = self.internal_env();

        if self.exception_check() {
            env.exception_describe();
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn exception_clear(&self) -> Result<(), ()> {
        let env = self.internal_env();

        if self.exception_check() {
            env.exception_clear();
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn fatal_error(&self, msg: &str) -> Result<!, Error> {
        let env = self.internal_env();
        let c_msg = cstr_from_str(msg)?;

        env.fatal_error(c_msg.as_ptr())
    }

    pub fn ensure_local_capacity(&self, capacity: i32) -> Result<(), Error> {
        let env = self.internal_env();

        let result = env.ensure_local_capacity(capacity);
        if result != 0 {
            Err(Error::new(&format!("Couldn't ensure local capacity of at least {}", capacity), result))
        } else {
            Ok(())
        }
    }

    pub fn push_local_frame(&self, capacity: i32) -> Result<(), Error> {
        let env = self.internal_env();

        let result = env.push_local_frame(capacity);
        if result != 0 {
            Err(Error::new(&format!("Couldn't push local from with capacity {}", capacity), result))
        } else {
            Ok(())
        }
    }

    pub fn pop_local_frame<'a>(&self, obj: Option<&JObject>) -> Option<&JObject> {
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
                // Some(JObject::new(out).expect("Null pointer in `pop_local_frame` despite null check"))
                Some(self.local_ref(out))
            }
        }
    }

    pub fn new_global_ref(&self, obj: &JObject) -> Result<JObject, Error> {
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

    pub fn delete_global_ref(&self, obj: JObject) {
        let env = self.internal_env();

        // SAFETY: Internal pointer use
        unsafe {
            env.delete_global_ref(obj.borrow_ptr())
        }
    }

    pub fn new_local_ref(&self, obj: &JObject) -> Result<&JObject, Error> {
        let env = self.internal_env();

        // SAFETY: Internal pointer use
        unsafe {
            let obj = env.new_local_ref(obj.borrow_ptr());
            if obj.is_null() {
                Err(Error::new("Couldn't create new local reference", ffi::constants::JNI_ERR))
            } else {
                Ok(self.local_ref(obj))
            }
        }
    }

    /// Deletes a local reference on the JVM. If you use this, you must ensure that all
    /// other references to the passed JObject are not used. Any use of them past here
    /// will cause undefined behavior.
    pub unsafe fn delete_local_ref(&self, obj: &JObject) {
        let env = self.internal_env();

        // SAFETY: Internal pointer use
        let ptr = obj.borrow_ptr();
        env.delete_local_ref(obj.borrow_ptr());
        self.drop_local_ref(ptr);
    }

    pub fn is_same_object(&self, obj1: &JObject, obj2: &JObject) -> bool {
        let env = self.internal_env();

        // SAFETY: Internal pointer use
        unsafe {
            env.is_same_object(obj1.borrow_ptr(), obj2.borrow_ptr()) != 0
        }
    }

    pub fn alloc_object(&self, cls: &JClass) -> Result<&JObject, Error> {
        let env = self.internal_env();

        // SAFETY: Internal pointer use
        unsafe {
            let obj = env.alloc_object(cls.borrow_ptr());
            if obj.is_null() {
                Err(Error::new("Couldn't allocate object", ffi::constants::JNI_ERR))
            } else {
                Ok(self.local_ref(obj))
            }
        }
    }

    pub fn new_object(&self, cls: &JClass, id: &JMethodID, args: &[JValue]) -> Result<&JObject, Error> {
        let env = self.internal_env();

        // SAFETY: Internal pointer use
        unsafe {
            let c_args = JValue::make_ffi_vec(args);
            let obj = env.new_object(cls.borrow_ptr(), id.borrow_ptr(), c_args.as_ptr());
            if obj.is_null() {
                Err(Error::new("Couldn't create new object", ffi::constants::JNI_ERR))
            } else {
                Ok(self.local_ref(obj))
            }
        }
    }

    pub fn get_object_class(&self, obj: &JObject) -> Result<&JClass, Error> {
        let env = self.internal_env();

        // SAFETY: Internal pointer use
        unsafe {
            let cls = env.get_object_class(obj.borrow_ptr());
            if cls.is_null() {
                Err(Error::new("Couldn't get object class", ffi::constants::JNI_ERR))
            } else {
                Ok(self.local_ref(cls))
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

    pub fn get_method_id(&self, cls: &JClass, name: &str, sig: &str) -> Result<JMethodID, Error> {
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

    pub fn call_method(&self, obj: &JObject, id: &JMethodID, args: &[JValue]) -> Result<Option<JValue>, Error> {
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
                    Ok(Some(JValue::Object(Some(self.local_ref(result)))))
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

    pub fn call_method_nonvoid(&self, obj: &JObject, id: &JMethodID, args: &[JValue]) {

    }

    pub fn call_nonvirtual_method(&self, obj: &JObject, cls: &JClass, id: &JMethodID, args: &[JValue]) -> Result<Option<JValue>, Error> {
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
                    Ok(Some(JValue::Object(Some(self.local_ref(result)))))
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

    pub fn get_field_id(&self, cls: &JClass, name: &str, sig: &str) -> Result<JFieldID, Error> {
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

    pub fn get_field(&self, obj: &JObject, id: &JFieldID) -> Result<JValue, Error> {
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
                    Ok(JValue::Object(Some(self.local_ref(result))))
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

    pub fn set_field(&self, obj: &JObject, id: &JFieldID, val: JValue) -> Result<(), Error> {
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

    pub fn get_static_method_id(&self, cls: &JClass, name: &str, sig: &str) -> Result<JMethodID, Error> {
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

    pub fn call_static_method(&self, cls: &JClass, id: &JMethodID, args: &[JValue]) -> Result<Option<JValue>, Error> {
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

        match id.ret_ty() { // TODO: Add error check for non-object calls?
            JType::Object => {
                let result = env.call_static_object_method(raw_cls, raw_id, args.as_ptr());
                if result.is_null() {
                    Ok(Some(JValue::Object(None)))
                } else {
                    Ok(Some(JValue::Object(Some(self.local_ref(result)))))
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

    pub fn get_static_field_id(&self, cls: &JClass, name: &str, sig: &str) -> Result<JFieldID, Error> {
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

    pub fn get_static_field(&self, cls: &JClass, id: &JFieldID) -> Result<JValue, Error> {
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
                    Ok(JValue::Object(Some(self.local_ref(result))))
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

    pub fn set_static_field(&self, cls: &JClass, id: &JFieldID, val: JValue) -> Result<(), Error> {
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

    pub fn new_string(&self, chars: &[char]) -> Result<&JString, Error> {
        let env = self.internal_env();

        let chars: Vec<u16> = chars.iter().map(|c| {*c as u16}).collect();

        let result = env.new_string(chars.as_ptr(), chars.len() as i32);
        if result.is_null() {
            Err(Error::new("Couldn't create new string", ffi::constants::JNI_ERR))
        } else {
            Ok(self.local_ref(result))
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
                    std::char::from_u32(c as u32).unwrap()
                })
                .collect();

            env.release_string_chars(str.borrow_ptr(), chars);

            out
        }
    }

    pub fn new_string_utf(&self, str: &str) -> Result<&JString, Error> {
        let env = self.internal_env();
        let c_str = cstr_from_str(str)?;

        let new_str = env.new_string_utf(c_str.as_ptr());
        if new_str.is_null() {
            Err(Error::new("Couldn't create string from UTF", ffi::constants::JNI_ERR))
        } else {
            Ok(self.local_ref(new_str))
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

    pub fn new_object_array(&self, len: usize, cls: &JClass, init: Option<&JObject>) -> Result<&JObjectArray, Error> {
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
                Ok(self.local_ref(result))
            }
        }
    }

    pub fn get_object_array_element(&self, array: &JObjectArray, idx: usize) -> Result<&JObject, Error> {
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
                Ok(self.local_ref(result))
            }
        }
    }

    pub fn set_object_array_element(&self, array: &JObjectArray, idx: usize, val: &JObject) -> Result<(), Error> {
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

    // TODO: Figure out native array handling
    pub fn new_native_array(&self) {
        unimplemented!()
    }

    pub fn get_native_array_elements(&self) {
        unimplemented!()
    }

    pub fn release_native_array_elements(&self) {
        unimplemented!()
    }

    pub fn get_native_array_region(&self) {
        unimplemented!()
    }

    pub fn set_native_array_region(&self) {
        unimplemented!()
    }

    pub fn register_natives(&self, cls: &JClass, methods: &[JNINativeMethod]) -> Result<(), Error> {
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

    pub fn unregister_natives(&self, cls: &JClass) -> Result<(), Error> {
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

    pub fn monitor_enter(&self, obj: &JObject) -> Result<(), Error> {
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

    pub fn monitor_exit(&self, obj: &JObject) -> Result<(), Error> {
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

    pub fn get_jvm(&self) -> Result<JavaVM, Error> {
        let env = self.internal_env();
        let mut vm = std::ptr::null_mut();
        env.get_java_vm(&mut vm);

        JavaVM::new(self.version, vm)
    }

    pub fn get_string_region(&self) {}

    pub fn get_string_utf_region(&self) {}

    pub fn get_primitive_array_critical(&self) {}

    pub fn release_primitive_array_critical(&self) {}

    pub fn new_weak_global_ref(&self, obj: &JObject) -> Result<JWeak, Error> {
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

    pub fn delete_weak_global_ref(&self, weak: JWeak) {
        let env = self.internal_env();

        unsafe {
            env.delete_weak_global_ref(weak.borrow_ptr())
        }
    }

    pub fn new_direct_byte_buffer<'a>(&self, buff: &'a [u8]) -> Result<JByteBuffer<'a>, Error> {
        let env = self.internal_env();

        let obj = env.new_direct_byte_buffer(
            buff.as_ptr() as *mut std::ffi::c_void,
            buff.len() as i64
        );

        if obj.is_null() {
            Err(Error::new("Couldn't create direct byte buffer", ffi::constants::JNI_ERR))
        } else {
            Ok(JByteBuffer::new(obj)?)
        }
    }

    pub fn get_direct_buffer_slice<'a>(&self, buff: &JByteBuffer<'a>) -> Result<&'a [u8], Error> {
        let env = self.internal_env();

        // SAFETY: Internal pointer use
        unsafe {
            let pos = env.get_direct_buffer_address(buff.borrow_ptr());
            let len = env.get_direct_buffer_capacity(buff.borrow_ptr());

            if pos.is_null() {
                Err(Error::new("Couldn't get buffer from object", ffi::constants::JNI_ERR))
            } else {
                Ok(slice::from_raw_parts(pos as *const u8, len as usize))
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

    pub fn get_module(&self, cls: &JClass) -> Result<&JObject, Error> {
        let env = self.internal_env();

        // SAFETY: Internal pointer use
        unsafe {
            let result = env.get_module(cls.borrow_ptr());
            if result.is_null() {
                Err(Error::new("Couldn't get module for class", ffi::constants::JNI_ERR))
            } else {
                Ok(self.local_ref(result))
            }
        }
    }
}
