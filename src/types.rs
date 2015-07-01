//! JNI types

use libc::{c_long, c_longlong};

#[cfg(target_arch = "x86_64")]
pub type Jint = c_long;

#[cfg(target_arch = "x86_64")]
pub type Jlong = c_longlong;

pub type Jbyte = i8;

#[cfg(target_arch = "x86_64")]
pub type Jpointer = u64;

// JNI Types

pub type Jboolean = u8;
pub type Jchar = u16;
pub type Jshort = i16;
pub type Jfloat = f32;
pub type Jdouble = f64;
pub type Jsize = Jint;

pub type Jobject = Jpointer;

pub type Jclass = Jobject;
pub type Jthrowable = Jobject;
pub type Jstring = Jobject;
pub type Jarray = Jobject;

pub type JbooleanArray = Jarray;
pub type JbyteArray = Jarray;
pub type JcharArray = Jarray;
pub type JshortArray = Jarray;
pub type JintArray = Jarray;
pub type JlongArray = Jarray;
pub type JfloatArray = Jarray;
pub type JdoubleArray = Jarray;
pub type JobjectArray = Jarray;

pub type Jtweak = Jobject;

pub type Jvalue = Jobject; // union in C/C++

pub type JfieldID = Jpointer;
pub type JmethodID = Jpointer;

#[repr(C)]
pub enum JobjectRefType {
    JNIInvalidRefType           = 0,
    JNILocalRefType             = 1,
    JNIGlobalRefType            = 2,
    JNIWeakGlobalRefType        = 3 
}

pub fn is_null(p:Jpointer) -> bool {
    p == 0u64
}
