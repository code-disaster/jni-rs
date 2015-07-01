use libc::{c_char, c_void};
use libc::funcs::c95::stdlib::free;
use std::ffi::CString;
use std::mem::transmute;
use std::ptr;

//use consts::*;
use types::*;

//#[repr(C)]
//struct JNINativeMethod {
//  name: *mut c_char,
//  signature: *mut c_char,
//  fn_ptr: *mut u8
//}

#[repr(C)]
#[allow(dead_code)]
struct JNINativeInterface {
    reserved0: *mut c_void,
    reserved1: *mut c_void,
    reserved2: *mut c_void,
    reserved3: *mut c_void,

    get_version: fn(env:*mut JNIEnv) -> Jint,

    define_class: fn() -> *mut u8, // not implemented

    find_class: extern fn(env:*mut JNIEnv, name:*const c_char) -> Jclass,

    from_reflected_method: fn() -> *mut u8, // not implemented
    from_reflected_field: fn() -> *mut u8, // not implemented
    to_reflected_method: fn() -> *mut u8, // not implemented
    get_super_class: fn() -> *mut u8, // not implemented
    is_assignable_from: fn() -> *mut u8, // not implemented
    to_reflected_field: fn() -> *mut u8, // not implemented
    
    throw: fn() -> *mut u8, // not implemented
    throw_new: fn() -> *mut u8, // not implemented
    exception_occured: fn(env:*mut JNIEnv) -> Jthrowable,
    exception_describe: fn(env:*mut JNIEnv),
    exception_clear: fn(env:*mut JNIEnv),
    fatal_error: fn() -> *mut u8, // not implemented

    push_local_frame: fn() -> *mut u8, // not implemented
    pop_local_frame: fn() -> *mut u8, // not implemented

    new_global_ref: fn() -> *mut u8, // not implemented
    delete_global_ref: fn() -> *mut u8, // not implemented
    delete_local_ref: fn() -> *mut u8, // not implemented
    is_same_object: fn() -> *mut u8, // not implemented
    new_local_ref: fn() -> *mut u8, // not implemented
    ensure_local_capacity: fn() -> *mut u8, // not implemented

    alloc_object: fn() -> *mut u8, // not implemented
    new_object: fn() -> *mut u8, // not implemented,
    new_object_v: fn() -> *mut u8, // not implemented
    new_object_a: fn(env:*mut JNIEnv, clazz:Jclass, method:JmethodID, args:*const Jvalue) -> Jobject,

    get_object_class: fn() -> *mut u8, // not implemented
    is_instance_of: fn() -> *mut u8, // not implemented

    get_method_id: fn(env:*mut JNIEnv, clazz:Jclass, name:*const c_char, sig:*const c_char) -> JmethodID,

    call_object_method: fn() -> *mut u8, // not implemented
    call_object_method_v: fn() -> *mut u8, // not implemented
    call_object_method_a: fn(env:*mut JNIEnv, obj:Jobject, method:JmethodID, args:*const Jvalue) -> Jobject,

    call_boolean_method: fn() -> *mut u8, // not implemented
    call_boolean_method_v: fn() -> *mut u8, // not implemented
    call_boolean_method_a: fn() -> *mut u8, // not implemented

    call_byte_method: fn() -> *mut u8, // not implemented
    call_byte_method_v: fn() -> *mut u8, // not implemented
    call_byte_method_a: fn() -> *mut u8, // not implemented
    
    call_char_method: fn() -> *mut u8, // not implemented
    call_char_method_v: fn() -> *mut u8, // not implemented
    call_char_method_a: fn() -> *mut u8, // not implemented

    call_short_method: fn() -> *mut u8, // not implemented
    call_short_method_v: fn() -> *mut u8, // not implemented
    call_short_method_a: fn() -> *mut u8, // not implemented

    call_int_method: fn() -> *mut u8, // not implemented
    call_int_method_v: fn() -> *mut u8, // not implemented
    call_int_method_a: fn() -> *mut u8, // not implemented

    call_long_method: fn() -> *mut u8, // not implemented
    call_long_method_v: fn() -> *mut u8, // not implemented
    call_long_method_a: fn() -> *mut u8, // not implemented

    call_float_method: fn() -> *mut u8, // not implemented
    call_float_method_v: fn() -> *mut u8, // not implemented
    call_float_method_a: fn() -> *mut u8, // not implemented

    call_double_method: fn() -> *mut u8, // not implemented
    call_double_method_v: fn() -> *mut u8, // not implemented
    call_double_method_a: fn() -> *mut u8, // not implemented

    call_void_method: fn() -> *mut u8, // not implemented
    call_void_method_v: fn() -> *mut u8, // not implemented
    call_void_method_a: fn(env:*mut JNIEnv, obj:Jobject, method:JmethodID, args:*const Jvalue),

    call_nonvirtual_object_method: fn() -> *mut u8, // not implemented
    call_nonvirtual_object_method_v: fn() -> *mut u8, // not implemented
    call_nonvirtual_object_method_a: fn() -> *mut u8, // not implemented

    call_nonvirtual_boolean_method: fn() -> *mut u8, // not implemented
    call_nonvirtual_boolean_method_v: fn() -> *mut u8, // not implemented
    call_nonvirtual_boolean_method_a: fn() -> *mut u8, // not implemented

    call_nonvirtual_byte_method: fn() -> *mut u8, // not implemented
    call_nonvirtual_byte_method_v: fn() -> *mut u8, // not implemented
    call_nonvirtual_byte_method_a: fn() -> *mut u8, // not implemented
    
    call_nonvirtual_char_method: fn() -> *mut u8, // not implemented
    call_nonvirtual_char_method_v: fn() -> *mut u8, // not implemented
    call_nonvirtual_char_method_a: fn() -> *mut u8, // not implemented

    call_nonvirtual_short_method: fn() -> *mut u8, // not implemented
    call_nonvirtual_short_method_v: fn() -> *mut u8, // not implemented
    call_nonvirtual_short_method_a: fn() -> *mut u8, // not implemented

    call_nonvirtual_int_method: fn() -> *mut u8, // not implemented
    call_nonvirtual_int_method_v: fn() -> *mut u8, // not implemented
    call_nonvirtual_int_method_a: fn() -> *mut u8, // not implemented

    call_nonvirtual_long_method: fn() -> *mut u8, // not implemented
    call_nonvirtual_long_method_v: fn() -> *mut u8, // not implemented
    call_nonvirtual_long_method_a: fn() -> *mut u8, // not implemented

    call_nonvirtual_float_method: fn() -> *mut u8, // not implemented
    call_nonvirtual_float_method_v: fn() -> *mut u8, // not implemented
    call_nonvirtual_float_method_a: fn() -> *mut u8, // not implemented

    call_nonvirtual_double_method: fn() -> *mut u8, // not implemented
    call_nonvirtual_double_method_v: fn() -> *mut u8, // not implemented
    call_nonvirtual_double_method_a: fn() -> *mut u8, // not implemented

    call_nonvirtual_void_method: fn() -> *mut u8, // not implemented
    call_nonvirtual_void_method_v: fn() -> *mut u8, // not implemented
    call_nonvirtual_void_method_a: fn() -> *mut u8, // not implemented

    get_field_id: fn() -> *mut u8, // not implemented

    get_object_field: fn() -> *mut u8, // not implemented
    get_boolean_field: fn() -> *mut u8, // not implemented
    get_byte_field: fn() -> *mut u8, // not implemented
    get_char_field: fn() -> *mut u8, // not implemented
    get_short_field: fn() -> *mut u8, // not implemented
    get_int_field: fn() -> *mut u8, // not implemented
    get_long_field: fn() -> *mut u8, // not implemented
    get_float_field: fn() -> *mut u8, // not implemented
    get_double_field: fn() -> *mut u8, // not implemented

    set_object_field: fn() -> *mut u8, // not implemented
    set_boolean_field: fn() -> *mut u8, // not implemented
    set_byte_field: fn() -> *mut u8, // not implemented
    set_char_field: fn() -> *mut u8, // not implemented
    set_short_field: fn() -> *mut u8, // not implemented
    set_int_field: fn() -> *mut u8, // not implemented
    set_long_field: fn() -> *mut u8, // not implemented
    set_float_field: fn() -> *mut u8, // not implemented
    set_double_field: fn() -> *mut u8, // not implemented

    get_static_method_id: fn(env:*mut JNIEnv, clazz:Jclass, name:*const c_char, sig:*const c_char) -> JmethodID,

    call_static_object_method: fn() -> *mut u8, // not implemented
    call_static_object_method_v: fn() -> *mut u8, // not implemented
    call_static_object_method_a: fn(env:*mut JNIEnv, clazz:Jclass, method:JmethodID, args:*const Jvalue) -> Jobject,

    call_static_boolean_method: fn() -> *mut u8, // not implemented
    call_static_boolean_method_v: fn() -> *mut u8, // not implemented
    call_static_boolean_method_a: fn() -> *mut u8, // not implemented

    call_static_byte_method: fn() -> *mut u8, // not implemented
    call_static_byte_method_v: fn() -> *mut u8, // not implemented
    call_static_byte_method_a: fn() -> *mut u8, // not implemented
    
    call_static_char_method: fn() -> *mut u8, // not implemented
    call_static_char_method_v: fn() -> *mut u8, // not implemented
    call_static_char_method_a: fn() -> *mut u8, // not implemented

    call_static_short_method: fn() -> *mut u8, // not implemented
    call_static_short_method_v: fn() -> *mut u8, // not implemented
    call_static_short_method_a: fn() -> *mut u8, // not implemented

    call_static_int_method: fn() -> *mut u8, // not implemented
    call_static_int_method_v: fn() -> *mut u8, // not implemented
    call_static_int_method_a: fn() -> *mut u8, // not implemented

    call_static_long_method: fn() -> *mut u8, // not implemented
    call_static_long_method_v: fn() -> *mut u8, // not implemented
    call_static_long_method_a: fn() -> *mut u8, // not implemented

    call_static_float_method: fn() -> *mut u8, // not implemented
    call_static_float_method_v: fn() -> *mut u8, // not implemented
    call_static_float_method_a: fn() -> *mut u8, // not implemented

    call_static_double_method: fn() -> *mut u8, // not implemented
    call_static_double_method_v: fn() -> *mut u8, // not implemented
    call_static_double_method_a: fn() -> *mut u8, // not implemented

    call_static_void_method: fn() -> *mut u8, // not implemented
    call_static_void_method_v: fn() -> *mut u8, // not implemented
    call_static_void_method_a: fn(env:*mut JNIEnv, clazz:Jclass, method:JmethodID, args:*const Jvalue),

    get_static_field_id: fn() -> *mut u8, // not implemented

    get_static_object_field: fn() -> *mut u8, // not implemented
    get_static_boolean_field: fn() -> *mut u8, // not implemented
    get_static_byte_field: fn() -> *mut u8, // not implemented
    get_static_char_field: fn() -> *mut u8, // not implemented
    get_static_short_field: fn() -> *mut u8, // not implemented
    get_static_int_field: fn() -> *mut u8, // not implemented
    get_static_long_field: fn() -> *mut u8, // not implemented
    get_static_float_field: fn() -> *mut u8, // not implemented
    get_static_double_field: fn() -> *mut u8, // not implemented

    set_static_object_field: fn() -> *mut u8, // not implemented
    set_static_boolean_field: fn() -> *mut u8, // not implemented
    set_static_byte_field: fn() -> *mut u8, // not implemented
    set_static_char_field: fn() -> *mut u8, // not implemented
    set_static_short_field: fn() -> *mut u8, // not implemented
    set_static_int_field: fn() -> *mut u8, // not implemented
    set_static_long_field: fn() -> *mut u8, // not implemented
    set_static_float_field: fn() -> *mut u8, // not implemented
    set_static_double_field: fn() -> *mut u8, // not implemented

    new_string: fn() -> *mut u8, // not implemented
    get_string_length: fn() -> *mut u8, // not implemented
    get_string_chars: fn() -> *mut u8, // not implemented
    release_string_chars: fn() -> *mut u8, // not implemented

    new_string_utf: fn(env:*mut JNIEnv, utf:*const c_char) -> Jstring,
    get_string_utf_length: fn() -> *mut u8, // not implemented
    get_string_utf_chars: fn() -> *mut u8, // not implemented
    release_string_utf_chars: fn() -> *mut u8, // not implemented

    get_array_length: fn() -> *mut u8, // not implemented

    new_object_array: fn(env:*mut JNIEnv, len:Jsize, clazz:Jclass, init:Jobject) -> JobjectArray,
    get_object_array_element: fn() -> *mut u8, // not implemented
    set_object_array_element: fn(env:*mut JNIEnv, array:JobjectArray, index:Jsize, val:Jobject),

    new_boolean_array: fn() -> *mut u8, // not implemented
    new_byte_array: fn() -> *mut u8, // not implemented
    new_char_array: fn() -> *mut u8, // not implemented
    new_short_array: fn() -> *mut u8, // not implemented
    new_int_array: fn() -> *mut u8, // not implemented
    new_long_array: fn() -> *mut u8, // not implemented
    new_float_array: fn() -> *mut u8, // not implemented
    new_double_array: fn() -> *mut u8, // not implemented

    get_boolean_array_elements: fn() -> *mut u8, // not implemented
    get_byte_array_elements: fn() -> *mut u8, // not implemented
    get_char_array_elements: fn() -> *mut u8, // not implemented
    get_short_array_elements: fn() -> *mut u8, // not implemented
    get_int_array_elements: fn() -> *mut u8, // not implemented
    get_long_array_elements: fn() -> *mut u8, // not implemented
    get_float_array_elements: fn() -> *mut u8, // not implemented
    get_double_array_elements: fn() -> *mut u8, // not implemented

    release_boolean_array_elements: fn() -> *mut u8, // not implemented
    release_byte_array_elements: fn() -> *mut u8, // not implemented
    release_char_array_elements: fn() -> *mut u8, // not implemented
    release_short_array_elements: fn() -> *mut u8, // not implemented
    release_int_array_elements: fn() -> *mut u8, // not implemented
    release_long_array_elements: fn() -> *mut u8, // not implemented
    release_float_array_elements: fn() -> *mut u8, // not implemented
    release_double_array_elements: fn() -> *mut u8, // not implemented

    get_boolean_array_region: fn() -> *mut u8, // not implemented
    get_byte_array_region: fn() -> *mut u8, // not implemented
    get_char_array_region: fn() -> *mut u8, // not implemented
    get_short_array_region: fn() -> *mut u8, // not implemented
    get_int_array_region: fn() -> *mut u8, // not implemented
    get_long_array_region: fn() -> *mut u8, // not implemented
    get_float_array_region: fn() -> *mut u8, // not implemented
    get_double_array_region: fn() -> *mut u8, // not implemented

    set_boolean_array_region: fn() -> *mut u8, // not implemented
    set_byte_array_region: fn() -> *mut u8, // not implemented
    set_char_array_region: fn() -> *mut u8, // not implemented
    set_short_array_region: fn() -> *mut u8, // not implemented
    set_int_array_region: fn() -> *mut u8, // not implemented
    set_long_array_region: fn() -> *mut u8, // not implemented
    set_float_array_region: fn() -> *mut u8, // not implemented
    set_double_array_region: fn() -> *mut u8, // not implemented

    register_natives: fn() -> *mut u8, // not implemented
    unregister_natives: fn() -> *mut u8, // not implemented

    monitor_enter: fn() -> *mut u8, // not implemented
    monitor_exit: fn() -> *mut u8, // not implemented

    get_java_vm: fn() -> *mut u8, // not implemented

    get_string_region: fn() -> *mut u8, // not implemented
    get_string_utf_region: fn() -> *mut u8, // not implemented

    get_primitive_array_critical: fn() -> *mut u8, // not implemented
    release_primitive_array_critical: fn() -> *mut u8, // not implemented

    get_string_critical: fn() -> *mut u8, // not implemented
    release_string_critical: fn() -> *mut u8, // not implemented

    new_weak_global_ref: fn() -> *mut u8, // not implemented
    delete_weak_global_ref: fn() -> *mut u8, // not implemented

    exception_check: fn() -> *mut u8, // not implemented

    new_direct_byte_buffer: fn() -> *mut u8, // not implemented
    get_direct_buffer_address: fn() -> *mut u8, // not implemented
    get_direct_buffer_capacity: fn() -> *mut u8, // not implemented

    get_object_ref_type: fn() -> *mut u8, // not implemented
}

#[repr(C)]
pub struct JNIEnv {
    functions: *const JNINativeInterface
}

#[repr(C)]
#[allow(dead_code)]
pub struct JavaVMOption {
    pub option_string: *const c_char,
    pub extra_info: *mut u8
}

fn free_jvm_option(option: &JavaVMOption) {
    unsafe {
        free(transmute::<_, *mut c_void>(option.option_string))
    };
}

pub fn free_jvm_options(options: &Vec<JavaVMOption>) {
    for option in options {
        free_jvm_option(&option);
    }
}

#[repr(C)]
#[allow(dead_code)]
pub struct JavaVMInitArgs {
    pub version: Jint,
    pub n_options: Jint,
    pub options: *mut JavaVMOption,
    pub ignore_unrecognized: Jboolean
}

//#[repr(C)]
//struct JavaVMAttachArgs {
//  version: Jint,
//  name: *const c_char,
//  group: Jobject
//}

#[repr(C)]
#[allow(dead_code)]
pub struct JNIInvokeInterface {
    reserved0: *mut c_void,
    reserved1: *mut c_void,
    reserved2: *mut c_void,

    destroy_java_vm: fn(vm:*mut JavaVM) -> Jint,
    attach_current_thread: fn(vm:*mut JavaVM, env:*mut(*mut c_void), args:*mut c_void) -> Jint,
    detach_current_thread: fn(vm:*mut JavaVM) -> Jint, // not implemented
    get_env: fn(vm:*mut JavaVM, env:*mut (*mut c_void), version:Jint) -> Jint, // not implemented
    attach_current_thread_as_daemon: fn(vm:*mut JavaVM, env:*mut (*mut c_void), args:*mut c_void) -> Jint // not implemented
}

#[repr(C)]
pub struct JavaVM {
    functions: *const JNIInvokeInterface
}

pub fn destroy_java_vm(jvm:*mut JavaVM) -> Jint {
    let call = unsafe {
        (*(*jvm).functions).destroy_java_vm
    };

    call(jvm)
}

pub fn attach_current_thread(jvm:*mut JavaVM, env:*mut JNIEnv) -> Jint {
    let call = unsafe {
        (*(*jvm).functions).attach_current_thread
    };

    let env_ptr = unsafe {
        transmute::<_, *mut (*mut c_void)>(env)
    };

    call(jvm, env_ptr, ptr::null_mut())
}

pub fn get_version(env:*mut JNIEnv) -> Jint {
    let call = unsafe {
        (*(*env).functions).get_version
    };

    call(env)
}

pub fn find_class(env:*mut JNIEnv, name:&str) -> Jclass {
    let call = unsafe {
        (*(*env).functions).find_class
    };

    let name_ptr = CString::new(name).unwrap();

    call(env, name_ptr.as_ptr())
}

pub fn exception_occured(env:*mut JNIEnv) -> Jthrowable {
    let call = unsafe {
        (*(*env).functions).exception_occured
    };

    call(env)
}

pub fn exception_describe(env:*mut JNIEnv) {
    let call = unsafe {
        (*(*env).functions).exception_describe
    };

    call(env)
}

pub fn exception_clear(env:*mut JNIEnv) {
    let call = unsafe {
        (*(*env).functions).exception_clear
    };

    call(env)
}

pub fn new_object_a(env:*mut JNIEnv, clazz:Jclass, method:JmethodID, args:&[Jvalue]) -> Jobject {
    let call = unsafe {
        (*(*env).functions).new_object_a
    };

    let args_ptr = unsafe {
        transmute::<_, *const Jvalue>(args.as_ptr())
    };

    call(env, clazz, method, args_ptr)
}

pub fn get_method_id(env:*mut JNIEnv, clazz:Jclass, name:&str, sig:&str) -> JmethodID {
    let call = unsafe {
        (*(*env).functions).get_method_id
    };

    let name_ptr = CString::new(name).unwrap();

    let sig_ptr = CString::new(sig).unwrap();

    call(env, clazz, name_ptr.as_ptr(), sig_ptr.as_ptr())
}

pub fn call_object_method_a(env:*mut JNIEnv, obj:Jobject, method:JmethodID, args:&[Jvalue]) -> Jobject {
    let call = unsafe {
        (*(*env).functions).call_object_method_a
    };

    let args_ptr = unsafe {
        transmute::<_, *const Jvalue>(args.as_ptr())
    };

    call(env, obj, method, args_ptr)
}

pub fn call_void_method_a(env:*mut JNIEnv, obj:Jobject, method:JmethodID, args:&[Jvalue]) {
    let call = unsafe {
        (*(*env).functions).call_void_method_a
    };

    let args_ptr = unsafe {
        transmute::<_, *const Jvalue>(args.as_ptr())
    };

    call(env, obj, method, args_ptr)
}

pub fn get_static_method_id(env:*mut JNIEnv, clazz:Jclass, name:&str, sig:&str) -> JmethodID {
    let call = unsafe {
        (*(*env).functions).get_static_method_id
    };

    let name_ptr = CString::new(name).unwrap();

    let sig_ptr = CString::new(sig).unwrap();

    call(env, clazz, name_ptr.as_ptr(), sig_ptr.as_ptr())
}

pub fn call_static_object_method_a(env:*mut JNIEnv, clazz:Jclass, method:JmethodID, args:&[Jvalue]) -> Jobject {
    let call = unsafe {
        (*(*env).functions).call_static_object_method_a
    };

    let args_ptr = unsafe {
        transmute::<_, *const Jvalue>(args.as_ptr())
    };

    call(env, clazz, method, args_ptr)
}

pub fn call_static_void_method_a(env:*mut JNIEnv, clazz:Jclass, method:JmethodID, args:&[Jvalue]) {
    let call = unsafe {
        (*(*env).functions).call_static_void_method_a
    };

    let args_ptr = unsafe {
        transmute::<_, *const Jvalue>(args.as_ptr())
    };

    call(env, clazz, method, args_ptr)
}

pub fn new_string_utf(env:*mut JNIEnv, utf:&str) -> Jstring {
    let call = unsafe {
        (*(*env).functions).new_string_utf
    };

    let utf_ptr = CString::new(utf).unwrap();

    call(env, utf_ptr.as_ptr())
}

pub fn new_object_array(env:*mut JNIEnv, len:Jsize, clazz:Jclass, init:Jobject) -> JobjectArray {
    let call = unsafe {
        (*(*env).functions).new_object_array
    };

    call(env, len, clazz, init)
}

pub fn set_object_array_element(env:*mut JNIEnv, array:JobjectArray, index:Jsize, val:Jobject) {
    let call = unsafe {
        (*(*env).functions).set_object_array_element
    };

    call(env, array, index, val)
}
