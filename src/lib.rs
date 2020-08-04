
//!
//! rust_jni is a library for writing of JNI compatible libraries, in rust. It uses higher-level
//! abstractions along with a #[java] macro to allow the easy writing of safe, sound code.
//! It trades off some efficiency and control in the name of greater soundness and better error
//! handling.
//!
//! # Example of a #[java] function
//!
//! ```
//! use rust_jni::*;
//! use rust_jni_proc::java;
//!
//! #[java(class = "com.foo.Bar")]
//! fn FuncName(env: &JNIEnv, this: &JObject, arg1: &JObject, arg2: JInt) -> &JClass {
//!     let class = env.get_object_class(arg1).expect("Couldn't get object class");
//!
//!     &class
//! }
//! ```
//!
//! # Soundness
//!
//! - #[java] functions must take non-native types by reference. (And native types by value)
//!   Why? The pointers passed are local, and thus are guaranteed to live
//!   for the length of that call, but not any longer. (Or until [delete_local_ref] is called,
//!   which is why it's unsafe)
//! - Returned objects are references, if local. They're stored in the env, deleted them if the ref
//!   is deleted.
//! - Casts transmute safely because all backing pointers are the same, the JVM just calls them
//!   different things for type safety (And the casts done are either checked or verified safe)
//! - [borrow_ptr] is unsafe because external systems may break the pointer promises. All
//!   places labeled with 'Internal pointer use' as their safety are places where we are
//!   using the pointers in JNI-sound ways.
//!
//! Macro Rules:
//! - Must take environment as first param
//! - Must take non-native types by ref, native by ref
//! - Must return native by value, non-native by ref
//! - Must include `class = ""`, may either use actual name or `name = ""`

#![allow(unused)]

#![feature(c_variadic)]
#![feature(never_type)]
#![feature(alloc_layout_extra)]

pub mod error;
pub mod ffi;

pub mod vm;
pub mod env;
pub mod types;
pub mod mangling;
pub mod macros;

pub use error::{Error, Result};

pub use types::*;
pub use vm::JavaVM;
pub use env::JNIEnv;
