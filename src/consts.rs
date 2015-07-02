//! JNI constants

use types::*;

#[cfg(target_arch = "x86_64")]
pub const JNI_NULL:Jpointer     = 0u64;

pub static JNI_FALSE:Jboolean   = 0;
pub static JNI_TRUE:Jboolean 	= 1;

pub const JNI_OK:Jint           = 0;
pub const JNI_ERR:Jint          = -1;
pub const JNI_EDETACHED:Jint    = -2;
pub const JNI_EVERSION:Jint     = -3;
pub const JNI_ENOMEM:Jint       = -4;
pub const JNI_EEXIST:Jint       = -5;
pub const JNI_EINVAL:Jint       = -6;

pub const JNI_COMMIT:Jint       = 1;
pub const JNI_ABORT:Jint        = 2;

pub const JNI_VERSION_1_6:Jint  = 0x00010006;
