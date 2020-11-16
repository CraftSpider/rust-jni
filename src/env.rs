//!
//! Module containing a higher-level wrapper over a raw JNI Environment. This higher-level
//! implementation provides safe versions of the standard environment operations, as well as merging
//! many of the return-type specific functions into single functions using enums
//!

use std::ffi::CString;
use std::slice;

use crate::{ffi, JNativeType, JNativeArray, JNativeSlice, ReleaseMode, JNativeVec};
use crate::ffi::constants::JNI_ERR;
use crate::types::{JNIVersion, JType, JValue, JObject, JClass, JMethodID, JFieldID, JThrowable, JString, JArray, JObjectArray, JavaDownCast, JNonVoidType, JNINativeMethod, JavaUpCast};
use crate::error::{Error, Result};
use crate::mangling::{mangle_class, TypeSignature};
use crate::vm::JavaVM;
use crate::types::jtype::JRefType;
use crate::types::object::JWeak;


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
            Err(Error::new_null("JNIEnv Constructor"))
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
        let new_cls = unsafe {
            env.define_class(name.as_ptr(), loader.borrow_ptr(), buffer.as_ptr() as _, buffer.len() as i32)
        };

        if new_cls.is_null() {
            Err(Error::new("Could not define new Java Class", JNI_ERR))
        } else {
            Ok(JClass::new(new_cls)?)
        }
    }

    /// Find an existing class by name. The passed name should consist only of ASCII characters
    pub fn find_class(&self, name: &str) -> Result<JClass> {
        let env = self.internal_env();
        let c_name = cstr_from_str(&mangle_class(name).mangled())?;

        let new_cls = env.find_class(c_name.as_ptr());
        if new_cls.is_null() {
            Err(Error::new(&format!("Could not find Java Class {}", name), JNI_ERR))
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
        let id = unsafe { env.from_reflected_method(method.borrow_ptr()) };

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

        // SAFETY: Guaranteed safe upcast, we know the type
        let chars = unsafe { self.get_string_chars(&ret_name.upcast_raw())? };
        let chars: String = chars.into_iter().collect();
        let ret_type = JType::from_name(&chars);

        if id.is_null() {
            Err(Error::new("Could not find method ID", JNI_ERR))
        } else {
            Ok(JMethodID::new(id, ret_type, num_args )?)
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
        let id = unsafe { env.from_reflected_field(field.borrow_ptr()) };

        let ty_cls = self.call_method(field, &get_ty, &vec![])?
            .expect("Unexpected void result")
            .into_obj()?
            .expect("Unexpected null result");
        let ty_name = self.call_method(&ty_cls, &get_name, &vec![])?
            .expect("Unexpected void result")
            .into_obj()?
            .expect("Unexpected null result");

        // SAFETY: Guaranteed safe upcast, we know the type
        let chars = unsafe { self.get_string_chars(&ty_name.upcast_raw())? };
        let chars: String = chars.into_iter().collect();
        let ty = JType::from_name(&chars).as_nonvoid().unwrap();

        if id.is_null() {
            Err(Error::new("Could not find field ID", JNI_ERR))
        } else {
            Ok(JFieldID::new(id, ty)?)
        }
    }

    /// Build a reflected Method object from a class, method ID, and static-ness
    ///
    /// TODO: Maybe make is_static part of IDs?
    pub fn to_reflected_method(&self, cls: &JClass, id: &JMethodID, is_static: bool) -> Result<JObject> {
        let env = self.internal_env();

        // SAFETY: Internal pointer use
        let obj = unsafe {
            env.to_reflected_method(cls.borrow_ptr(), id.borrow_ptr(), is_static.into())
        };

        if obj.is_null() {
            Err(Error::new("Could not find reflected method", JNI_ERR))
        } else {
            Ok(JObject::new(obj)?)
        }
    }

    /// Build a reflected Field object from a class, field ID, and static-ness
    ///
    /// TODO: Maybe make is_static part of IDs?
    pub fn to_reflected_field(&self, cls: &JClass, id: &JFieldID, is_static: bool) -> Result<JObject> {
        let env = self.internal_env();

        // SAFETY: Internal pointer use
        let obj = unsafe {
            env.to_reflected_field(cls.borrow_ptr(), id.borrow_ptr(), is_static.into())
        };

        if obj.is_null() {
            Err(Error::new("Could not find reflected field", JNI_ERR))
        } else {
            Ok(JObject::new(obj)?)
        }
    }

    /// Get the superclass of a given class. Will return an error if the class is Object or other
    /// class with no superclass.
    pub fn get_superclass(&self, cls: &JClass) -> Result<JClass> {
        let env = self.internal_env();

        // SAFETY: Internal pointer use
        let obj = unsafe { env.get_superclass(cls.borrow_ptr()) };
        if obj.is_null() {
            Err(Error::new("Could not get object superclass", JNI_ERR))
        } else {
            Ok(JClass::new(obj)?)
        }
    }

    /// Checks whether an object with the type of the first argument can be safely cast to an object
    /// with the type of the second object
    pub fn is_assignable_from(&self, from: &JClass, to: &JClass) -> bool {
        let env = self.internal_env();

        // SAFETY: Internal pointer use
        unsafe {
            env.is_assignable_from(from.borrow_ptr(), to.borrow_ptr())
        }
    }

    /// Start throwing an exception on the JVM. Result is Ok if exception *is* thrown, Err if no
    /// exception is thrown.
    pub fn throw(&self, exception: &JThrowable) -> Result<()> {
        let env = self.internal_env();

        // SAFETY: Internal pointer use
        let result = unsafe { env.throw(exception.borrow_ptr()) };
        if result != 0 {
            Err(Error::new("Could not throw exception", JNI_ERR))
        } else {
            Ok(())
        }
    }

    /// Start throwing a new instance of an exception on the JVM. Result is Ok if exception *is*
    /// thrown, Err if no exception is thrown.
    pub fn throw_new(&self, cls: &JClass, msg: &str) -> Result<()> {
        let env = self.internal_env();
        let c_msg = cstr_from_str(msg)?;

        // SAFETY: Internal pointer use
        let result = unsafe { env.throw_new(cls.borrow_ptr(), c_msg.as_ptr()) };
        if result != 0 {
            Err(Error::new("Could not throw exception", JNI_ERR))
        } else {
            Ok(())
        }
    }

    /// Check whether an exception is currently occuring on the JVM
    pub fn exception_check(&self) -> bool {
        let env = self.internal_env();
        env.exception_check()
    }

    /// Get the current exception being thrown, or Err
    pub fn exception_occurred(&self) -> Result<JThrowable> {
        let env = self.internal_env();

        let exc = env.exception_occurred();
        if exc.is_null() {
            Err(Error::new("No active exception to retrieve", JNI_ERR))
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
            Err(Error::new("No active exception to describe", JNI_ERR))
        }
    }

    /// If an exception is being thrown, clear it. Otherwise Err
    pub fn exception_clear(&self) -> Result<()> {
        let env = self.internal_env();

        if self.exception_check() {
            env.exception_clear();
            Ok(())
        } else {
            Err(Error::new("No active error to clear", JNI_ERR))
        }
    }

    /// Raise a fatal error, and don't expect the JVM to continue.
    pub fn fatal_error(&self, msg: &str) -> Result<!> {
        let env = self.internal_env();
        let c_msg = cstr_from_str(msg)?;

        env.fatal_error(c_msg.as_ptr())
    }

    /// Ensure that the JVM can create at least N many objects. Returns Err if it can't, as on
    /// failure the JVM raises an exception
    pub fn ensure_local_capacity(&self, capacity: i32) -> Result<()> {
        let env = self.internal_env();

        let result = env.ensure_local_capacity(capacity);
        if result != 0 {
            Err(Error::new(&format!("Couldn't ensure local capacity of at least {}", capacity), result))
        } else {
            Ok(())
        }
    }

    /// Push a frame onto the JVM. All references created within this frame will be freed once it
    /// is closed.
    ///
    /// TODO: Maybe make this return a new 'environment' so all refs don't outlive it?
    pub fn push_local_frame(&self, capacity: i32) -> Result<()> {
        let env = self.internal_env();

        let result = env.push_local_frame(capacity);
        if result != 0 {
            Err(Error::new(&format!("Couldn't push local from with capacity {}", capacity), result))
        } else {
            Ok(())
        }
    }

    /// Pop a frame from the JVM. All references created within it are freed, except one passed
    /// as an argument, which is returned
    pub fn pop_local_frame<'a>(&self, obj: Option<JObject<'a>>) -> Option<JObject<'a>> {
        let env = self.internal_env();

        // SAFETY: Internal pointer use
        let ptr = unsafe {
            if let Some(obj) = obj {
                obj.borrow_ptr()
            } else {
                std::ptr::null_mut()
            }
        };

        let out = env.pop_local_frame(ptr);

        if out.is_null() {
            None
        } else {
            Some(JObject::new(out).expect("Null pointer in `pop_local_frame` despite null check"))
            // Some(self.local_ref(out))
        }
    }

    /// Create a new global reference, from an existing reference to an object
    pub fn new_global_ref(&self, obj: &JObject) -> Result<JObject<'static>> {
        let env = self.internal_env();

        // SAFETY: Internal pointer use
        let obj = unsafe { env.new_global_ref(obj.borrow_ptr()) };
        if obj.is_null() {
            Err(Error::new("Couldn't create new globabl reference", JNI_ERR))
        } else {
            Ok(JObject::new(obj)?)
        }
    }

    /// Delete an existing global reference.
    pub fn delete_global_ref(&self, obj: JObject<'static>) {
        let env = self.internal_env();

        // SAFETY: Internal pointer use
        unsafe {
            env.delete_global_ref(obj.borrow_ptr())
        }
    }

    /// Create a new local reference to an object. This can be used to increment refcount and
    /// prevent garbage collection on a delete_local_ref call.
    pub fn new_local_ref(&self, obj: &JObject) -> Result<JObject> {
        let env = self.internal_env();

        // SAFETY: Internal pointer use
        let obj = unsafe { env.new_local_ref(obj.borrow_ptr()) };
        if obj.is_null() {
            Err(Error::new("Couldn't create new local reference", JNI_ERR))
        } else {
            Ok(JObject::new(obj)?)
        }
    }

    /// Deletes a local reference on the JVM. If this reduces the refcount to 0, you must ensure
    /// that all other references to the passed JObject are not used. Any use of them past here
    /// will cause undefined behavior.
    pub fn delete_local_ref(&self, obj: JObject) {
        let env = self.internal_env();

        // SAFETY: Internal pointer use
        unsafe {
            env.delete_local_ref(obj.borrow_ptr());
        }
    }

    /// Check whether two references refer to the same object
    pub fn is_same_object(&self, obj1: &JObject, obj2: &JObject) -> bool {
        let env = self.internal_env();

        // SAFETY: Internal pointer use
        unsafe {
            env.is_same_object(obj1.borrow_ptr(), obj2.borrow_ptr())
        }
    }

    /// Allocate an object with enough space to hold an instance of the passed class, but do not
    /// call any constructor or do any initialization
    pub fn alloc_object(&self, cls: &JClass) -> Result<JObject> {
        let env = self.internal_env();

        // SAFETY: Internal pointer use
        let obj = unsafe { env.alloc_object(cls.borrow_ptr()) };
        if obj.is_null() {
            Err(Error::new("Couldn't allocate object", JNI_ERR))
        } else {
            Ok(JObject::new(obj)?)
        }
    }

    /// Create a new object, calling a constructor with the passed args. Constructors are methods
    /// with the name `<init>`
    pub fn new_object(&self, cls: &JClass, id: &JMethodID, args: &[JValue]) -> Result<JObject> {
        let env = self.internal_env();

        let c_args = JValue::make_ffi_vec(args);

        // SAFETY: Internal pointer use
        let obj = unsafe { env.new_object(cls.borrow_ptr(), id.borrow_ptr(), c_args.as_ptr()) };
        if obj.is_null() {
            Err(Error::new("Couldn't create new object", JNI_ERR))
        } else {
            Ok(JObject::new(obj)?)
        }
    }

    /// Get the class of an object
    pub fn get_object_class(&self, obj: &JObject) -> Result<JClass> {
        let env = self.internal_env();

        // SAFETY: Internal pointer use
        let cls = unsafe { env.get_object_class(obj.borrow_ptr()) };
        if cls.is_null() {
            Err(Error::new("Couldn't get object class", JNI_ERR))
        } else {
            Ok(JClass::new(cls)?)
        }
    }

    /// Check whether an object is an instance of a given class
    pub fn is_instance_of(&self, obj: &JObject, cls: &JClass) -> bool {
        let env = self.internal_env();

        // SAFETY: Internal pointer use
        unsafe {
            env.is_instance_of(obj.borrow_ptr(), cls.borrow_ptr())
        }
    }

    /// Get a method ID from a class, name, and signature. The signature uses the syntax defined
    /// in the root documentation
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
            return Err(Error::new("Expected method signature", JNI_ERR));
        }

        let c_sig = cstr_from_str(&sig.mangled())?;

        // SAFETY: Internal pointer use
        let id = unsafe { env.get_method_id(cls.borrow_ptr(), c_name.as_ptr(), c_sig.as_ptr()) };
        if id.is_null() {
            Err(Error::new(&format!("Couldn't get method id of {}", name), JNI_ERR))
        } else {
            Ok(JMethodID::new(id, ret_ty, num_args)?)
        }
    }

    /// Call a method on an object. Takes the object to bind to `this`, the ID of the method, and
    /// the arguments to pass. Return Err if the method errors, otherwise Ok. Option is None if the
    /// method is void typed, otherwise a JValue containing the return.
    pub fn call_method(&self, obj: &JObject, id: &JMethodID, args: &[JValue]) -> Result<Option<JValue>> {
        if args.len() != id.num_args() {
            return Err(Error::new("Invalid number of arguement for method", JNI_ERR))
        }

        let env = self.internal_env();
        let args = JValue::make_ffi_vec(args);

        // SAFETY: Internal pointer use
        let (raw_obj, raw_id) = unsafe { (
            obj.borrow_ptr(), id.borrow_ptr()
        ) };

        let result = match id.ret_ty() {
            JType::Object => {
                let result = env.call_object_method(raw_obj, raw_id, args.as_ptr());
                if result.is_null() {
                    Some(JValue::Object(None))
                } else {
                    Some(JValue::Object(Some(JObject::new(result)?)))
                }
            }
            JType::Boolean => {
                let result = env.call_boolean_method(raw_obj, raw_id, args.as_ptr());
                Some(JValue::Bool(result))
            }
            JType::Byte => {
                let result = env.call_byte_method(raw_obj, raw_id, args.as_ptr());
                Some(JValue::Byte(result))
            }
            JType::Char => {
                let result = env.call_char_method(raw_obj, raw_id, args.as_ptr());
                Some(JValue::Char(
                    std::char::from_u32(result as u32).expect("Java returned bad char")
                ))
            }
            JType::Short => {
                let result = env.call_short_method(raw_obj, raw_id, args.as_ptr());
                Some(JValue::Short(result))
            }
            JType::Int => {
                let result = env.call_int_method(raw_obj, raw_id, args.as_ptr());
                Some(JValue::Int(result))
            }
            JType::Long => {
                let result = env.call_long_method(raw_obj, raw_id, args.as_ptr());
                Some(JValue::Long(result))
            }
            JType::Float => {
                let result = env.call_float_method(raw_obj, raw_id, args.as_ptr());
                Some(JValue::Float(result))
            }
            JType::Double => {
                let result = env.call_double_method(raw_obj, raw_id, args.as_ptr());
                Some(JValue::Double(result))
            }
            JType::Void => {
                env.call_void_method(raw_obj, raw_id, args.as_ptr());
                None
            }
        };

        if self.exception_check() {
            Err(Error::new("Error occured during method call", JNI_ERR))
        } else {
            Ok(result)
        }
    }

    /// Call a method on an object without doing virtual lookup, instead using a passed class.
    /// Takes the object to bind to `this`, the class to use, the ID of the method, and the
    /// arguments to pass. Return Err if the method errors, otherwise Ok. Option is None if the
    /// method is void typed, otherwise a JValue containing the return.
    pub fn call_nonvirtual_method(&self, obj: &JObject, cls: &JClass, id: &JMethodID, args: &[JValue]) -> Result<Option<JValue>> {
        if args.len() != id.num_args() {
            return Err(Error::new("Invalid number of arguments for method", JNI_ERR))
        }

        let env = self.internal_env();
        let args = JValue::make_ffi_vec(args);

        // SAFETY: Internal pointer use
        let (raw_obj, raw_cls, raw_id) = unsafe { (
            obj.borrow_ptr(), cls.borrow_ptr(), id.borrow_ptr()
        ) };

        let result = match id.ret_ty() {
            JType::Object => {
                let result = env.call_nonvirtual_object_method(raw_obj, raw_cls, raw_id, args.as_ptr());
                if result.is_null() {
                    Some(JValue::Object(None))
                } else {
                    Some(JValue::Object(Some(JObject::new(result)?)))
                }
            }
            JType::Boolean => {
                let result = env.call_nonvirtual_boolean_method(raw_obj, raw_cls, raw_id, args.as_ptr());
                Some(JValue::Bool(result))
            }
            JType::Byte => {
                let result = env.call_nonvirtual_byte_method(raw_obj, raw_cls, raw_id, args.as_ptr());
                Some(JValue::Byte(result))
            }
            JType::Char => {
                let result = env.call_nonvirtual_char_method(raw_obj, raw_cls, raw_id, args.as_ptr());
                Some(JValue::Char(
                    std::char::from_u32(result as u32).expect("Java returned bad char")
                ))
            }
            JType::Short => {
                let result = env.call_nonvirtual_short_method(raw_obj, raw_cls, raw_id, args.as_ptr());
                Some(JValue::Short(result))
            }
            JType::Int => {
                let result = env.call_nonvirtual_int_method(raw_obj, raw_cls, raw_id, args.as_ptr());
                Some(JValue::Int(result))
            }
            JType::Long => {
                let result = env.call_nonvirtual_long_method(raw_obj, raw_cls, raw_id, args.as_ptr());
                Some(JValue::Long(result))
            }
            JType::Float => {
                let result = env.call_nonvirtual_float_method(raw_obj, raw_cls, raw_id, args.as_ptr());
                Some(JValue::Float(result))
            }
            JType::Double => {
                let result = env.call_nonvirtual_double_method(raw_obj, raw_cls, raw_id, args.as_ptr());
                Some(JValue::Double(result))
            }
            JType::Void => {
                env.call_nonvirtual_void_method(raw_obj, raw_cls, raw_id, args.as_ptr());
                None
            }
        };

        if self.exception_check() {
            Err(Error::new("Error occured during method call", JNI_ERR))
        } else {
            Ok(result)
        }
    }

    /// Get a field ID from a class, name, and type. The type uses the syntax defined in the root
    /// documentation
    pub fn get_field_id(&self, cls: &JClass, name: &str, sig: &str) -> Result<JFieldID> {
        let env = self.internal_env();
        let c_name = cstr_from_str(name)?;

        let sig = mangle_class(sig);
        let ty= sig.java_type().as_nonvoid().expect("Expected field type to be non-void");

        let c_sig = cstr_from_str(&sig.mangled())?;

        // SAFETY: Internal pointer use
        let id = unsafe { env.get_field_id(cls.borrow_ptr(), c_name.as_ptr(), c_sig.as_ptr()) };
        if id.is_null() {
            Err(Error::new(&format!("Couldn't get field id of {}", name), JNI_ERR))
        } else {
            Ok(JFieldID::new(id, ty)?)
        }
    }

    /// Get the value of a field on an object. Takes the object to retrieve from and the ID of the
    /// field. Returns Err if the field can't be retrieved, otherwise Ok with a JValue containing
    /// the current value
    pub fn get_field(&self, obj: &JObject, id: &JFieldID) -> Result<JValue> {
        let env = self.internal_env();

        // SAFETY: Internal pointer use
        let (raw_obj, raw_id) = unsafe { (
            obj.borrow_ptr(), id.borrow_ptr()
        ) };

        Ok(match id.ty() {
            JNonVoidType::Object => {
                let result = env.get_object_field(raw_obj, raw_id);
                if result.is_null() {
                    JValue::Object(None)
                } else {
                    JValue::Object(Some(JObject::new(result)?))
                }
            }
            JNonVoidType::Boolean => {
                let result = env.get_boolean_field(raw_obj, raw_id);
                JValue::Bool(result)
            }
            JNonVoidType::Byte => {
                let result = env.get_byte_field(raw_obj, raw_id);
                JValue::Byte(result)
            }
            JNonVoidType::Char => {
                let result = env.get_char_field(raw_obj, raw_id);
                JValue::Char(std::char::from_u32(result as u32).expect("Java returned bad char"))
            }
            JNonVoidType::Short => {
                let result = env.get_short_field(raw_obj, raw_id);
                JValue::Short(result)
            }
            JNonVoidType::Int => {
                let result = env.get_int_field(raw_obj, raw_id);
                JValue::Int(result)
            }
            JNonVoidType::Long => {
                let result = env.get_long_field(raw_obj, raw_id);
                JValue::Long(result)
            }
            JNonVoidType::Float => {
                let result = env.get_float_field(raw_obj, raw_id);
                JValue::Float(result)
            }
            JNonVoidType::Double => {
                let result = env.get_double_field(raw_obj, raw_id);
                JValue::Double(result)
            }
        })
    }

    /// Set the value of a field on an object. Takes the object to set the field on and the ID of
    /// the field. Returns Err if the field can't be set, otherwise Ok.
    pub fn set_field(&self, obj: &JObject, id: &JFieldID, val: JValue) -> Result<()> {
        let env = self.internal_env();

        // SAFETY: Internal pointer use
        let (raw_obj, raw_id) = unsafe { (
            obj.borrow_ptr(), id.borrow_ptr()
        ) };

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
            }
            JNonVoidType::Boolean => {
                env.set_boolean_field(raw_obj, raw_id, val.into_bool()? as ffi::JBoolean);
            }
            JNonVoidType::Byte => {
                env.set_byte_field(raw_obj, raw_id, val.into_byte()? as ffi::JByte);
            }
            JNonVoidType::Char => {
                env.set_char_field(raw_obj, raw_id, val.into_char()? as ffi::JChar);
            }
            JNonVoidType::Short => {
                env.set_short_field(raw_obj, raw_id, val.into_short()? as ffi::JShort);
            }
            JNonVoidType::Int => {
                env.set_int_field(raw_obj, raw_id, val.into_int()? as ffi::JInt);
            }
            JNonVoidType::Long => {
                env.set_long_field(raw_obj, raw_id, val.into_long()? as ffi::JLong);
            }
            JNonVoidType::Float => {
                env.set_float_field(raw_obj, raw_id, val.into_float()? as ffi::JFloat);
            }
            JNonVoidType::Double => {
                env.set_double_field(raw_obj, raw_id, val.into_double()? as ffi::JDouble);
            }
        }

        Ok(())
    }

    /// Get a static method ID from a class, name, and signature. The signature uses the syntax
    /// defined in the root documentation
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
            return Err(Error::new("Expected method signature", JNI_ERR));
        }

        let c_sig = cstr_from_str(&sig.mangled())?;

        // SAFETY: Internal pointer use
        let id = unsafe { env.get_static_method_id(cls.borrow_ptr(), c_name.as_ptr(), c_sig.as_ptr()) };
        if id.is_null() {
            Err(Error::new(&format!("Couldn't get static method id of {}", name), JNI_ERR))
        } else {
            Ok(JMethodID::new(id, ret_ty, num_args)?)
        }
    }

    /// Call a static method on an class. Takes the class to use, the ID of the method, and the
    /// arguments to pass. Return Err if the method errors, otherwise Ok. Option is None if the
    /// method is void typed, otherwise a JValue containing the return.
    pub fn call_static_method(&self, cls: &JClass, id: &JMethodID, args: &[JValue]) -> Result<Option<JValue>> {
        if args.len() != id.num_args() {
            return Err(Error::new("Invalid number of arguments for method", JNI_ERR))
        }

        let env = self.internal_env();
        let args = JValue::make_ffi_vec(args);

        // SAFETY: Internal pointer use
        let (raw_cls, raw_id) = unsafe { (
            cls.borrow_ptr(), id.borrow_ptr()
        ) };

        let result = match id.ret_ty() {
            JType::Object => {
                let result = env.call_static_object_method(raw_cls, raw_id, args.as_ptr());
                if result.is_null() {
                    Some(JValue::Object(None))
                } else {
                    Some(JValue::Object(Some(JObject::new(result)?)))
                }
            }
            JType::Boolean => {
                let result = env.call_static_boolean_method(raw_cls, raw_id, args.as_ptr());
                Some(JValue::Bool(result))
            }
            JType::Byte => {
                let result = env.call_static_byte_method(raw_cls, raw_id, args.as_ptr());
                Some(JValue::Byte(result))
            }
            JType::Char => {
                let result = env.call_static_char_method(raw_cls, raw_id, args.as_ptr());
                Some(JValue::Char(
                    std::char::from_u32(result as u32).expect("Java returned bad char")
                ))
            }
            JType::Short => {
                let result = env.call_static_short_method(raw_cls, raw_id, args.as_ptr());
                Some(JValue::Short(result))
            }
            JType::Int => {
                let result = env.call_static_int_method(raw_cls, raw_id, args.as_ptr());
                Some(JValue::Int(result))
            }
            JType::Long => {
                let result = env.call_static_long_method(raw_cls, raw_id, args.as_ptr());
                Some(JValue::Long(result))
            }
            JType::Float => {
                let result = env.call_static_float_method(raw_cls, raw_id, args.as_ptr());
                Some(JValue::Float(result))
            }
            JType::Double => {
                let result = env.call_static_double_method(raw_cls, raw_id, args.as_ptr());
                Some(JValue::Double(result))
            }
            JType::Void => {
                env.call_static_void_method(raw_cls, raw_id, args.as_ptr());
                None
            }
        };

        if self.exception_check() {
            Err(Error::new("Error occured during method call", JNI_ERR))
        } else {
            Ok(result)
        }
    }

    /// Get a static field ID from a class, name, and type. The type uses the syntax defined in the
    /// root documentation
    pub fn get_static_field_id(&self, cls: &JClass, name: &str, sig: &str) -> Result<JFieldID> {
        let env = self.internal_env();
        let c_name = cstr_from_str(name)?;

        let sig = mangle_class(sig);
        let ty= sig.java_type().as_nonvoid().expect("Expected field type to be non-void");

        let c_sig = cstr_from_str(&sig.mangled())?;

        // SAFETY: Internal pointer use
        let id = unsafe {
            env.get_static_field_id(cls.borrow_ptr(), c_name.as_ptr(), c_sig.as_ptr())
        };

        if id.is_null() {
            Err(Error::new(&format!("Couldn't get static field id of {}", name), JNI_ERR))
        } else {
            Ok(JFieldID::new(id, ty)?)
        }
    }

    /// Get the value of a static field on a class. Takes the class to retrieve from and the ID of
    /// the field. Returns Err if the field can't be retrieved, otherwise Ok with a JValue
    /// containing the current value
    pub fn get_static_field(&self, cls: &JClass, id: &JFieldID) -> Result<JValue> {
        let env = self.internal_env();

        // SAFETY: Internal pointer use
        let (raw_cls, raw_id) = unsafe { (
            cls.borrow_ptr(), id.borrow_ptr()
        ) };

        Ok(match id.ty() {
            JNonVoidType::Object => {
                let result = env.get_static_object_field(raw_cls, raw_id);
                if result.is_null() {
                    JValue::Object(None)
                } else {
                    JValue::Object(Some(JObject::new(result)?))
                }
            }
            JNonVoidType::Boolean => {
                let result = env.get_static_boolean_field(raw_cls, raw_id);
                JValue::Bool(result)
            }
            JNonVoidType::Byte => {
                let result = env.get_static_byte_field(raw_cls, raw_id);
                JValue::Byte(result)
            }
            JNonVoidType::Char => {
                let result = env.get_static_char_field(raw_cls, raw_id);
                JValue::Char(std::char::from_u32(result as u32).expect("Java returned bad char"))
            }
            JNonVoidType::Short => {
                let result = env.get_static_short_field(raw_cls, raw_id);
                JValue::Short(result)
            }
            JNonVoidType::Int => {
                let result = env.get_static_int_field(raw_cls, raw_id);
                JValue::Int(result)
            }
            JNonVoidType::Long => {
                let result = env.get_static_long_field(raw_cls, raw_id);
                JValue::Long(result)
            }
            JNonVoidType::Float => {
                let result = env.get_static_float_field(raw_cls, raw_id);
                JValue::Float(result)
            }
            JNonVoidType::Double => {
                let result = env.get_static_double_field(raw_cls, raw_id);
                JValue::Double(result)
            }
        })
    }

    /// Set the value of a static field on a class. Takes the class to set the field on and the ID
    /// of the field. Returns Err if the field can't be set, otherwise Ok.
    pub fn set_static_field(&self, cls: &JClass, id: &JFieldID, val: JValue) -> Result<()> {
        let env = self.internal_env();

        // SAFETY: Internal pointer use
        let (raw_cls, raw_id) = unsafe { (
            cls.borrow_ptr(), id.borrow_ptr()
        ) };

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
            }
            JNonVoidType::Boolean => {
                env.set_static_boolean_field(raw_cls, raw_id, val.into_bool()? as ffi::JBoolean);
            }
            JNonVoidType::Byte => {
                env.set_static_byte_field(raw_cls, raw_id, val.into_byte()? as ffi::JByte);
            }
            JNonVoidType::Char => {
                env.set_static_char_field(raw_cls, raw_id, val.into_char()? as ffi::JChar);
            }
            JNonVoidType::Short => {
                env.set_static_short_field(raw_cls, raw_id, val.into_short()? as ffi::JShort);
            }
            JNonVoidType::Int => {
                env.set_static_int_field(raw_cls, raw_id, val.into_int()? as ffi::JInt);
            }
            JNonVoidType::Long => {
                env.set_static_long_field(raw_cls, raw_id, val.into_long()? as ffi::JLong);
            }
            JNonVoidType::Float => {
                env.set_static_float_field(raw_cls, raw_id, val.into_float()? as ffi::JFloat);
            }
            JNonVoidType::Double => {
                env.set_static_double_field(raw_cls, raw_id, val.into_double()? as ffi::JDouble);
            }
        }

        Ok(())
    }

    /// Create a new [String][JString] object from a slice of characters
    pub fn new_string(&self, chars: &[char]) -> Result<JString> {
        let env = self.internal_env();

        let chars: Vec<u16> = chars.iter().map(|c| {*c as u16}).collect();

        let result = env.new_string(chars.as_ptr(), chars.len() as i32);
        if result.is_null() {
            Err(Error::new("Couldn't create new string", JNI_ERR))
        } else {
            Ok(JString::new(result)?)
        }
    }

    /// Get the length of a [String][JString] in terms of number of [char]s
    pub fn get_string_length(&self, str: &JString) -> usize {
        let env = self.internal_env();

        // SAFETY: Internal pointer use
        unsafe {
            env.get_string_length(str.borrow_ptr()) as usize
        }
    }

    /// Get a vector of the [char]s in a [String][JString]
    pub fn get_string_chars(&self, str: &JString) -> Result<Vec<char>> {
        let env = self.internal_env();
        let mut is_copy = false;

        // SAFETY: Internal pointer use
        let chars = unsafe { env.get_string_chars(str.borrow_ptr(), &mut is_copy) };

        if chars.is_null() {
            return Err(Error::new("Couldn't get string characters", JNI_ERR))
        }

        // SAFETY: Java verifies returned pointer will be valid until release_string_chars is called
        let raw_slice = unsafe { slice::from_raw_parts(chars, self.get_string_length(str)) };

        let out = raw_slice
            .into_iter()
            .map(|c| {
                std::char::from_u32(*c as u32).expect("Java returned bad char")
            })
            .collect();

        // SAFETY: Internal pointer use
        unsafe {
            env.release_string_chars(str.borrow_ptr(), chars)
        }

        Ok(out)
    }

    /// Create a new [String][JString] object from a UTF string
    pub fn new_string_utf(&self, str: &str) -> Result<JString> {
        let env = self.internal_env();
        let c_str = cstr_from_str(str)?;

        let new_str = env.new_string_utf(c_str.as_ptr());
        if new_str.is_null() {
            Err(Error::new("Couldn't create string from UTF", JNI_ERR))
        } else {
            Ok(JString::new(new_str)?)
        }
    }

    /// Get the length of a [String][JString] in terms of number of modified UTF bytes
    pub fn get_string_utf_length(&self, str: &JString) -> usize {
        let env = self.internal_env();

        // SAFETY: Internal pointer use
        unsafe {
            env.get_string_utf_length(str.borrow_ptr()) as usize
        }
    }

    /// Get the characters of a [String][JString] as a slice of modified UTF bytes
    pub fn get_string_utf_chars(&self, str: &JString) -> Result<Vec<u8>> {
        let env = self.internal_env();
        let mut is_copy = false;

        // SAFETY: Internal pointer use
        let chars = unsafe { env.get_string_utf_chars(str.borrow_ptr(), &mut is_copy) as *const u8 };

        if chars.is_null() {
            return Err(Error::new("Couldn't get string characters", JNI_ERR))
        }

        // SAFETY: Java verifies returned pointer will be valid until release_string_utf_chars is called
        let raw_slice = unsafe { slice::from_raw_parts(chars, self.get_string_utf_length(str)) };

        let vec: Vec<_> = raw_slice.iter().cloned().collect();

        // SAFETY: Internal pointer use
        unsafe {
            env.release_string_utf_chars(str.borrow_ptr(), chars as _)
        }

        Ok(vec)
    }

    /// Get the length of an array
    pub fn get_array_length(&self, array: &JArray) -> usize {
        let env = self.internal_env();

        // SAFETY: Internal pointer use
        unsafe {
            env.get_array_length(array.borrow_ptr()) as usize
        }
    }

    /// Create a new array of objects, with a type of the given class and initialized to the given
    /// object value.
    pub fn new_object_array(&self, len: usize, cls: &JClass, init: Option<&JObject>) -> Result<JObjectArray> {
        let env = self.internal_env();

        // SAFETY: Internal pointer use
        let raw_init = unsafe {
            if let Some(obj) = init {
                obj.borrow_ptr()
            } else {
                std::ptr::null_mut()
            }
        };

        // SAFETY: Internal pointer use
        let result = unsafe { env.new_object_array(len as i32, cls.borrow_ptr(), raw_init) };

        if result.is_null() {
            Err(Error::new("Couldn't create new object array", JNI_ERR))
        } else {
            Ok(JObjectArray::new(result)?)
        }
    }

    /// Get the element of an object array at a given index
    pub fn get_object_array_element(&self, array: &JObjectArray, idx: usize) -> Result<JObject> {
        let env = self.internal_env();

        if idx >= self.get_array_length(array.downcast()) {
            return Err(Error::new("Index outside array bounds", JNI_ERR));
        }

        // SAFETY: Internal pointer use
        let result = unsafe { env.get_object_array_element(array.borrow_ptr(), idx as i32) };
        if result.is_null() {
            Err(Error::new("Failed to get array element", JNI_ERR))
        } else {
            Ok(JObject::new(result)?)
        }
    }

    /// Set the element of an object array at a given index
    pub fn set_object_array_element(&self, array: &JObjectArray, idx: usize, val: &JObject) -> Result<()> {
        let env = self.internal_env();

        if idx >= self.get_array_length(array.downcast()) {
            return Err(Error::new("Index outside array bounds", JNI_ERR))
        }

        // SAFETY: Internal pointer use
        unsafe {
            env.set_object_array_element(array.borrow_ptr(), idx as i32, val.borrow_ptr());
        }

        Ok(())
    }

    /// Create a new java array of a primitive type
    pub fn new_native_array(&self, len: usize, ty: JNativeType) -> Result<JNativeArray> {
        let len = len as i32;
        let env = self.internal_env();

        let result: *mut ffi::JArray = match ty {
            JNativeType::Boolean =>
                env.new_boolean_array(len) as _,
            JNativeType::Byte =>
                env.new_byte_array(len) as _,
            JNativeType::Char =>
                env.new_char_array(len) as _,
            JNativeType::Short =>
                env.new_short_array(len) as _,
            JNativeType::Int =>
                env.new_int_array(len) as _,
            JNativeType::Long =>
                env.new_long_array(len) as _,
            JNativeType::Float =>
                env.new_float_array(len) as _,
            JNativeType::Double =>
                env.new_double_array(len) as _
        };

        if result.is_null() {
            Err(Error::new("Couldn't create new native array", JNI_ERR))
        } else {
            // SAFETY: Types must match do to above match statement
            unsafe {
                Ok(JNativeArray::new_raw(result, ty)?)
            }
        }
    }

    /// Get a whole-array slice of a primitive java array
    pub fn get_native_array_elements<'a>(&self, arr: &'a JNativeArray ) -> Result<JNativeSlice<'a>> {
        let env = self.internal_env();
        let jarr = arr.as_jarray();

        // SAFETY: Internal pointer use
        unsafe {
            let mut is_copy = false;
            let len = self.get_array_length(jarr);

            let ptr: *mut std::ffi::c_void = match arr {
                JNativeArray::Boolean(arr) =>
                    env.get_boolean_array_elements(arr.borrow_ptr(), &mut is_copy) as _,
                JNativeArray::Byte(arr) =>
                    env.get_byte_array_elements(arr.borrow_ptr(), &mut is_copy) as _,
                JNativeArray::Char(arr) =>
                    env.get_char_array_elements(arr.borrow_ptr(), &mut is_copy) as _,
                JNativeArray::Short(arr) =>
                    env.get_short_array_elements(arr.borrow_ptr(), &mut is_copy) as _,
                JNativeArray::Int(arr) =>
                    env.get_int_array_elements(arr.borrow_ptr(), &mut is_copy) as _,
                JNativeArray::Long(arr) =>
                    env.get_long_array_elements(arr.borrow_ptr(), &mut is_copy) as _,
                JNativeArray::Float(arr) =>
                    env.get_float_array_elements(arr.borrow_ptr(), &mut is_copy) as _,
                JNativeArray::Double(arr) =>
                    env.get_double_array_elements(arr.borrow_ptr(), &mut is_copy) as _
            };

            if ptr.is_null() {
                Err(Error::new("Couldn't get array elements", JNI_ERR))
            } else {
                Ok(match arr {
                    JNativeArray::Boolean(_) =>
                        JNativeSlice::Boolean(slice::from_raw_parts_mut(ptr as _, len)),
                    JNativeArray::Byte(_) =>
                        JNativeSlice::Byte(slice::from_raw_parts_mut(ptr as _, len)),
                    JNativeArray::Char(_) =>
                        JNativeSlice::Char(slice::from_raw_parts_mut(ptr as _, len)),
                    JNativeArray::Short(_) =>
                        JNativeSlice::Short(slice::from_raw_parts_mut(ptr as _, len)),
                    JNativeArray::Int(_) =>
                        JNativeSlice::Int(slice::from_raw_parts_mut(ptr as _, len)),
                    JNativeArray::Long(_) =>
                        JNativeSlice::Long(slice::from_raw_parts_mut(ptr as _, len)),
                    JNativeArray::Float(_) =>
                        JNativeSlice::Float(slice::from_raw_parts_mut(ptr as _, len)),
                    JNativeArray::Double(_) =>
                        JNativeSlice::Double(slice::from_raw_parts_mut(ptr as _, len))
                })
            }
        }
    }

    /// Release a whole-array slice of a primitive java array
    pub fn release_native_array_elements(&self, arr: &JNativeArray, slice: JNativeSlice, mode: ReleaseMode) -> Result<()> {
        if arr.jtype() != slice.jtype() {
            return Err(Error::new("Invalid array/slice combo", JNI_ERR))
        }

        let env = self.internal_env();
        let mode = mode.into();

        // SAFETY: Internal pointer use
        unsafe {
            match arr {
                JNativeArray::Boolean(arr) => {
                    env.release_boolean_array_elements(arr.borrow_ptr(), slice.borrow_ptr() as _, mode)
                }
                JNativeArray::Byte(arr) => {
                    env.release_byte_array_elements(arr.borrow_ptr(), slice.borrow_ptr() as _, mode)
                }
                JNativeArray::Char(arr) => {
                    env.release_char_array_elements(arr.borrow_ptr(), slice.borrow_ptr() as _, mode)
                }
                JNativeArray::Short(arr) => {
                    env.release_short_array_elements(arr.borrow_ptr(), slice.borrow_ptr() as _, mode)
                }
                JNativeArray::Int(arr) => {
                    env.release_int_array_elements(arr.borrow_ptr(), slice.borrow_ptr() as _, mode)
                }
                JNativeArray::Long(arr) => {
                    env.release_long_array_elements(arr.borrow_ptr(), slice.borrow_ptr() as _, mode)
                }
                JNativeArray::Float(arr) => {
                    env.release_float_array_elements(arr.borrow_ptr(), slice.borrow_ptr() as _, mode)
                }
                JNativeArray::Double(arr) => {
                    env.release_double_array_elements(arr.borrow_ptr(), slice.borrow_ptr() as _, mode)
                }
            }
        }

        Ok(())
    }

    /// Get a partial slice of a primitive java array
    pub fn get_native_array_region(&self, arr: &JNativeArray, start: usize, len: usize) -> Result<JNativeVec> {
        let env = self.internal_env();

        unsafe {
            Ok(match arr {
                JNativeArray::Boolean(arr) => {
                    let mut out = Vec::with_capacity(len);
                    env.get_boolean_array_region(arr.borrow_ptr(), start as i32, len as i32, out.as_mut_ptr());
                    JNativeVec::Boolean(out)
                }
                JNativeArray::Byte(arr) => {
                    let mut out = Vec::with_capacity(len);
                    env.get_byte_array_region(arr.borrow_ptr(), start as i32, len as i32, out.as_mut_ptr());
                    JNativeVec::Byte(out)
                }
                JNativeArray::Char(arr) => {
                    let mut out = Vec::with_capacity(len);
                    env.get_char_array_region(arr.borrow_ptr(), start as i32, len as i32, out.as_mut_ptr());
                    JNativeVec::Char(out.into_iter().map(|c| {std::char::from_u32(c as u32).expect("Java returned bad char")}).collect())
                }
                JNativeArray::Short(arr) => {
                    let mut out = Vec::with_capacity(len);
                    env.get_short_array_region(arr.borrow_ptr(), start as i32, len as i32, out.as_mut_ptr());
                    JNativeVec::Short(out)
                }
                JNativeArray::Int(arr) => {
                    let mut out = Vec::with_capacity(len);
                    env.get_int_array_region(arr.borrow_ptr(), start as i32, len as i32, out.as_mut_ptr());
                    JNativeVec::Int(out)
                }
                JNativeArray::Long(arr) => {
                    let mut out = Vec::with_capacity(len);
                    env.get_long_array_region(arr.borrow_ptr(), start as i32, len as i32, out.as_mut_ptr());
                    JNativeVec::Long(out)
                }
                JNativeArray::Float(arr) => {
                    let mut out = Vec::with_capacity(len);
                    env.get_float_array_region(arr.borrow_ptr(), start as i32, len as i32, out.as_mut_ptr());
                    JNativeVec::Float(out)
                }
                JNativeArray::Double(arr) => {
                    let mut out = Vec::with_capacity(len);
                    env.get_double_array_region(arr.borrow_ptr(), start as i32, len as i32, out.as_mut_ptr());
                    JNativeVec::Double(out)
                }
            })
        }
    }

    /// Release a partial slice of a primitive java array
    pub fn set_native_array_region(&self, arr: &JNativeArray, start: usize, len: usize, slice: &JNativeVec) -> Result<()> {
        if arr.jtype() != slice.jtype() {
            return Err(Error::new("Invalid array/vec combo", JNI_ERR))
        }

        let env = self.internal_env();
        let start = start as i32;
        let len = len as i32;

        // SAFETY: Internal pointer use
        unsafe {
            match arr {
                JNativeArray::Boolean(arr) => {
                    let temp: &Vec<_>;
                    if let JNativeVec::Boolean(vec) = slice {
                        temp = vec;
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

    /// Register a set of native methods to a Java class
    pub fn register_natives(&self, cls: &JClass, methods: &[JNINativeMethod]) -> Result<()> {
        let env = self.internal_env();

        let methods = JNINativeMethod::make_ffi_vec(methods);

        // SAFETY: Internal pointer use
        let result = unsafe { env.register_natives(cls.borrow_ptr(), methods.as_ptr(), methods.len() as i32) };
        if result != 0 {
            Err(Error::new("Couldn't register native methods", result))
        } else {
            Ok(())
        }
    }

    /// Unregister native methods from a java class
    pub fn unregister_natives(&self, cls: &JClass) -> Result<()> {
        let env = self.internal_env();

        // SAFETY: Internal pointer use
        let result = unsafe { env.unregister_natives(cls.borrow_ptr()) };
        if result != 0 {
            Err(Error::new("Couldn't unregister native methods", result))
        } else {
            Ok(())
        }
    }

    /// Enter the perf monitor for an object
    pub fn monitor_enter(&self, obj: &JObject) -> Result<()> {
        let env = self.internal_env();

        // SAFETY: Internal pointer use
        let result = unsafe { env.monitor_enter(obj.borrow_ptr()) };
        if result != 0 {
            Err(Error::new("Couldn't enter monitor", result))
        } else {
            Ok(())
        }
    }

    /// Exit the perf monitor for an object
    pub fn monitor_exit(&self, obj: &JObject) -> Result<()> {
        let env = self.internal_env();

        // SAFETY: Internal pointer use
        let result = unsafe { env.monitor_exit(obj.borrow_ptr()) };
        if result != 0 {
            Err(Error::new("Couldn't exit monitor", result))
        } else {
            Ok(())
        }
    }

    /// Get the JVM instance associated with this environment
    pub fn get_jvm(&self) -> Result<JavaVM> {
        let env = self.internal_env();
        let mut vm = std::ptr::null_mut();
        env.get_java_vm(&mut vm);

        JavaVM::new(self.version, vm, false)
    }

    /// Get a region of a string as a vector of chars
    pub fn get_string_region(&self, str: JString, start: usize, len: usize) -> Result<Vec<char>> {
        let env = self.internal_env();
        let mut buffer = Vec::with_capacity(len);

        // SAFETY: Internal pointer use
        unsafe {
            env.get_string_region(str.borrow_ptr(), start as i32, len as i32, buffer.as_mut_ptr());
        }

        Ok(buffer.into_iter().map(|c| {std::char::from_u32(c as u32).expect("Java returned bad char")}).collect())
    }

    /// Get a region of a string as a vector of bytes
    pub fn get_string_utf_region(&self, str: JString, start: usize, len: usize) -> Result<Vec<u8>> {
        let env = self.internal_env();
        let mut buffer = Vec::with_capacity(len);

        // SAFETY: Internal pointer use
        unsafe {
            env.get_string_utf_region(str.borrow_ptr(), start as i32, len as i32, buffer.as_mut_ptr());
        }

        Ok(buffer.into_iter().map(|c| {c as u8}).collect())
    }

    /// Get a region of a primitive java array, with some limits:
    /// - No other JNI methods should be called before this slice is released
    /// - We should not block on code that might itself rely on a different thread that calls JNI
    ///   methods
    /// This increases the likelihood of the JVM not copying the array backing
    pub fn get_primitive_array_critical<'a>(&self, arr: &'a JNativeArray) -> Result<JNativeSlice<'a>> {
        let env = self.internal_env();
        let jarr = arr.as_jarray();

        // SAFETY: Internal pointer use
        unsafe {
            let mut is_copy = false;
            let len = self.get_array_length(jarr);
            let ptr = env.get_primitive_array_critical(jarr.borrow_ptr() as _, &mut is_copy);

            if ptr.is_null() {
                Err(Error::new("Couldn't get array elements", JNI_ERR))
            } else {
                Ok(match arr {
                    JNativeArray::Boolean(_) =>
                        JNativeSlice::Boolean(slice::from_raw_parts_mut(ptr as _, len)),
                    JNativeArray::Byte(_) =>
                        JNativeSlice::Byte(slice::from_raw_parts_mut(ptr as _, len)),
                    JNativeArray::Char(_) =>
                        JNativeSlice::Char(slice::from_raw_parts_mut(ptr as _, len)),
                    JNativeArray::Short(_) =>
                        JNativeSlice::Short(slice::from_raw_parts_mut(ptr as _, len)),
                    JNativeArray::Int(_) =>
                        JNativeSlice::Int(slice::from_raw_parts_mut(ptr as _, len)),
                    JNativeArray::Long(_) =>
                        JNativeSlice::Long(slice::from_raw_parts_mut(ptr as _, len)),
                    JNativeArray::Float(_) =>
                        JNativeSlice::Float(slice::from_raw_parts_mut(ptr as _, len)),
                    JNativeArray::Double(_) =>
                        JNativeSlice::Double(slice::from_raw_parts_mut(ptr as _, len))
                })
            }
        }
    }

    /// Release a region of a primitive java array
    pub fn release_primitive_array_critical(&self, arr: &JNativeArray, slice: &JNativeSlice, mode: ReleaseMode) -> Result<()> {
        if arr.jtype() != slice.jtype() {
            return Err(Error::new("Invalid array/slice combo", JNI_ERR))
        }

        let env = self.internal_env();
        let mode = mode.into();
        let jarr = arr.as_jarray();

        // SAFETY: Internal pointer use
        unsafe {
            env.release_primitive_array_critical(jarr.borrow_ptr(), slice.borrow_ptr(), mode)
        }

        Ok(())
    }

    /// Create a new weak global reference to an object. This reference only lives as long as other,
    /// stronger references exist.
    pub fn new_weak_global_ref(&self, obj: &JObject) -> Result<JWeak<'static>> {
        let env = self.internal_env();

        // SAFETY: Internal pointer use
        let weak = unsafe { env.new_weak_global_ref(obj.borrow_ptr()) };
        if weak.is_null() {
            Err(Error::new("Couldn't create weak global reference", JNI_ERR))
        } else {
            Ok(JWeak::new(weak)?)
        }
    }

    /// Delete an existing weak global reference to an object
    pub fn delete_weak_global_ref(&self, weak: JWeak<'static>) {
        let env = self.internal_env();

        unsafe {
            env.delete_weak_global_ref(weak.borrow_ptr())
        }
    }

    /// Create a new direct byte buffer from a slice of bytes
    pub fn new_direct_byte_buffer<'a>(&self, buff: &'a mut [u8]) -> Result<JObject<'a>> {
        let env = self.internal_env();

        let obj = env.new_direct_byte_buffer(
            buff.as_mut_ptr() as *mut std::ffi::c_void,
            buff.len() as i64
        );

        if obj.is_null() {
            Err(Error::new("Couldn't create direct byte buffer", JNI_ERR))
        } else {
            Ok(JObject::new(obj)?)
        }
    }

    /// Get a slice from a direct byte buffer object
    pub fn get_direct_buffer_slice<'a>(&self, buff: &JObject<'a>) -> Result<&'a mut [u8]> {
        let env = self.internal_env();

        // SAFETY: Internal pointer use, returned pointer is guaranteed valid as long as buffer is valid
        unsafe {
            let pos = env.get_direct_buffer_address(buff.borrow_ptr());
            let len = env.get_direct_buffer_capacity(buff.borrow_ptr());

            if pos.is_null() {
                Err(Error::new("Couldn't get buffer from object", JNI_ERR))
            } else {
                Ok(slice::from_raw_parts_mut(pos as *mut u8, len as usize))
            }
        }
    }

    /// Get the type of a reference, this function can be used to determine if a reference has been
    /// GCed and is thus no longer safe to use
    pub fn get_object_ref_type(&self, obj: &JObject) -> JRefType {
        let env = self.internal_env();

        // SAFETY: Internal pointer use
        let result = unsafe { env.get_object_ref_type(obj.borrow_ptr()) };
        result.into()
    }

    /// Get the module a class is defined in
    pub fn get_module(&self, cls: &JClass) -> Result<JObject> {
        let env = self.internal_env();

        // SAFETY: Internal pointer use
        let result = unsafe { env.get_module(cls.borrow_ptr()) };
        if result.is_null() {
            Err(Error::new("Couldn't get module for class", JNI_ERR))
        } else {
            Ok(JObject::new(result)?)
        }
    }
}

#[cfg(test)]
mod tests;
