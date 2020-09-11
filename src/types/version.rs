//!
//! Module containing an enum representing possible JNI versions
//!

use crate::ffi::constants;

///
/// An enum containing variants representing all the supported JNI versions
///
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd)]
pub enum JNIVersion {
    /// JNI 1.1
    Ver11,
    /// JNI 1.2
    Ver12,
    /// JNI 1.4
    Ver14,
    /// JNI 1.6
    Ver16,
    /// JNI 1.8
    Ver18,
    /// JNI 9
    Ver9,
    /// JNI 10
    Ver10
}

impl From<i32> for JNIVersion {
    fn from(val: i32) -> Self {
        match val {
            constants::JNI_VERSION_1_1 => {
                JNIVersion::Ver11
            }
            constants::JNI_VERSION_1_2 => {
                JNIVersion::Ver12
            }
            constants::JNI_VERSION_1_4 => {
                JNIVersion::Ver14
            }
            constants::JNI_VERSION_1_6 => {
                JNIVersion::Ver16
            }
            constants::JNI_VERSION_1_8 => {
                JNIVersion::Ver18
            }
            constants::JNI_VERSION_9 => {
                JNIVersion::Ver9
            }
            constants::JNI_VERSION_10 => {
                JNIVersion::Ver10
            }
            _ => {
                panic!("Invalid value for JNIVersion")
            }
        }
    }
}

impl From<JNIVersion> for i32 {
    fn from(val: JNIVersion) -> Self {
        match val {
            JNIVersion::Ver11 => {
                constants::JNI_VERSION_1_1
            }
            JNIVersion::Ver12 => {
                constants::JNI_VERSION_1_2
            }
            JNIVersion::Ver14 => {
                constants::JNI_VERSION_1_4
            }
            JNIVersion::Ver16 => {
                constants::JNI_VERSION_1_6
            }
            JNIVersion::Ver18 => {
                constants::JNI_VERSION_1_8
            }
            JNIVersion::Ver9 => {
                constants::JNI_VERSION_9
            }
            JNIVersion::Ver10 => {
                constants::JNI_VERSION_10
            }
        }
    }
}
