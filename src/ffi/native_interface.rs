//!
//! Module containing the implementation of the JNINativeInterface method table,
//! as well as the JNIEnv functions that rely on it
//!

use std::ffi::c_void;
use crate::ffi::types::*;

///
/// A struct representing the method table backing the JNI environment, the only part of the
/// environment which isn't opaque to the user.
///
#[repr(C)]
pub struct JNINativeInterface {
    reserved0: *const c_void,
    reserved1: *const c_void,
    reserved2: *const c_void,
    reserved3: *const c_void,

    get_version: extern "system" fn(*const JNIEnv) -> JInt,

    define_class: extern "system" fn(*const JNIEnv, *const i8, *mut JObject, *const JByte, JSize) -> *mut JClass,
    find_class: extern "system" fn(*const JNIEnv, *const i8) -> *mut JClass,

    from_reflected_method: extern "system" fn(*const JNIEnv, *mut JObject) -> *const JMethodID,
    from_reflected_field: extern "system" fn(*const JNIEnv, *mut JObject) -> *const JFieldID,
    to_reflected_method: extern "system" fn(*const JNIEnv, *mut JClass, *const JMethodID, JBoolean) -> *mut JObject,

    get_superclass: extern "system" fn(*const JNIEnv, *mut JClass) -> *mut JClass,
    is_assignable_from: extern "system" fn(*const JNIEnv, *mut JClass, *mut JClass) -> JBoolean,

    to_reflected_field: extern "system" fn(*const JNIEnv, *mut JClass, *const JFieldID, JBoolean) -> *mut JObject,

    throw: extern "system" fn(*const JNIEnv, *mut JThrowable) -> JInt,
    throw_new: extern "system" fn(*const JNIEnv, *mut JClass, *const i8) -> JInt,
    exception_occurred: extern "system" fn(*const JNIEnv) -> *mut JThrowable,
    exception_describe: extern "system" fn(*const JNIEnv),
    exception_clear: extern "system" fn(*const JNIEnv),
    fatal_error: extern "system" fn(*const JNIEnv, *const i8) -> !,

    push_local_frame: extern "system" fn(*const JNIEnv, JInt) -> JInt,
    pop_local_frame: extern "system" fn(*const JNIEnv, *mut JObject) -> *mut JObject,

    new_global_ref: extern "system" fn(*const JNIEnv, *mut JObject) -> *mut JObject,
    delete_global_ref: extern "system" fn(*const JNIEnv, *mut JObject),
    delete_local_ref: extern "system" fn(*const JNIEnv, *mut JObject),
    is_same_object: extern "system" fn(*const JNIEnv, *mut JObject, *mut JObject) -> JBoolean,
    new_local_ref: extern "system" fn(*const JNIEnv, *mut JObject) -> *mut JObject,
    ensure_local_capacity: extern "system" fn(*const JNIEnv, JInt) -> JInt,

    alloc_object: extern "system" fn(*const JNIEnv, *mut JClass) -> *mut JObject,

    new_object: extern "cdecl" fn(*const JNIEnv, *mut JClass, *const JMethodID, ...) -> *mut JObject,
    new_object_v: extern "system" fn(*const JNIEnv, *mut JClass, *const JMethodID, std::ffi::VaList) -> *mut JObject,
    new_object_a: extern "system" fn(*const JNIEnv, *mut JClass, *const JMethodID, *const JValue) -> *mut JObject,

    get_object_class: extern "system" fn(*const JNIEnv, *mut JObject) -> *mut JClass,
    is_instance_of: extern "system" fn(*const JNIEnv, *mut JObject, *mut JClass) -> JBoolean,

    get_method_id: extern "system" fn(*const JNIEnv, *mut JClass, *const i8, *const i8) -> *const JMethodID,

    call_object_method: extern "cdecl" fn(*const JNIEnv, *mut JObject, *const JMethodID, ...) -> *mut JObject,
    call_object_method_v: extern "system" fn(*const JNIEnv, *mut JObject, *const JMethodID, std::ffi::VaList) -> *mut JObject,
    call_object_method_a: extern "system" fn(*const JNIEnv, *mut JObject, *const JMethodID, *const JValue) -> *mut JObject,

    call_boolean_method: extern "cdecl" fn(*const JNIEnv, *mut JObject, *const JMethodID, ...) -> JBoolean,
    call_boolean_method_v: extern "system" fn(*const JNIEnv, *mut JObject, *const JMethodID, std::ffi::VaList) -> JBoolean,
    call_boolean_method_a: extern "system" fn(*const JNIEnv, *mut JObject, *const JMethodID, *const JValue) -> JBoolean,

    call_byte_method: extern "cdecl" fn(*const JNIEnv, *mut JObject, *const JMethodID, ...) -> JByte,
    call_byte_method_v: extern "system" fn(*const JNIEnv, *mut JObject, *const JMethodID, std::ffi::VaList) -> JByte,
    call_byte_method_a: extern "system" fn(*const JNIEnv, *mut JObject, *const JMethodID, *const JValue) -> JByte,

    call_char_method: extern "cdecl" fn(*const JNIEnv, *mut JObject, *const JMethodID, ...) -> JChar,
    call_char_method_v: extern "system" fn(*const JNIEnv, *mut JObject, *const JMethodID, std::ffi::VaList) -> JChar,
    call_char_method_a: extern "system" fn(*const JNIEnv, *mut JObject, *const JMethodID, *const JValue) -> JChar,

    call_short_method: extern "cdecl" fn(*const JNIEnv, *mut JObject, *const JMethodID, ...) -> JShort,
    call_short_method_v: extern "system" fn(*const JNIEnv, *mut JObject, *const JMethodID, std::ffi::VaList) -> JShort,
    call_short_method_a: extern "system" fn(*const JNIEnv, *mut JObject, *const JMethodID, *const JValue) -> JShort,

    call_int_method: extern "cdecl" fn(*const JNIEnv, *mut JObject, *const JMethodID, ...) -> JInt,
    call_int_method_v: extern "system" fn(*const JNIEnv, *mut JObject, *const JMethodID, std::ffi::VaList) -> JInt,
    call_int_method_a: extern "system" fn(*const JNIEnv, *mut JObject, *const JMethodID, *const JValue) -> JInt,

    call_long_method: extern "cdecl" fn(*const JNIEnv, *mut JObject, *const JMethodID, ...) -> JLong,
    call_long_method_v: extern "system" fn(*const JNIEnv, *mut JObject, *const JMethodID, std::ffi::VaList) -> JLong,
    call_long_method_a: extern "system" fn(*const JNIEnv, *mut JObject, *const JMethodID, *const JValue) -> JLong,

    call_float_method: extern "cdecl" fn(*const JNIEnv, *mut JObject, *const JMethodID, ...) -> JFloat,
    call_float_method_v: extern "system" fn(*const JNIEnv, *mut JObject, *const JMethodID, std::ffi::VaList) -> JFloat,
    call_float_method_a: extern "system" fn(*const JNIEnv, *mut JObject, *const JMethodID, *const JValue) -> JFloat,

    call_double_method: extern "cdecl" fn(*const JNIEnv, *mut JObject, *const JMethodID, ...) -> JDouble,
    call_double_method_v: extern "system" fn(*const JNIEnv, *mut JObject, *const JMethodID, std::ffi::VaList) -> JDouble,
    call_double_method_a: extern "system" fn(*const JNIEnv, *mut JObject, *const JMethodID, *const JValue) -> JDouble,

    call_void_method: extern "cdecl" fn(*const JNIEnv, *mut JObject, *const JMethodID, ...),
    call_void_method_v: extern "system" fn(*const JNIEnv, *mut JObject, *const JMethodID, std::ffi::VaList),
    call_void_method_a: extern "system" fn(*const JNIEnv, *mut JObject, *const JMethodID, *const JValue),

    call_nonvirtual_object_method: extern "cdecl" fn(*const JNIEnv, *mut JObject, *mut JClass, *const JMethodID, ...) -> *mut JObject,
    call_nonvirtual_object_method_v: extern "system" fn(*const JNIEnv, *mut JObject, *mut JClass, *const JMethodID, std::ffi::VaList) -> *mut JObject,
    call_nonvirtual_object_method_a: extern "system" fn(*const JNIEnv, *mut JObject, *mut JClass, *const JMethodID, *const JValue) -> *mut JObject,

    call_nonvirtual_boolean_method: extern "cdecl" fn(*const JNIEnv, *mut JObject, *mut JClass, *const JMethodID, ...) -> JBoolean,
    call_nonvirtual_boolean_method_v: extern "system" fn(*const JNIEnv, *mut JObject, *mut JClass, *const JMethodID, std::ffi::VaList) -> JBoolean,
    call_nonvirtual_boolean_method_a: extern "system" fn(*const JNIEnv, *mut JObject, *mut JClass, *const JMethodID, *const JValue) -> JBoolean,

    call_nonvirtual_byte_method: extern "cdecl" fn(*const JNIEnv, *mut JObject, *mut JClass, *const JMethodID, ...) -> JByte,
    call_nonvirtual_byte_method_v: extern "system" fn(*const JNIEnv, *mut JObject, *mut JClass, *const JMethodID, std::ffi::VaList) -> JByte,
    call_nonvirtual_byte_method_a: extern "system" fn(*const JNIEnv, *mut JObject, *mut JClass, *const JMethodID, *const JValue) -> JByte,

    call_nonvirtual_char_method: extern "cdecl" fn(*const JNIEnv, *mut JObject, *mut JClass, *const JMethodID, ...) -> JChar,
    call_nonvirtual_char_method_v: extern "system" fn(*const JNIEnv, *mut JObject, *mut JClass, *const JMethodID, std::ffi::VaList) -> JChar,
    call_nonvirtual_char_method_a: extern "system" fn(*const JNIEnv, *mut JObject, *mut JClass, *const JMethodID, *const JValue) -> JChar,

    call_nonvirtual_short_method: extern "cdecl" fn(*const JNIEnv, *mut JObject, *mut JClass, *const JMethodID, ...) -> JShort,
    call_nonvirtual_short_method_v: extern "system" fn(*const JNIEnv, *mut JObject, *mut JClass, *const JMethodID, std::ffi::VaList) -> JShort,
    call_nonvirtual_short_method_a: extern "system" fn(*const JNIEnv, *mut JObject, *mut JClass, *const JMethodID, *const JValue) -> JShort,

    call_nonvirtual_int_method: extern "cdecl" fn(*const JNIEnv, *mut JObject, *mut JClass, *const JMethodID, ...) -> JInt,
    call_nonvirtual_int_method_v: extern "system" fn(*const JNIEnv, *mut JObject, *mut JClass, *const JMethodID, std::ffi::VaList) -> JInt,
    call_nonvirtual_int_method_a: extern "system" fn(*const JNIEnv, *mut JObject, *mut JClass, *const JMethodID, *const JValue) -> JInt,

    call_nonvirtual_long_method: extern "cdecl" fn(*const JNIEnv, *mut JObject, *mut JClass, *const JMethodID, ...) -> JLong,
    call_nonvirtual_long_method_v: extern "system" fn(*const JNIEnv, *mut JObject, *mut JClass, *const JMethodID, std::ffi::VaList) -> JLong,
    call_nonvirtual_long_method_a: extern "system" fn(*const JNIEnv, *mut JObject, *mut JClass, *const JMethodID, *const JValue) -> JLong,

    call_nonvirtual_float_method: extern "cdecl" fn(*const JNIEnv, *mut JObject, *mut JClass, *const JMethodID, ...) -> JFloat,
    call_nonvirtual_float_method_v: extern "system" fn(*const JNIEnv, *mut JObject, *mut JClass, *const JMethodID, std::ffi::VaList) -> JFloat,
    call_nonvirtual_float_method_a: extern "system" fn(*const JNIEnv, *mut JObject, *mut JClass, *const JMethodID, *const JValue) -> JFloat,

    call_nonvirtual_double_method: extern "cdecl" fn(*const JNIEnv, *mut JObject, *mut JClass, *const JMethodID, ...) -> JDouble,
    call_nonvirtual_double_method_v: extern "system" fn(*const JNIEnv, *mut JObject, *mut JClass, *const JMethodID, std::ffi::VaList) -> JDouble,
    call_nonvirtual_double_method_a: extern "system" fn(*const JNIEnv, *mut JObject, *mut JClass, *const JMethodID, *const JValue) -> JDouble,

    call_nonvirtual_void_method: extern "cdecl" fn(*const JNIEnv, *mut JObject, *mut JClass, *const JMethodID, ...),
    call_nonvirtual_void_method_v: extern "system" fn(*const JNIEnv, *mut JObject, *mut JClass, *const JMethodID, std::ffi::VaList),
    call_nonvirtual_void_method_a: extern "system" fn(*const JNIEnv, *mut JObject, *mut JClass, *const JMethodID, *const JValue),

    get_field_id: extern "system" fn(*const JNIEnv, *mut JClass, *const i8, *const i8) -> *const JFieldID,

    get_object_field: extern "system" fn(*const JNIEnv, *mut JObject, *const JFieldID) -> *mut JObject,
    get_boolean_field: extern "system" fn(*const JNIEnv, *mut JObject, *const JFieldID) -> JBoolean,
    get_byte_field: extern "system" fn(*const JNIEnv, *mut JObject, *const JFieldID) -> JByte,
    get_char_field: extern "system" fn(*const JNIEnv, *mut JObject, *const JFieldID) -> JChar,
    get_short_field: extern "system" fn(*const JNIEnv, *mut JObject, *const JFieldID) -> JShort,
    get_int_field: extern "system" fn(*const JNIEnv, *mut JObject, *const JFieldID) -> JInt,
    get_long_field: extern "system" fn(*const JNIEnv, *mut JObject, *const JFieldID) -> JLong,
    get_float_field: extern "system" fn(*const JNIEnv, *mut JObject, *const JFieldID) -> JFloat,
    get_double_field: extern "system" fn(*const JNIEnv, *mut JObject, *const JFieldID) -> JDouble,

    set_object_field: extern "system" fn(*const JNIEnv, *mut JObject, *const JFieldID, *mut JObject),
    set_boolean_field: extern "system" fn(*const JNIEnv, *mut JObject, *const JFieldID, JBoolean),
    set_byte_field: extern "system" fn(*const JNIEnv, *mut JObject, *const JFieldID, JByte),
    set_char_field: extern "system" fn(*const JNIEnv, *mut JObject, *const JFieldID, JChar),
    set_short_field: extern "system" fn(*const JNIEnv, *mut JObject, *const JFieldID, JShort),
    set_int_field: extern "system" fn(*const JNIEnv, *mut JObject, *const JFieldID, JInt),
    set_long_field: extern "system" fn(*const JNIEnv, *mut JObject, *const JFieldID, JLong),
    set_float_field: extern "system" fn(*const JNIEnv, *mut JObject, *const JFieldID, JFloat),
    set_double_field: extern "system" fn(*const JNIEnv, *mut JObject, *const JFieldID, JDouble),

    get_static_method_id: extern "system" fn(*const JNIEnv, *mut JClass, *const i8, *const i8) -> *const JMethodID,

    call_static_object_method: extern "cdecl" fn(*const JNIEnv, *mut JClass, *const JMethodID, ...) -> *mut JObject,
    call_static_object_method_v: extern "system" fn(*const JNIEnv, *mut JClass, *const JMethodID, std::ffi::VaList) -> *mut JObject,
    call_static_object_method_a: extern "system" fn(*const JNIEnv, *mut JClass, *const JMethodID, *const JValue) -> *mut JObject,

    call_static_boolean_method: extern "cdecl" fn(*const JNIEnv, *mut JClass, *const JMethodID, ...) -> JBoolean,
    call_static_boolean_method_v: extern "system" fn(*const JNIEnv, *mut JClass, *const JMethodID, std::ffi::VaList) -> JBoolean,
    call_static_boolean_method_a: extern "system" fn(*const JNIEnv, *mut JClass, *const JMethodID, *const JValue) -> JBoolean,

    call_static_byte_method: extern "cdecl" fn(*const JNIEnv, *mut JClass, *const JMethodID, ...) -> JByte,
    call_static_byte_method_v: extern "system" fn(*const JNIEnv, *mut JClass, *const JMethodID, std::ffi::VaList) -> JByte,
    call_static_byte_method_a: extern "system" fn(*const JNIEnv, *mut JClass, *const JMethodID, *const JValue) -> JByte,

    call_static_char_method: extern "cdecl" fn(*const JNIEnv, *mut JClass, *const JMethodID, ...) -> JChar,
    call_static_char_method_v: extern "system" fn(*const JNIEnv, *mut JClass, *const JMethodID, std::ffi::VaList) -> JChar,
    call_static_char_method_a: extern "system" fn(*const JNIEnv, *mut JClass, *const JMethodID, *const JValue) -> JChar,

    call_static_short_method: extern "cdecl" fn(*const JNIEnv, *mut JClass, *const JMethodID, ...) -> JShort,
    call_static_short_method_v: extern "system" fn(*const JNIEnv, *mut JClass, *const JMethodID, std::ffi::VaList) -> JShort,
    call_static_short_method_a: extern "system" fn(*const JNIEnv, *mut JClass, *const JMethodID, *const JValue) -> JShort,

    call_static_int_method: extern "cdecl" fn(*const JNIEnv, *mut JClass, *const JMethodID, ...) -> JInt,
    call_static_int_method_v: extern "system" fn(*const JNIEnv, *mut JClass, *const JMethodID, std::ffi::VaList) -> JInt,
    call_static_int_method_a: extern "system" fn(*const JNIEnv, *mut JClass, *const JMethodID, *const JValue) -> JInt,

    call_static_long_method: extern "cdecl" fn(*const JNIEnv, *mut JClass, *const JMethodID, ...) -> JLong,
    call_static_long_method_v: extern "system" fn(*const JNIEnv, *mut JClass, *const JMethodID, std::ffi::VaList) -> JLong,
    call_static_long_method_a: extern "system" fn(*const JNIEnv, *mut JClass, *const JMethodID, *const JValue) -> JLong,

    call_static_float_method: extern "cdecl" fn(*const JNIEnv, *mut JClass, *const JMethodID, ...) -> JFloat,
    call_static_float_method_v: extern "system" fn(*const JNIEnv, *mut JClass, *const JMethodID, std::ffi::VaList) -> JFloat,
    call_static_float_method_a: extern "system" fn(*const JNIEnv, *mut JClass, *const JMethodID, *const JValue) -> JFloat,

    call_static_double_method: extern "cdecl" fn(*const JNIEnv, *mut JClass, *const JMethodID, ...) -> JDouble,
    call_static_double_method_v: extern "system" fn(*const JNIEnv, *mut JClass, *const JMethodID, std::ffi::VaList) -> JDouble,
    call_static_double_method_a: extern "system" fn(*const JNIEnv, *mut JClass, *const JMethodID, *const JValue) -> JDouble,

    call_static_void_method: extern "cdecl" fn(*const JNIEnv, *mut JClass, *const JMethodID, ...),
    call_static_void_method_v: extern "system" fn(*const JNIEnv, *mut JClass, *const JMethodID, std::ffi::VaList),
    call_static_void_method_a: extern "system" fn(*const JNIEnv, *mut JClass, *const JMethodID, *const JValue),

    get_static_field_id: extern "system" fn(*const JNIEnv, *mut JClass, *const i8, *const i8) -> *const JFieldID,

    get_static_object_field: extern "system" fn(*const JNIEnv, *mut JClass, *const JFieldID) -> *mut JObject,
    get_static_boolean_field: extern "system" fn(*const JNIEnv, *mut JClass, *const JFieldID) -> JBoolean,
    get_static_byte_field: extern "system" fn(*const JNIEnv, *mut JClass, *const JFieldID) -> JByte,
    get_static_char_field: extern "system" fn(*const JNIEnv, *mut JClass, *const JFieldID) -> JChar,
    get_static_short_field: extern "system" fn(*const JNIEnv, *mut JClass, *const JFieldID) -> JShort,
    get_static_int_field: extern "system" fn(*const JNIEnv, *mut JClass, *const JFieldID) -> JInt,
    get_static_long_field: extern "system" fn(*const JNIEnv, *mut JClass, *const JFieldID) -> JLong,
    get_static_float_field: extern "system" fn(*const JNIEnv, *mut JClass, *const JFieldID) -> JFloat,
    get_static_double_field: extern "system" fn(*const JNIEnv, *mut JClass, *const JFieldID) -> JDouble,

    set_static_object_field: extern "system" fn(*const JNIEnv, *mut JClass, *const JFieldID, *mut JObject),
    set_static_boolean_field: extern "system" fn(*const JNIEnv, *mut JClass, *const JFieldID, JBoolean),
    set_static_byte_field: extern "system" fn(*const JNIEnv, *mut JClass, *const JFieldID, JByte),
    set_static_char_field: extern "system" fn(*const JNIEnv, *mut JClass, *const JFieldID, JChar),
    set_static_short_field: extern "system" fn(*const JNIEnv, *mut JClass, *const JFieldID, JShort),
    set_static_int_field: extern "system" fn(*const JNIEnv, *mut JClass, *const JFieldID, JInt),
    set_static_long_field: extern "system" fn(*const JNIEnv, *mut JClass, *const JFieldID, JLong),
    set_static_float_field: extern "system" fn(*const JNIEnv, *mut JClass, *const JFieldID, JFloat),
    set_static_double_field: extern "system" fn(*const JNIEnv, *mut JClass, *const JFieldID, JDouble),

    new_string: extern "system" fn(*const JNIEnv, *const JChar, JSize) -> *mut JString,

    get_string_length: extern "system" fn(*const JNIEnv, *mut JString) -> JSize,
    get_string_chars: extern "system" fn(*const JNIEnv, *mut JString, *mut JBoolean) -> *const JChar,
    release_string_chars: extern "system" fn(*const JNIEnv, *mut JString, *const JChar),

    new_string_utf: extern "system" fn(*const JNIEnv, *const i8) -> *mut JString,

    get_string_utf_length: extern "system" fn(*const JNIEnv, *mut JString) -> JSize,
    get_string_utf_chars: extern "system" fn(*const JNIEnv, *mut JString, *mut JBoolean) -> *const i8,
    release_string_utf_chars: extern "system" fn(*const JNIEnv, *mut JString, *const i8),

    get_array_length: extern "system" fn(*const JNIEnv, *mut JArray) -> JSize,

    new_object_array: extern "system" fn(*const JNIEnv, JSize, *mut JClass, *mut JObject) -> *mut JObjectArray,
    get_object_array_element: extern "system" fn(*const JNIEnv, *mut JObjectArray, JSize) -> *mut JObject,
    set_object_array_element: extern "system" fn(*const JNIEnv, *mut JObjectArray, JSize, *mut JObject),

    new_boolean_array: extern "system" fn(*const JNIEnv, JSize) -> *mut JBooleanArray,
    new_byte_array: extern "system" fn(*const JNIEnv, JSize) -> *mut JByteArray,
    new_char_array: extern "system" fn(*const JNIEnv, JSize) -> *mut JCharArray,
    new_short_array: extern "system" fn(*const JNIEnv, JSize) -> *mut JShortArray,
    new_int_array: extern "system" fn(*const JNIEnv, JSize) -> *mut JIntArray,
    new_long_array: extern "system" fn(*const JNIEnv, JSize) -> *mut JLongArray,
    new_float_array: extern "system" fn(*const JNIEnv, JSize) -> *mut JFloatArray,
    new_double_array: extern "system" fn(*const JNIEnv, JSize) -> *mut JDoubleArray,

    get_boolean_array_elements: extern "system" fn(*const JNIEnv, *mut JBooleanArray, *mut JBoolean) -> *mut JBoolean,
    get_byte_array_elements: extern "system" fn(*const JNIEnv, *mut JByteArray, *mut JBoolean) -> *mut JByte,
    get_char_array_elements: extern "system" fn(*const JNIEnv, *mut JCharArray, *mut JBoolean) -> *mut JChar,
    get_short_array_elements: extern "system" fn(*const JNIEnv, *mut JShortArray, *mut JBoolean) -> *mut JShort,
    get_int_array_elements: extern "system" fn(*const JNIEnv, *mut JIntArray, *mut JBoolean) -> *mut JInt,
    get_long_array_elements: extern "system" fn(*const JNIEnv, *mut JLongArray, *mut JBoolean) -> *mut JLong,
    get_float_array_elements: extern "system" fn(*const JNIEnv, *mut JFloatArray, *mut JBoolean) -> *mut JFloat,
    get_double_array_elements: extern "system" fn(*const JNIEnv, *mut JDoubleArray, *mut JBoolean) -> *mut JDouble,

    release_boolean_array_elements: extern "system" fn(*const JNIEnv, *mut JBooleanArray, *mut JBoolean, JInt),
    release_byte_array_elements: extern "system" fn(*const JNIEnv, *mut JByteArray, *mut JByte, JInt),
    release_char_array_elements: extern "system" fn(*const JNIEnv, *mut JCharArray, *mut JChar, JInt),
    release_short_array_elements: extern "system" fn(*const JNIEnv, *mut JShortArray, *mut JShort, JInt),
    release_int_array_elements: extern "system" fn(*const JNIEnv, *mut JIntArray, *mut JInt, JInt),
    release_long_array_elements: extern "system" fn(*const JNIEnv, *mut JLongArray, *mut JLong, JInt),
    release_float_array_elements: extern "system" fn(*const JNIEnv, *mut JFloatArray, *mut JFloat, JInt),
    release_double_array_elements: extern "system" fn(*const JNIEnv, *mut JDoubleArray, *mut JDouble, JInt),

    get_boolean_array_region: extern "system" fn(*const JNIEnv, *mut JBooleanArray, JSize, JSize, *mut JBoolean),
    get_byte_array_region: extern "system" fn(*const JNIEnv, *mut JByteArray, JSize, JSize, *mut JByte),
    get_char_array_region: extern "system" fn(*const JNIEnv, *mut JCharArray, JSize, JSize, *mut JChar),
    get_short_array_region: extern "system" fn(*const JNIEnv, *mut JShortArray, JSize, JSize, *mut JShort),
    get_int_array_region: extern "system" fn(*const JNIEnv, *mut JIntArray, JSize, JSize, *mut JInt),
    get_long_array_region: extern "system" fn(*const JNIEnv, *mut JLongArray, JSize, JSize, *mut JLong),
    get_float_array_region: extern "system" fn(*const JNIEnv, *mut JFloatArray, JSize, JSize, *mut JFloat),
    get_double_array_region: extern "system" fn(*const JNIEnv, *mut JDoubleArray, JSize, JSize, *mut JDouble),

    set_boolean_array_region: extern "system" fn(*const JNIEnv, *mut JBooleanArray, JSize, JSize, *const JBoolean),
    set_byte_array_region: extern "system" fn(*const JNIEnv, *mut JByteArray, JSize, JSize, *const JByte),
    set_char_array_region: extern "system" fn(*const JNIEnv, *mut JCharArray, JSize, JSize, *const JChar),
    set_short_array_region: extern "system" fn(*const JNIEnv, *mut JShortArray, JSize, JSize, *const JShort),
    set_int_array_region: extern "system" fn(*const JNIEnv, *mut JIntArray, JSize, JSize, *const JInt),
    set_long_array_region: extern "system" fn(*const JNIEnv, *mut JLongArray, JSize, JSize, *const JLong),
    set_float_array_region: extern "system" fn(*const JNIEnv, *mut JFloatArray, JSize, JSize, *const JFloat),
    set_double_array_region: extern "system" fn(*const JNIEnv, *mut JDoubleArray, JSize, JSize, *const JDouble),

    register_natives: extern "system" fn(*const JNIEnv, *mut JClass, *const JNINativeMethod, JInt) -> JInt,
    unregister_natives: extern "system" fn(*const JNIEnv, *mut JClass) -> JInt,

    monitor_enter: extern "system" fn(*const JNIEnv, *mut JObject) -> JInt,
    monitor_exit: extern "system" fn(*const JNIEnv, *mut JObject) -> JInt,

    get_java_vm: extern "system" fn(*const JNIEnv, *mut *mut JavaVM) -> JInt,

    get_string_region: extern "system" fn(*const JNIEnv, *mut JString, JSize, JSize, *mut JChar),
    get_string_utf_region: extern "system" fn(*const JNIEnv, *mut JString, JSize, JSize, *mut i8),

    get_primitive_array_critical: extern "system" fn(*const JNIEnv, *mut JArray, *mut JBoolean) -> *mut c_void,
    release_primitive_array_critical: extern "system" fn(*const JNIEnv, *mut JArray, *mut c_void, JInt),

    get_string_critical: extern "system" fn(*const JNIEnv, *mut JString, *mut JBoolean) -> *const JChar,
    release_string_critical: extern "system" fn(*const JNIEnv, *mut JString, *const JChar),

    new_weak_global_ref: extern "system" fn(*const JNIEnv, *mut JObject) -> *mut JWeak,
    delete_weak_global_ref: extern "system" fn(*const JNIEnv, *mut JWeak),

    exception_check: extern "system" fn(*const JNIEnv) -> JBoolean,

    new_direct_byte_buffer: extern "system" fn(*const JNIEnv, *mut c_void, JLong) -> *mut JObject,
    get_direct_buffer_address: extern "system" fn(*const JNIEnv, *mut JObject) -> *mut c_void,
    get_direct_buffer_capacity: extern "system" fn(*const JNIEnv, *mut JObject) -> JLong,

    get_object_ref_type: extern "system" fn(*const JNIEnv, *mut JObject) -> JObjectRefType,

    get_module: extern "system" fn(*const JNIEnv, *mut JClass) -> *mut JObject,
}

impl JNIEnv {

    fn get_functions(&self) -> &JNINativeInterface {
        unsafe {
            self.functions.as_ref().expect("Invalid JNIEnv")
        }
    }

    /// Wrapper for env->GetVersion()
    pub fn get_version(&self) -> JInt {
        (self.get_functions().get_version)(self)
    }

    /// Wrapper for env->DefineClass(...)
    pub fn define_class(&self, name: *const i8, loader: *mut JObject, buffer: *const JByte, buf_len: JSize) -> *mut JClass {
        (self.get_functions().define_class)(self, name, loader, buffer, buf_len)
    }

    /// Wrapper for env->FindClass(...)
    pub fn find_class(&self, name: *const i8) -> *mut JClass {
        (self.get_functions().find_class)(self, name)
    }

    /// Wrapper for env->FromReflectedMethod(...)
    pub fn from_reflected_method(&self, method: *mut JObject) -> *const JMethodID {
        (self.get_functions().from_reflected_method)(self, method)
    }

    /// Wrapper for env->FromReflectedField(...)
    pub fn from_reflected_field(&self, field: *mut JObject) -> *const JFieldID {
        (self.get_functions().from_reflected_field)(self, field)
    }

    /// Wrapper for env->ToReflectedMethod(...)
    pub fn to_reflected_method(&self, cls: *mut JClass, id: *const JMethodID, is_static: JBoolean) -> *mut JObject {
        (self.get_functions().to_reflected_method)(self, cls, id, is_static)
    }

    /// Wrapper for env->GetSuperclass(...)
    pub fn get_superclass(&self, cls: *mut JClass) -> *mut JClass {
        (self.get_functions().get_superclass)(self, cls)
    }

    /// Wrapper for env->IsAssignableFrom(...)
    pub fn is_assignable_from(&self, cls1: *mut JClass, cls2: *mut JClass) -> JBoolean {
        (self.get_functions().is_assignable_from)(self, cls1, cls2)
    }

    /// Wrapper for env->ToReflectedField(...)
    pub fn to_reflected_field(&self, cls: *mut JClass, id: *const JFieldID, is_static: JBoolean) -> *mut JObject {
        (self.get_functions().to_reflected_field)(self, cls, id, is_static)
    }

    /// Wrapper for env->Throw(...)
    pub fn throw(&self, exception: *mut JThrowable) -> JInt {
        (self.get_functions().throw)(self, exception)
    }

    /// Wrapper for env->ThrowNew(...)
    pub fn throw_new(&self, cls: *mut JClass, msg: *const i8) -> JInt {
        (self.get_functions().throw_new)(self, cls, msg)
    }

    /// Wrapper for env->ExceptionOccurred()
    pub fn exception_occurred(&self) -> *mut JThrowable {
        (self.get_functions().exception_occurred)(self)
    }

    /// Wrapper for env->ExceptionDescribe()
    pub fn exception_describe(&self) {
        (self.get_functions().exception_describe)(self)
    }

    /// Wrapper for env->ExceptionClear()
    pub fn exception_clear(&self) {
        (self.get_functions().exception_clear)(self)
    }

    /// Wrapper for env->FatalError(...)
    pub fn fatal_error(&self, msg: *const i8) -> ! {
        (self.get_functions().fatal_error)(self, msg)
    }

    /// Wrapper for env->PushLocalFrame(...)
    pub fn push_local_frame(&self, capacity: JInt) -> JInt {
        (self.get_functions().push_local_frame)(self, capacity)
    }

    /// Wrapper for env->PopLocalFrame(...)
    pub fn pop_local_frame(&self, retval: *mut JObject) -> *mut JObject {
        (self.get_functions().pop_local_frame)(self, retval)
    }

    /// Wrapper for env->NewGlobalRef(...)
    pub fn new_global_ref(&self, obj: *mut JObject) -> *mut JObject {
        (self.get_functions().new_global_ref)(self, obj)
    }

    /// Wrapper for env->DeleteGlobalRef(...)
    pub fn delete_global_ref(&self, obj: *mut JObject) {
        (self.get_functions().delete_global_ref)(self, obj)
    }

    /// Wrapper for env->DeleteLocalRef(...)
    pub fn delete_local_ref(&self, obj: *mut JObject) {
        (self.get_functions().delete_local_ref)(self, obj)
    }

    /// Wrapper for env->IsSameObject(...)
    pub fn is_same_object(&self, obj1: *mut JObject, obj2: *mut JObject) -> JBoolean {
        (self.get_functions().is_same_object)(self, obj1, obj2)
    }

    /// Wrapper for env->NewLocalRef(...)
    pub fn new_local_ref(&self, obj: *mut JObject) -> *mut JObject {
        (self.get_functions().new_local_ref)(self, obj)
    }

    /// Wrapper for env->EnsureLocalCapacity(...)
    pub fn ensure_local_capacity(&self, capacity: JInt) -> JInt {
        (self.get_functions().ensure_local_capacity)(self, capacity)
    }

    /// Wrapper for env->AllocObject(...)
    pub fn alloc_object(&self, cls: *mut JClass) -> *mut JObject {
        (self.get_functions().alloc_object)(self, cls)
    }

    /// Default to the value array form, because rust doesn't support variadics
    /// Wrapper for env->NewObjectA(...)
    pub fn new_object(&self, cls: *mut JClass, id: *const JMethodID, args: *const JValue) -> *mut JObject {
        (self.get_functions().new_object_a)(self, cls, id, args)
    }

    /// Wrapper for env->NewObjectV(...)
    pub fn new_object_v(&self, cls: *mut JClass, id: *const JMethodID, args: std::ffi::VaList) -> *mut JObject {
        (self.get_functions().new_object_v)(self, cls, id, args)
    }

    /// Wrapper for env->GetObjectClass(...)
    pub fn get_object_class(&self, obj: *mut JObject) -> *mut JClass {
        (self.get_functions().get_object_class)(self, obj)
    }

    /// Wrapper for env->IsInstanceOf(...)
    pub fn is_instance_of(&self, obj: *mut JObject, cls: *mut JClass) -> JBoolean {
        (self.get_functions().is_instance_of)(self, obj, cls)
    }

    /// Wrapper for env->GetMethodId(...)
    pub fn get_method_id(&self, cls: *mut JClass, name: *const i8, sig: *const i8) -> *const JMethodID {
        (self.get_functions().get_method_id)(self, cls, name, sig)
    }

    /// Default to the value array form, becauses rust doesn't support variadics
    /// Wrapper for env->CallObjectMethodA(...)
    pub fn call_object_method(&self, obj: *mut JObject, id: *const JMethodID, args: *const JValue) -> *mut JObject {
        (self.get_functions().call_object_method_a)(self, obj, id, args)
    }

    /// Wrapper for env->CallObjectMethodV(...)
    pub fn call_object_method_v(&self, obj: *mut JObject, id: *const JMethodID, args: std::ffi::VaList) -> *mut JObject {
        (self.get_functions().call_object_method_v)(self, obj, id, args)
    }

    /// Default to the value array form, becauses rust doesn't support variadics
    /// Wrapper for env->CallBooleanMethodA(...)
    pub fn call_boolean_method(&self, obj: *mut JObject, id: *const JMethodID, args: *const JValue) -> JBoolean {
        (self.get_functions().call_boolean_method_a)(self, obj, id, args)
    }

    /// Wrapper for env->CallBooleanMethodV(...)
    pub fn call_boolean_method_v(&self, obj: *mut JObject, id: *const JMethodID, args: std::ffi::VaList) -> JBoolean {
        (self.get_functions().call_boolean_method_v)(self, obj, id, args)
    }

    /// Default to the value array form, becauses rust doesn't support variadics
    /// Wrapper for env->CallByteMethodA(...)
    pub fn call_byte_method(&self, obj: *mut JObject, id: *const JMethodID, args: *const JValue) -> JByte {
        (self.get_functions().call_byte_method_a)(self, obj, id, args)
    }

    /// Wrapper for env->CallByteMethodV(...)
    pub fn call_byte_method_v(&self, obj: *mut JObject, id: *const JMethodID, args: std::ffi::VaList) -> JByte {
        (self.get_functions().call_byte_method_v)(self, obj, id, args)
    }

    /// Default to the value array form, becauses rust doesn't support variadics
    /// Wrapper for env->CallCharMethodA(...)
    pub fn call_char_method(&self, obj: *mut JObject, id: *const JMethodID, args: *const JValue) -> JChar {
        (self.get_functions().call_char_method_a)(self, obj, id, args)
    }

    /// Wrapper for env->CallCharMethodV(...)
    pub fn call_char_method_v(&self, obj: *mut JObject, id: *const JMethodID, args: std::ffi::VaList) -> JChar {
        (self.get_functions().call_char_method_v)(self, obj, id, args)
    }

    /// Default to the value array form, becauses rust doesn't support variadics
    /// Wrapper for env->CallShortMethodA(...)
    pub fn call_short_method(&self, obj: *mut JObject, id: *const JMethodID, args: *const JValue) -> JShort {
        (self.get_functions().call_short_method_a)(self, obj, id, args)
    }

    /// Wrapper for env->CallShortMethodV(...)
    pub fn call_short_method_v(&self, obj: *mut JObject, id: *const JMethodID, args: std::ffi::VaList) -> JShort {
        (self.get_functions().call_short_method_v)(self, obj, id, args)
    }

    /// Default to the value array form, becauses rust doesn't support variadics
    /// Wrapper for env->CallIntMethodA(...)
    pub fn call_int_method(&self, obj: *mut JObject, id: *const JMethodID, args: *const JValue) -> JInt {
        (self.get_functions().call_int_method_a)(self, obj, id, args)
    }

    /// Wrapper for env->CallIntMethodV(...)
    pub fn call_int_method_v(&self, obj: *mut JObject, id: *const JMethodID, args: std::ffi::VaList) -> JInt {
        (self.get_functions().call_int_method_v)(self, obj, id, args)
    }

    /// Default to the value array form, becauses rust doesn't support variadics
    /// Wrapper for env->CallLongMethodA(...)
    pub fn call_long_method(&self, obj: *mut JObject, id: *const JMethodID, args: *const JValue) -> JLong {
        (self.get_functions().call_long_method_a)(self, obj, id, args)
    }

    /// Wrapper for env->CallLongMethodV(...)
    pub fn call_long_method_v(&self, obj: *mut JObject, id: *const JMethodID, args: std::ffi::VaList) -> JLong {
        (self.get_functions().call_long_method_v)(self, obj, id, args)
    }

    /// Default to the value array form, becauses rust doesn't support variadics
    /// Wrapper for env->CallFloatMethodA(...)
    pub fn call_float_method(&self, obj: *mut JObject, id: *const JMethodID, args: *const JValue) -> JFloat {
        (self.get_functions().call_float_method_a)(self, obj, id, args)
    }

    /// Wrapper for env->CallFloatMethodV(...)
    pub fn call_float_method_v(&self, obj: *mut JObject, id: *const JMethodID, args: std::ffi::VaList) -> JFloat {
        (self.get_functions().call_float_method_v)(self, obj, id, args)
    }

    /// Default to the value array form, becauses rust doesn't support variadics
    /// Wrapper for env->CallDoubleMethodA(...)
    pub fn call_double_method(&self, obj: *mut JObject, id: *const JMethodID, args: *const JValue) -> JDouble {
        (self.get_functions().call_double_method_a)(self, obj, id, args)
    }

    /// Wrapper for env->CallDoubleMethodV(...)
    pub fn call_double_method_v(&self, obj: *mut JObject, id: *const JMethodID, args: std::ffi::VaList) -> JDouble {
        (self.get_functions().call_double_method_v)(self, obj, id, args)
    }

    /// Default to the value array form, becauses rust doesn't support variadics
    /// Wrapper for env->CallVoidMethodA(...)
    pub fn call_void_method(&self, obj: *mut JObject, id: *const JMethodID, args: *const JValue) {
        (self.get_functions().call_void_method_a)(self, obj, id, args)
    }

    /// Wrapper for env->CallVoidMethodV(...)
    pub fn call_void_method_v(&self, obj: *mut JObject, id: *const JMethodID, args: std::ffi::VaList) {
        (self.get_functions().call_void_method_v)(self, obj, id, args)
    }

    /// Default to the value array form, becauses rust doesn't support variadics
    /// Wrapper for env->CallNonvirtualObjectMethodA(...)
    pub fn call_nonvirtual_object_method(&self, obj: *mut JObject, cls: *mut JClass, id: *const JMethodID, args: *const JValue) -> *mut JObject {
        (self.get_functions().call_nonvirtual_object_method_a)(self, obj, cls, id, args)
    }

    /// Wrapper for env->CallNonvirtualObjectMethodV(...)
    pub fn call_nonvirtual_object_method_v(&self, obj: *mut JObject, cls: *mut JClass, id: *const JMethodID, args: std::ffi::VaList) -> *mut JObject {
        (self.get_functions().call_nonvirtual_object_method_v)(self, obj, cls, id, args)
    }

    /// Default to the value array form, becauses rust doesn't support variadics
    /// Wrapper for env->CallNonvirtualBooleanMethodA(...)
    pub fn call_nonvirtual_boolean_method(&self, obj: *mut JObject, cls: *mut JClass, id: *const JMethodID, args: *const JValue) -> JBoolean {
        (self.get_functions().call_nonvirtual_boolean_method_a)(self, obj, cls, id, args)
    }

    /// Wrapper for env->CallNonvirtualBooleanMethodV(...)
    pub fn call_nonvirtual_boolean_method_v(&self, obj: *mut JObject, cls: *mut JClass, id: *const JMethodID, args: std::ffi::VaList) -> JBoolean {
        (self.get_functions().call_nonvirtual_boolean_method_v)(self, obj, cls, id, args)
    }

    /// Default to the value array form, becauses rust doesn't support variadics
    /// /// Wrapper for env->CallNonvirtualByteMethodA(...)
    pub fn call_nonvirtual_byte_method(&self, obj: *mut JObject, cls: *mut JClass, id: *const JMethodID, args: *const JValue) -> JByte {
        (self.get_functions().call_nonvirtual_byte_method_a)(self, obj, cls, id, args)
    }

    /// Wrapper for env->CallNonvirtualByteMethodV(...)
    pub fn call_nonvirtual_byte_method_v(&self, obj: *mut JObject, cls: *mut JClass, id: *const JMethodID, args: std::ffi::VaList) -> JByte {
        (self.get_functions().call_nonvirtual_byte_method_v)(self, obj, cls, id, args)
    }

    /// Default to the value array form, becauses rust doesn't support variadics
    /// Wrapper for env->CallNonvirtualCharMethodA(...)
    pub fn call_nonvirtual_char_method(&self, obj: *mut JObject, cls: *mut JClass, id: *const JMethodID, args: *const JValue) -> JChar {
        (self.get_functions().call_nonvirtual_char_method_a)(self, obj, cls, id, args)
    }

    /// Wrapper for env->CallNonvirtualCharMethodV(...)
    pub fn call_nonvirtual_char_method_v(&self, obj: *mut JObject, cls: *mut JClass, id: *const JMethodID, args: std::ffi::VaList) -> JChar {
        (self.get_functions().call_nonvirtual_char_method_v)(self, obj, cls, id, args)
    }

    /// Default to the value array form, becauses rust doesn't support variadics
    /// Wrapper for env->CallNonvirtualShortMethodA(...)
    pub fn call_nonvirtual_short_method(&self, obj: *mut JObject, cls: *mut JClass, id: *const JMethodID, args: *const JValue) -> JShort {
        (self.get_functions().call_nonvirtual_short_method_a)(self, obj, cls, id, args)
    }

    /// Wrapper for env->CallNonvirtualShortMethodV(...)
    pub fn call_nonvirtual_short_method_v(&self, obj: *mut JObject, cls: *mut JClass, id: *const JMethodID, args: std::ffi::VaList) -> JShort {
        (self.get_functions().call_nonvirtual_short_method_v)(self, obj, cls, id, args)
    }

    /// Default to the value array form, becauses rust doesn't support variadics
    /// Wrapper for env->CallNonvirtualIntMethodA(...)
    pub fn call_nonvirtual_int_method(&self, obj: *mut JObject, cls: *mut JClass, id: *const JMethodID, args: *const JValue) -> JInt {
        (self.get_functions().call_nonvirtual_int_method_a)(self, obj, cls, id, args)
    }

    /// Wrapper for env->CallNonvirtualIntMethodV(...)
    pub fn call_nonvirtual_int_method_v(&self, obj: *mut JObject, cls: *mut JClass, id: *const JMethodID, args: std::ffi::VaList) -> JInt {
        (self.get_functions().call_nonvirtual_int_method_v)(self, obj, cls, id, args)
    }

    /// Default to the value array form, becauses rust doesn't support variadics
    /// Wrapper for env->CallNonvirtualLongMethodA(...)
    pub fn call_nonvirtual_long_method(&self, obj: *mut JObject, cls: *mut JClass, id: *const JMethodID, args: *const JValue) -> JLong {
        (self.get_functions().call_nonvirtual_long_method_a)(self, obj, cls, id, args)
    }

    /// Wrapper for env->CallNonvirtualLongMethodV(...)
    pub fn call_nonvirtual_long_method_v(&self, obj: *mut JObject, cls: *mut JClass, id: *const JMethodID, args: std::ffi::VaList) -> JLong {
        (self.get_functions().call_nonvirtual_long_method_v)(self, obj, cls, id, args)
    }

    /// Default to the value array form, becauses rust doesn't support variadics
    /// Wrapper for env->CallNonvirtualFloatMethodA(...)
    pub fn call_nonvirtual_float_method(&self, obj: *mut JObject, cls: *mut JClass, id: *const JMethodID, args: *const JValue) -> JFloat {
        (self.get_functions().call_nonvirtual_float_method_a)(self, obj, cls, id, args)
    }

    /// Wrapper for env->CallNonvirtualFloatMethodV(...)
    pub fn call_nonvirtual_float_method_v(&self, obj: *mut JObject, cls: *mut JClass, id: *const JMethodID, args: std::ffi::VaList) -> JFloat {
        (self.get_functions().call_nonvirtual_float_method_v)(self, obj, cls, id, args)
    }

    /// Default to the value array form, becauses rust doesn't support variadics
    /// Wrapper for env->CallNonvirtualDoubleMethodA(...)
    pub fn call_nonvirtual_double_method(&self, obj: *mut JObject, cls: *mut JClass, id: *const JMethodID, args: *const JValue) -> JDouble {
        (self.get_functions().call_nonvirtual_double_method_a)(self, obj, cls, id, args)
    }

    /// Wrapper for env->CallNonvirtualDoubleMethodV(...)
    pub fn call_nonvirtual_double_method_v(&self, obj: *mut JObject, cls: *mut JClass, id: *const JMethodID, args: std::ffi::VaList) -> JDouble {
        (self.get_functions().call_nonvirtual_double_method_v)(self, obj, cls, id, args)
    }

    /// Default to the value array form, becauses rust doesn't support variadics
    /// Wrapper for env->CallNonvirtualVoidMethodA(...)
    pub fn call_nonvirtual_void_method(&self, obj: *mut JObject, cls: *mut JClass, id: *const JMethodID, args: *const JValue) {
        (self.get_functions().call_nonvirtual_void_method_a)(self, obj, cls, id, args)
    }

    /// Wrapper for env->CallNonvirtualVoidMethodV(...)
    pub fn call_nonvirtual_void_method_v(&self, obj: *mut JObject, cls: *mut JClass, id: *const JMethodID, args: std::ffi::VaList) {
        (self.get_functions().call_nonvirtual_void_method_v)(self, obj, cls, id, args)
    }

    /// Wrapper for env->GetFieldId(...)
    pub fn get_field_id(&self, cls: *mut JClass, name: *const i8, sig: *const i8) -> *const JFieldID {
        (self.get_functions().get_field_id)(self, cls, name, sig)
    }

    /// Wrapper for env->GetObjectField(...)
    pub fn get_object_field(&self, obj: *mut JObject, id: *const JFieldID) -> *mut JObject {
        (self.get_functions().get_object_field)(self, obj, id)
    }

    /// Wrapper for env->GetBooleanField(...)
    pub fn get_boolean_field(&self, obj: *mut JObject, id: *const JFieldID) -> JBoolean {
        (self.get_functions().get_boolean_field)(self, obj, id)
    }

    /// Wrapper for env->GetByteField(...)
    pub fn get_byte_field(&self, obj: *mut JObject, id: *const JFieldID) -> JByte {
        (self.get_functions().get_byte_field)(self, obj, id)
    }

    /// Wrapper for env->GetCharField(...)
    pub fn get_char_field(&self, obj: *mut JObject, id: *const JFieldID) -> JChar {
        (self.get_functions().get_char_field)(self, obj, id)
    }

    /// Wrapper for env->GetShortField(...)
    pub fn get_short_field(&self, obj: *mut JObject, id: *const JFieldID) -> JShort {
        (self.get_functions().get_short_field)(self, obj, id)
    }

    /// Wrapper for env->GetIntField(...)
    pub fn get_int_field(&self, obj: *mut JObject, id: *const JFieldID) -> JInt {
        (self.get_functions().get_int_field)(self, obj, id)
    }

    /// Wrapper for env->GetLongField(...)
    pub fn get_long_field(&self, obj: *mut JObject, id: *const JFieldID) -> JLong {
        (self.get_functions().get_long_field)(self, obj, id)
    }

    /// Wrapper for env->GetFloatField(...)
    pub fn get_float_field(&self, obj: *mut JObject, id: *const JFieldID) -> JFloat {
        (self.get_functions().get_float_field)(self, obj, id)
    }

    /// Wrapper for env->GetDoubleField(...)
    pub fn get_double_field(&self, obj: *mut JObject, id: *const JFieldID) -> JDouble {
        (self.get_functions().get_double_field)(self, obj, id)
    }

    /// Wrapper for env->GetObjectField(...)
    pub fn set_object_field(&self, obj: *mut JObject, id: *const JFieldID, val: *mut JObject) {
        (self.get_functions().set_object_field)(self, obj, id, val)
    }

    /// Wrapper for env->SetBooleanField(...)
    pub fn set_boolean_field(&self, obj: *mut JObject, id: *const JFieldID, val: JBoolean) {
        (self.get_functions().set_boolean_field)(self, obj, id, val)
    }

    /// Wrapper for env->SetByteField(...)
    pub fn set_byte_field(&self, obj: *mut JObject, id: *const JFieldID, val: JByte) {
        (self.get_functions().set_byte_field)(self, obj, id, val)
    }

    /// Wrapper for env->SetCharField(...)
    pub fn set_char_field(&self, obj: *mut JObject, id: *const JFieldID, val: JChar) {
        (self.get_functions().set_char_field)(self, obj, id, val)
    }

    /// Wrapper for env->SetShortField(...)
    pub fn set_short_field(&self, obj: *mut JObject, id: *const JFieldID, val: JShort) {
        (self.get_functions().set_short_field)(self, obj, id, val)
    }

    /// Wrapper for env->SetIntField(...)
    pub fn set_int_field(&self, obj: *mut JObject, id: *const JFieldID, val: JInt) {
        (self.get_functions().set_int_field)(self, obj, id, val)
    }

    /// Wrapper for env->SetLongField(...)
    pub fn set_long_field(&self, obj: *mut JObject, id: *const JFieldID, val: JLong) {
        (self.get_functions().set_long_field)(self, obj, id, val)
    }

    /// Wrapper for env->SetFloatField(...)
    pub fn set_float_field(&self, obj: *mut JObject, id: *const JFieldID, val: JFloat) {
        (self.get_functions().set_float_field)(self, obj, id, val)
    }

    /// Wrapper for env->SetDoubleField(...)
    pub fn set_double_field(&self, obj: *mut JObject, id: *const JFieldID, val: JDouble) {
        (self.get_functions().set_double_field)(self, obj, id, val)
    }

    /// Wrapper for env->GetStaticMethodId(...)
    pub fn get_static_method_id(&self, cls: *mut JClass, name: *const i8, sig: *const i8) -> *const JMethodID {
        (self.get_functions().get_static_method_id)(self, cls, name, sig)
    }

    /// Default to the value array form, becauses rust doesn't support variadics
    /// Wrapper for env->CallStaticObjectMethodA(...)
    pub fn call_static_object_method(&self, cls: *mut JClass, id: *const JMethodID, args: *const JValue) -> *mut JObject {
        (self.get_functions().call_static_object_method_a)(self, cls, id, args)
    }

    /// Wrapper for env->CallStaticObjectMethodV(...)
    pub fn call_static_object_method_v(&self, cls: *mut JClass, id: *const JMethodID, args: std::ffi::VaList) -> *mut JObject {
        (self.get_functions().call_static_object_method_v)(self, cls, id, args)
    }

    /// Default to the value array form, becauses rust doesn't support variadics
    /// Wrapper for env->CallStaticBooleanMethodA(...)
    pub fn call_static_boolean_method(&self, cls: *mut JClass, id: *const JMethodID, args: *const JValue) -> JBoolean {
        (self.get_functions().call_static_boolean_method_a)(self, cls, id, args)
    }

    /// Wrapper for env->CallStaticBooleanMethodV(...)
    pub fn call_static_boolean_method_v(&self, cls: *mut JClass, id: *const JMethodID, args: std::ffi::VaList) -> JBoolean {
        (self.get_functions().call_static_boolean_method_v)(self, cls, id, args)
    }

    /// Default to the value array form, becauses rust doesn't support variadics
    /// Wrapper for env->CallStaticByteMethodA(...)
    pub fn call_static_byte_method(&self, cls: *mut JClass, id: *const JMethodID, args: *const JValue) -> JByte {
        (self.get_functions().call_static_byte_method_a)(self, cls, id, args)
    }

    /// Wrapper for env->CallStaticByteMethodV(...)
    pub fn call_static_byte_method_v(&self, cls: *mut JClass, id: *const JMethodID, args: std::ffi::VaList) -> JByte {
        (self.get_functions().call_static_byte_method_v)(self, cls, id, args)
    }

    /// Default to the value array form, becauses rust doesn't support variadics
    /// Wrapper for env->CallStaticCharMethodA(...)
    pub fn call_static_char_method(&self, cls: *mut JClass, id: *const JMethodID, args: *const JValue) -> JChar {
        (self.get_functions().call_static_char_method_a)(self, cls, id, args)
    }

    /// Wrapper for env->CallStaticCharMethodV(...)
    pub fn call_static_char_method_v(&self, cls: *mut JClass, id: *const JMethodID, args: std::ffi::VaList) -> JChar {
        (self.get_functions().call_static_char_method_v)(self, cls, id, args)
    }

    /// Default to the value array form, becauses rust doesn't support variadics
    /// Wrapper for env->CallStaticShortMethodA(...)
    pub fn call_static_short_method(&self, cls: *mut JClass, id: *const JMethodID, args: *const JValue) -> JShort {
        (self.get_functions().call_static_short_method_a)(self, cls, id, args)
    }

    /// Wrapper for env->CallStaticShortMethodV(...)
    pub fn call_static_short_method_v(&self, cls: *mut JClass, id: *const JMethodID, args: std::ffi::VaList) -> JShort {
        (self.get_functions().call_static_short_method_v)(self, cls, id, args)
    }

    /// Default to the value array form, becauses rust doesn't support variadics
    /// Wrapper for env->CallStaticIntMethodA(...)
    pub fn call_static_int_method(&self, cls: *mut JClass, id: *const JMethodID, args: *const JValue) -> JInt {
        (self.get_functions().call_static_int_method_a)(self, cls, id, args)
    }

    /// Wrapper for env->CallStaticIntMethodV(...)
    pub fn call_static_int_method_v(&self, cls: *mut JClass, id: *const JMethodID, args: std::ffi::VaList) -> JInt {
        (self.get_functions().call_static_int_method_v)(self, cls, id, args)
    }

    /// Default to the value array form, becauses rust doesn't support variadics
    /// Wrapper for env->CallStaticLongMethodA(...)
    pub fn call_static_long_method(&self, cls: *mut JClass, id: *const JMethodID, args: *const JValue) -> JLong {
        (self.get_functions().call_static_long_method_a)(self, cls, id, args)
    }

    /// Wrapper for env->CallStaticLongMethodV(...)
    pub fn call_static_long_method_v(&self, cls: *mut JClass, id: *const JMethodID, args: std::ffi::VaList) -> JLong {
        (self.get_functions().call_static_long_method_v)(self, cls, id, args)
    }

    /// Default to the value array form, becauses rust doesn't support variadics
    /// Wrapper for env->CallStaticFloatMethodA(...)
    pub fn call_static_float_method(&self, cls: *mut JClass, id: *const JMethodID, args: *const JValue) -> JFloat {
        (self.get_functions().call_static_float_method_a)(self, cls, id, args)
    }

    /// Wrapper for env->CallStaticFloatMethodV(...)
    pub fn call_static_float_method_v(&self, cls: *mut JClass, id: *const JMethodID, args: std::ffi::VaList) -> JFloat {
        (self.get_functions().call_static_float_method_v)(self, cls, id, args)
    }

    /// Default to the value array form, becauses rust doesn't support variadics
    /// Wrapper for env->CallStaticDoubleMethodA(...)
    pub fn call_static_double_method(&self, cls: *mut JClass, id: *const JMethodID, args: *const JValue) -> JDouble {
        (self.get_functions().call_static_double_method_a)(self, cls, id, args)
    }

    /// Wrapper for env->CallStaticDoubleMethodV(...)
    pub fn call_static_double_method_v(&self, cls: *mut JClass, id: *const JMethodID, args: std::ffi::VaList) -> JDouble {
        (self.get_functions().call_static_double_method_v)(self, cls, id, args)
    }

    /// Default to the value array form, becauses rust doesn't support variadics
    /// Wrapper for env->CallStaticVoidMethodA(...)
    pub fn call_static_void_method(&self, cls: *mut JClass, id: *const JMethodID, args: *const JValue) {
        (self.get_functions().call_static_void_method_a)(self, cls, id, args)
    }

    /// Wrapper for env->CallStaticVoidMethodV(...)
    pub fn call_static_void_method_v(&self, cls: *mut JClass, id: *const JMethodID, args: std::ffi::VaList) {
        (self.get_functions().call_static_void_method_v)(self, cls, id, args)
    }

    /// Wrapper for env->GetStaticFieldId(...)
    pub fn get_static_field_id(&self, cls: *mut JClass, name: *const i8, sig: *const i8) -> *const JFieldID {
        (self.get_functions().get_static_field_id)(self, cls, name, sig)
    }

    /// Wrapper for env->GetStaticObjectField(...)
    pub fn get_static_object_field(&self, cls: *mut JClass, id: *const JFieldID) -> *mut JObject {
        (self.get_functions().get_static_object_field)(self, cls, id)
    }

    /// Wrapper for env->GetStaticBooleanField(...)
    pub fn get_static_boolean_field(&self, cls: *mut JClass, id: *const JFieldID) -> JBoolean {
        (self.get_functions().get_static_boolean_field)(self, cls, id)
    }

    /// Wrapper for env->GetStaticByteField(...)
    pub fn get_static_byte_field(&self, cls: *mut JClass, id: *const JFieldID) -> JByte {
        (self.get_functions().get_static_byte_field)(self, cls, id)
    }

    /// Wrapper for env->GetStaticCharField(...)
    pub fn get_static_char_field(&self, cls: *mut JClass, id: *const JFieldID) -> JChar {
        (self.get_functions().get_static_char_field)(self, cls, id)
    }

    /// Wrapper for env->GetStaticShortField(...)
    pub fn get_static_short_field(&self, cls: *mut JClass, id: *const JFieldID) -> JShort {
        (self.get_functions().get_static_short_field)(self, cls, id)
    }

    /// Wrapper for env->GetStaticIntField(...)
    pub fn get_static_int_field(&self, cls: *mut JClass, id: *const JFieldID) -> JInt {
        (self.get_functions().get_static_int_field)(self, cls, id)
    }

    /// Wrapper for env->GetStaticLongField(...)
    pub fn get_static_long_field(&self, cls: *mut JClass, id: *const JFieldID) -> JLong {
        (self.get_functions().get_static_long_field)(self, cls, id)
    }

    /// Wrapper for env->GetStaticFloatField(...)
    pub fn get_static_float_field(&self, cls: *mut JClass, id: *const JFieldID) -> JFloat {
        (self.get_functions().get_static_float_field)(self, cls, id)
    }

    /// Wrapper for env->GetStaticDoubleField(...)
    pub fn get_static_double_field(&self, cls: *mut JClass, id: *const JFieldID) -> JDouble {
        (self.get_functions().get_static_double_field)(self, cls, id)
    }

    /// Wrapper for env->GetStaticObjectField(...)
    pub fn set_static_object_field(&self, cls: *mut JClass, id: *const JFieldID, val: *mut JObject) {
        (self.get_functions().set_static_object_field)(self, cls, id, val)
    }

    /// Wrapper for env->SetStaticBooleanField(...)
    pub fn set_static_boolean_field(&self, cls: *mut JClass, id: *const JFieldID, val: JBoolean) {
        (self.get_functions().set_static_boolean_field)(self, cls, id, val)
    }

    /// Wrapper for env->SetStaticByteField(...)
    pub fn set_static_byte_field(&self, cls: *mut JClass, id: *const JFieldID, val: JByte) {
        (self.get_functions().set_static_byte_field)(self, cls, id, val)
    }

    /// Wrapper for env->SetStaticCharField(...)
    pub fn set_static_char_field(&self, cls: *mut JClass, id: *const JFieldID, val: JChar) {
        (self.get_functions().set_static_char_field)(self, cls, id, val)
    }

    /// Wrapper for env->SetStaticShortField(...)
    pub fn set_static_short_field(&self, cls: *mut JClass, id: *const JFieldID, val: JShort) {
        (self.get_functions().set_static_short_field)(self, cls, id, val)
    }

    /// Wrapper for env->SetStaticIntField(...)
    pub fn set_static_int_field(&self, cls: *mut JClass, id: *const JFieldID, val: JInt) {
        (self.get_functions().set_static_int_field)(self, cls, id, val)
    }

    /// Wrapper for env->SetStaticLongField(...)
    pub fn set_static_long_field(&self, cls: *mut JClass, id: *const JFieldID, val: JLong) {
        (self.get_functions().set_static_long_field)(self, cls, id, val)
    }

    /// Wrapper for env->SetStaticFloatField(...)
    pub fn set_static_float_field(&self, cls: *mut JClass, id: *const JFieldID, val: JFloat) {
        (self.get_functions().set_static_float_field)(self, cls, id, val)
    }

    /// Wrapper for env->SetStaticDoubleField(...)
    pub fn set_static_double_field(&self, cls: *mut JClass, id: *const JFieldID, val: JDouble) {
        (self.get_functions().set_static_double_field)(self, cls, id, val)
    }

    /// Wrapper for env->NewString(...)
    pub fn new_string(&self, chars: *const JChar, len: JSize) -> *mut JString {
        (self.get_functions().new_string)(self, chars, len)
    }

    /// Wrapper for env->GetStringLength(...)
    pub fn get_string_length(&self, str: *mut JString) -> JSize {
        (self.get_functions().get_string_length)(self, str)
    }

    /// Wrapper for env->GetStringChars(...)
    pub fn get_string_chars(&self, str: *mut JString, is_copy: *mut JBoolean) -> *const JChar {
        (self.get_functions().get_string_chars)(self, str, is_copy)
    }

    /// Wrapper for env->ReleaseStringChars(...)
    pub fn release_string_chars(&self, str: *mut JString, chars: *const JChar) {
        (self.get_functions().release_string_chars)(self, str, chars)
    }

    /// Wrapper for env->NewStringUTF(...)
    pub fn new_string_utf(&self, chars: *const i8) -> *mut JString {
        (self.get_functions().new_string_utf)(self, chars)
    }

    /// Wrapper for env->GetStringUTFLength(...)
    pub fn get_string_utf_length(&self, str: *mut JString) -> JSize {
        (self.get_functions().get_string_utf_length)(self, str)
    }

    /// Wrapper for env->GetStringUTFChars(...)
    pub fn get_string_utf_chars(&self, str: *mut JString, is_copy: *mut JBoolean) -> *const i8 {
        (self.get_functions().get_string_utf_chars)(self, str, is_copy)
    }

    /// Wrapper for env->ReleaseStringUTFChars(...)
    pub fn release_string_utf_chars(&self, str: *mut JString, chars: *const i8) {
        (self.get_functions().release_string_utf_chars)(self, str, chars)
    }

    /// Wrapper for env->GetArrayLength(...)
    pub fn get_array_length(&self, arr: *mut JArray) -> JSize {
        (self.get_functions().get_array_length)(self, arr)
    }

    /// Wrapper for env->NewObjectArray(...)
    pub fn new_object_array(&self, len: JSize, cls: *mut JClass, fill: *mut JObject) -> *mut JObjectArray {
        (self.get_functions().new_object_array)(self, len, cls, fill)
    }

    /// Wrapper for env->GetObjectArrayElement(...)
    pub fn get_object_array_element(&self, arr: *mut JObjectArray, index: JSize) -> *mut JObject {
        (self.get_functions().get_object_array_element)(self, arr, index)
    }

    /// Wrapper for env->SetObjectArrayElement(...)
    pub fn set_object_array_element(&self, arr: *mut JObjectArray, index: JSize, val: *mut JObject) {
        (self.get_functions().set_object_array_element)(self, arr, index, val)
    }

    /// Wrapper for env->NewBooleanArray(...)
    pub fn new_boolean_array(&self, len: JSize) -> *mut JBooleanArray {
        (self.get_functions().new_boolean_array)(self, len)
    }

    /// Wrapper for env->NewByteArray(...)
    pub fn new_byte_array(&self, len: JSize) -> *mut JByteArray {
        (self.get_functions().new_byte_array)(self, len)
    }

    /// Wrapper for env->NewCharArray(...)
    pub fn new_char_array(&self, len: JSize) -> *mut JCharArray {
        (self.get_functions().new_char_array)(self, len)
    }

    /// Wrapper for env->NewShortArray(...)
    pub fn new_short_array(&self, len: JSize) -> *mut JShortArray {
        (self.get_functions().new_short_array)(self, len)
    }

    /// Wrapper for env->NewIntArray(...)
    pub fn new_int_array(&self, len: JSize) -> *mut JIntArray {
        (self.get_functions().new_int_array)(self, len)
    }

    /// Wrapper for env->NewLongArray(...)
    pub fn new_long_array(&self, len: JSize) -> *mut JLongArray {
        (self.get_functions().new_long_array)(self, len)
    }

    /// Wrapper for env->NewFloatArray(...)
    pub fn new_float_array(&self, len: JSize) -> *mut JFloatArray {
        (self.get_functions().new_float_array)(self, len)
    }

    /// Wrapper for env->NewDoubleArray(...)
    pub fn new_double_array(&self, len: JSize) -> *mut JDoubleArray {
        (self.get_functions().new_double_array)(self, len)
    }

    /// Wrapper for env->GetBooleanArrayElements(...)
    pub fn get_boolean_array_elements(&self, arr: *mut JBooleanArray, is_copy: *mut JBoolean) -> *mut JBoolean {
        (self.get_functions().get_boolean_array_elements)(self, arr, is_copy)
    }

    /// Wrapper for env->GetByteArrayElements(...)
    pub fn get_byte_array_elements(&self, arr: *mut JByteArray, is_copy: *mut JBoolean) -> *mut JByte {
        (self.get_functions().get_byte_array_elements)(self, arr, is_copy)
    }

    /// Wrapper for env->GetCharArrayElements(...)
    pub fn get_char_array_elements(&self, arr: *mut JCharArray, is_copy: *mut JBoolean) -> *mut JChar {
        (self.get_functions().get_char_array_elements)(self, arr, is_copy)
    }

    /// Wrapper for env->GetShortArrayElements(...)
    pub fn get_short_array_elements(&self, arr: *mut JShortArray, is_copy: *mut JBoolean) -> *mut JShort {
        (self.get_functions().get_short_array_elements)(self, arr, is_copy)
    }

    /// Wrapper for env->GetIntArrayElements(...)
    pub fn get_int_array_elements(&self, arr: *mut JIntArray, is_copy: *mut JBoolean) -> *mut JInt {
        (self.get_functions().get_int_array_elements)(self, arr, is_copy)
    }

    /// Wrapper for env->GetLongArrayElements(...)
    pub fn get_long_array_elements(&self, arr: *mut JLongArray, is_copy: *mut JBoolean) -> *mut JLong {
        (self.get_functions().get_long_array_elements)(self, arr, is_copy)
    }

    /// Wrapper for env->GetFloatArrayElements(...)
    pub fn get_float_array_elements(&self, arr: *mut JFloatArray, is_copy: *mut JBoolean) -> *mut JFloat {
        (self.get_functions().get_float_array_elements)(self, arr, is_copy)
    }

    /// Wrapper for env->GetDoubleArrayElements(...)
    pub fn get_double_array_elements(&self, arr: *mut JDoubleArray, is_copy: *mut JBoolean) -> *mut JDouble {
        (self.get_functions().get_double_array_elements)(self, arr, is_copy)
    }

    /// Wrapper for env->ReleaseBooleanArrayElements(...)
    pub fn release_boolean_array_elements(&self, arr: *mut JBooleanArray, elems: *mut JBoolean, mode: JInt) {
        (self.get_functions().release_boolean_array_elements)(self, arr, elems, mode)
    }

    /// Wrapper for env->ReleaseByteArrayElements(...)
    pub fn release_byte_array_elements(&self, arr: *mut JByteArray, elems: *mut JByte, mode: JInt) {
        (self.get_functions().release_byte_array_elements)(self, arr, elems, mode)
    }

    /// Wrapper for env->ReleaseCharArrayElements(...)
    pub fn release_char_array_elements(&self, arr: *mut JCharArray, elems: *mut JChar, mode: JInt) {
        (self.get_functions().release_char_array_elements)(self, arr, elems, mode)
    }

    /// Wrapper for env->ReleaseShortArrayElements(...)
    pub fn release_short_array_elements(&self, arr: *mut JShortArray, elems: *mut JShort, mode: JInt) {
        (self.get_functions().release_short_array_elements)(self, arr, elems, mode)
    }

    /// Wrapper for env->ReleaseIntArrayElements(...)
    pub fn release_int_array_elements(&self, arr: *mut JIntArray, elems: *mut JInt, mode: JInt) {
        (self.get_functions().release_int_array_elements)(self, arr, elems, mode)
    }

    /// Wrapper for env->ReleaseLongArrayElements(...)
    pub fn release_long_array_elements(&self, arr: *mut JLongArray, elems: *mut JLong, mode: JInt) {
        (self.get_functions().release_long_array_elements)(self, arr, elems, mode)
    }

    /// Wrapper for env->ReleaseFloatArrayElements(...)
    pub fn release_float_array_elements(&self, arr: *mut JFloatArray, elems: *mut JFloat, mode: JInt) {
        (self.get_functions().release_float_array_elements)(self, arr, elems, mode)
    }

    /// Wrapper for env->ReleaseDoubleArrayElements(...)
    pub fn release_double_array_elements(&self, arr: *mut JDoubleArray, elems: *mut JDouble, mode: JInt) {
        (self.get_functions().release_double_array_elements)(self, arr, elems, mode)
    }

    /// Wrapper for env->GetBooleanArrayRegion(...)
    pub fn get_boolean_array_region(&self, arr: *mut JBooleanArray, start: JSize, len: JSize, buffer: *mut JBoolean) {
        (self.get_functions().get_boolean_array_region)(self, arr, start, len, buffer)
    }

    /// Wrapper for env->GetByteArrayRegion(...)
    pub fn get_byte_array_region(&self, arr: *mut JByteArray, start: JSize, len: JSize, buffer: *mut JByte) {
        (self.get_functions().get_byte_array_region)(self, arr, start, len, buffer)
    }

    /// Wrapper for env->GetCharArrayRegion(...)
    pub fn get_char_array_region(&self, arr: *mut JCharArray, start: JSize, len: JSize, buffer: *mut JChar) {
        (self.get_functions().get_char_array_region)(self, arr, start, len, buffer)
    }

    /// Wrapper for env->GetShortArrayRegion(...)
    pub fn get_short_array_region(&self, arr: *mut JShortArray, start: JSize, len: JSize, buffer: *mut JShort) {
        (self.get_functions().get_short_array_region)(self, arr, start, len, buffer)
    }

    /// Wrapper for env->GetIntArrayRegion(...)
    pub fn get_int_array_region(&self, arr: *mut JIntArray, start: JSize, len: JSize, buffer: *mut JInt) {
        (self.get_functions().get_int_array_region)(self, arr, start, len, buffer)
    }

    /// Wrapper for env->GetLongArrayRegion(...)
    pub fn get_long_array_region(&self, arr: *mut JLongArray, start: JSize, len: JSize, buffer: *mut JLong) {
        (self.get_functions().get_long_array_region)(self, arr, start, len, buffer)
    }

    /// Wrapper for env->GetFloatArrayRegion(...)
    pub fn get_float_array_region(&self, arr: *mut JFloatArray, start: JSize, len: JSize, buffer: *mut JFloat) {
        (self.get_functions().get_float_array_region)(self, arr, start, len, buffer)
    }

    /// Wrapper for env->GetDoubleArrayRegion(...)
    pub fn get_double_array_region(&self, arr: *mut JDoubleArray, start: JSize, len: JSize, buffer: *mut JDouble) {
        (self.get_functions().get_double_array_region)(self, arr, start, len, buffer)
    }

    /// Wrapper for env->SetBooleanArrayRegion(...)
    pub fn set_boolean_array_region(&self, arr: *mut JBooleanArray, start: JSize, len: JSize, buffer: *const JBoolean) {
        (self.get_functions().set_boolean_array_region)(self, arr, start, len, buffer)
    }

    /// Wrapper for env->SetByteArrayRegion(...)
    pub fn set_byte_array_region(&self, arr: *mut JByteArray, start: JSize, len: JSize, buffer: *const JByte) {
        (self.get_functions().set_byte_array_region)(self, arr, start, len, buffer)
    }

    /// Wrapper for env->SetCharArrayRegion(...)
    pub fn set_char_array_region(&self, arr: *mut JCharArray, start: JSize, len: JSize, buffer: *const JChar) {
        (self.get_functions().set_char_array_region)(self, arr, start, len, buffer)
    }

    /// Wrapper for env->SetShortArrayRegion(...)
    pub fn set_short_array_region(&self, arr: *mut JShortArray, start: JSize, len: JSize, buffer: *const JShort) {
        (self.get_functions().set_short_array_region)(self, arr, start, len, buffer)
    }

    /// Wrapper for env->SetIntArrayRegion(...)
    pub fn set_int_array_region(&self, arr: *mut JIntArray, start: JSize, len: JSize, buffer: *const JInt) {
        (self.get_functions().set_int_array_region)(self, arr, start, len, buffer)
    }

    /// Wrapper for env->SetLongArrayRegion(...)
    pub fn set_long_array_region(&self, arr: *mut JLongArray, start: JSize, len: JSize, buffer: *const JLong) {
        (self.get_functions().set_long_array_region)(self, arr, start, len, buffer)
    }

    /// Wrapper for env->SetFloatArrayRegion(...)
    pub fn set_float_array_region(&self, arr: *mut JFloatArray, start: JSize, len: JSize, buffer: *const JFloat) {
        (self.get_functions().set_float_array_region)(self, arr, start, len, buffer)
    }

    /// Wrapper for env->SetDoubleArrayRegion(...)
    pub fn set_double_array_region(&self, arr: *mut JDoubleArray, start: JSize, len: JSize, buffer: *const JDouble) {
        (self.get_functions().set_double_array_region)(self, arr, start, len, buffer)
    }

    /// Wrapper for env->RegisterNatives(...)
    pub fn register_natives(&self, cls: *mut JClass, methods: *const JNINativeMethod, num_methods: JInt) -> JInt {
        (self.get_functions().register_natives)(self, cls, methods, num_methods)
    }

    /// Wrapper for env->UnregisterNatives(...)
    pub fn unregister_natives(&self, cls: *mut JClass) -> JInt {
        (self.get_functions().unregister_natives)(self, cls)
    }

    /// Wrapper for env->MonitorEnter(...)
    pub fn monitor_enter(&self, obj: *mut JObject) -> JInt {
        (self.get_functions().monitor_enter)(self, obj)
    }

    /// Wrapper for env->MonitorExit(...)
    pub fn monitor_exit(&self, obj: *mut JObject) -> JInt {
        (self.get_functions().monitor_exit)(self, obj)
    }

    /// Wrapper for env->GetJavaVM(...)
    pub fn get_java_vm(&self, jvm_loc: *mut *mut JavaVM) -> JInt {
        (self.get_functions().get_java_vm)(self, jvm_loc)
    }

    /// Wrapper for env->GetStringRegion(...)
    pub fn get_string_region(&self, str: *mut JString, start: JSize, len: JSize, buffer: *mut JChar) {
        (self.get_functions().get_string_region)(self, str, start, len, buffer)
    }

    /// Wrapper for env->GetStringUTFRegion(...)
    pub fn get_string_utf_region(&self, str: *mut JString, start: JSize, len: JSize, buffer: *mut i8) {
        (self.get_functions().get_string_utf_region)(self, str, start, len, buffer)
    }

    /// Wrapper for env->GetPrimitiveArrayCritical(...)
    pub fn get_primitive_array_critical(&self, arr: *mut JArray, is_copy: *mut JBoolean) -> *mut c_void {
        (self.get_functions().get_primitive_array_critical)(self, arr, is_copy)
    }

    /// Wrapper for env->ReleasePrimitiveArrayCritical(...)
    pub fn release_primitive_array_critical(&self, arr: *mut JArray, elems: *mut c_void, mode: JInt) {
        (self.get_functions().release_primitive_array_critical)(self, arr, elems, mode)
    }

    /// Wrapper for env->GetStringCritical(...)
    pub fn get_string_critical(&self, str: *mut JString, is_copy: *mut JBoolean) -> *const JChar {
        (self.get_functions().get_string_critical)(self, str, is_copy)
    }

    /// Wrapper for env->ReleaseStringCritical(...)
    pub fn release_string_critical(&self, str: *mut JString, chars: *const JChar) {
        (self.get_functions().release_string_critical)(self, str, chars)
    }

    /// Wrapper for env->NewWeakGlobalRef(...)
    pub fn new_weak_global_ref(&self, obj: *mut JObject) -> *mut JWeak {
        (self.get_functions().new_weak_global_ref)(self, obj)
    }

    /// Wrapper for env->DeleteWeakGlobalRef(...)
    pub fn delete_weak_global_ref(&self, obj: *mut JWeak) {
        (self.get_functions().delete_weak_global_ref)(self, obj)
    }

    /// Wrapper for env->ExceptionCheck(...)
    pub fn exception_check(&self) -> JBoolean {
        (self.get_functions().exception_check)(self)
    }

    /// Wrapper for env->NewDirectByteBuffer(...)
    pub fn new_direct_byte_buffer(&self, buffer: *mut c_void, capacity: JLong) -> *mut JObject {
        (self.get_functions().new_direct_byte_buffer)(self, buffer, capacity)
    }

    /// Wrapper for env->GetDirectBufferAddress(...)
    pub fn get_direct_buffer_address(&self, buffer: *mut JObject) -> *mut c_void {
        (self.get_functions().get_direct_buffer_address)(self, buffer)
    }

    /// Wrapper for env->GetDirectBufferCapacity(...)
    pub fn get_direct_buffer_capacity(&self, buffer: *mut JObject) -> JLong {
        (self.get_functions().get_direct_buffer_capacity)(self, buffer)
    }

    /// Wrapper for env->GetObjectRefType(...)
    pub fn get_object_ref_type(&self, obj: *mut JObject) -> JObjectRefType {
        (self.get_functions().get_object_ref_type)(self, obj)
    }

    /// Wrapper for env->GetModule(...)
    pub fn get_module(&self, cls: *mut JClass) -> *mut JObject {
        (self.get_functions().get_module)(self, cls)
    }
}
