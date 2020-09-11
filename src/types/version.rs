
use crate::ffi::constants;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd)]
pub enum JNIVersion {
    Ver11,
    Ver12,
    Ver14,
    Ver16,
    Ver18,
    Ver9,
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
