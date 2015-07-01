//! JNI constants

use types::*;

#[cfg(target_arch = "x86_64")]
pub const JNI_NULL:Jpointer     = 0u64;

pub static JNI_FALSE:Jboolean   = 0;
pub static JNI_TRUE:Jboolean    = 1;

pub static JNI_OK:Jint          = 0;
pub static JNI_ERR:Jint         = -1;
pub static JNI_EDETACHED:Jint   = -2;
pub static JNI_EVERSION:Jint    = -3;
pub static JNI_ENOMEM:Jint      = -4;
pub static JNI_EEXIST:Jint      = -5;
pub static JNI_EINVAL:Jint      = -6;

pub static JNI_COMMIT:Jint      = 1;
pub static JNI_ABORT:Jint       = 2;

pub static JNI_VERSION_1_6:Jint = 0x00010006;
