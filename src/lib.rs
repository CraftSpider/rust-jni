
//!
//! rust_jni is a library for the writing of JNI compatible libraries, in rust. It uses higher-level
//! abstractions along with the [rust_jni_proc::java] macro to allow the easy writing of safe, sound
//! code. It trades off some efficiency and control in the name of greater soundness and better
//! error handling.
//!
//! # Type Signatures
//!
//!
//!
//! # Example of a #[rust_jni_proc::java] function
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
//! - `#[java]` functions must take all types by value, with non-static lifetimes. Why? The pointers
//!   passed are local, and thus are guaranteed to live for the length of that call, but not any
//!   longer. (Or until [env::JNIEnv::delete_local_ref] is called, which is why it's unsafe)
//! - Casts transmute safely because all backing pointers are the same, the JVM just calls them
//!   different things for type safety (And the casts done are either checked or verified safe)
//! - [types::JObject::borrow_ptr] is unsafe because external systems may break the pointer
//!   promises. All places labeled with 'Internal pointer use' as their safety are places where we
//!   are using the pointers in JNI-sound ways.
//!
//! Macro Rules:
//! - Must take environment as first param
//! - Must take all types by value
//! - Must return by value
//! - Must include `class = ""`, may either use actual name or `name = ""`

// #![allow(unused)]

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
