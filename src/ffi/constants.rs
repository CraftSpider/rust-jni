//!
//! Module containing definitions of constants used in the JNI.
//!

/// Version value for Java 1.1
pub const JNI_VERSION_1_1: i32 = 0x00010001;
/// Version value for Java 1.2
pub const JNI_VERSION_1_2: i32 = 0x00010002;
/// Version value for Java 1.4
pub const JNI_VERSION_1_4: i32 = 0x00010004;
/// Version value for Java 1.6
pub const JNI_VERSION_1_6: i32 = 0x00010006;
/// Version value for Java 1.8
pub const JNI_VERSION_1_8: i32 = 0x00010008;
/// Version value for Java 9
pub const JNI_VERSION_9: i32 = 0x00090000;
/// Version value for Java 10
pub const JNI_VERSION_10: i32 = 0x000a0000;

/// Value passed for FALSE in numeric contexts
pub const JNI_FALSE: u8 = 0;
/// Value passed for TRUE in numeric contexts
pub const JNI_TRUE: u8 = 1;

/// Return for a successful operation
pub const JNI_OK: i32 = 0;
/// Return for a generic error
pub const JNI_ERR: i32 = -1;
/// Return for if the current thread isn't attached
pub const JNI_EDETACHED: i32 = -2;
/// Return for if the JNI has an invalid version
pub const JNI_EVERSION: i32 = -3;
/// Return for if the JVM has run out of memory
pub const JNI_ENOMEM: i32 = -4;
/// Return for if a JVM already exists on this thread
pub const JNI_EEXIST: i32 = -5;
/// Return for if an invalid operation occurred
pub const JNI_EINVAL: i32 = -6;

/// Value for committing an array region change
pub const JNI_COMMIT: i32 = 1;
/// Value for aborting an array region change
pub const JNI_ABORT: i32 = 2;
