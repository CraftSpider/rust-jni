//!
//! A module containing implementations of FFI types and extern functions that will be linked
//! at compile-time
//!

// Public modules

pub mod native_interface;
pub mod invoke_interface;
pub mod types;
pub mod link;
pub mod constants;

// Public re-exports

pub use native_interface::JNINativeInterface;
pub use invoke_interface::JNIInvokeInterface;
pub use types::*;
pub use link::*;
