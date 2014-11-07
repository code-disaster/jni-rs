#![crate_name = "jni"]
#![feature(macro_rules)]

extern crate alloc;
extern crate libc;

use alloc::heap::{allocate, deallocate};
use libc::{c_char, c_long, c_longlong, c_void};
use std::c_vec::CVec;
use std::dynamic_lib::DynamicLibrary;
use std::mem::{align_of, size_of, transmute};
use std::ptr;


pub mod classpath;


// platform dependent JNI Types (jni_md.h)

#[cfg(target_arch = "x86_64")]
pub type Jint = c_long;

#[cfg(target_arch = "x86_64")]
pub type Jlong = c_longlong;

pub type Jbyte = i8;

#[cfg(target_arch = "x86_64")]
pub type Jpointer = u64;

#[cfg(target_arch = "x86_64")]
pub const JNI_NULL:Jpointer     = 0u64;

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

pub enum JobjectRefType {
    JNIInvalidRefType           = 0,
    JNILocalRefType             = 1,
    JNIGlobalRefType            = 2,
    JNIWeakGlobalRefType        = 3 
}

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

//static JNI_VERSION_1_1:Jint   = 0x00010001;
//static JNI_VERSION_1_2:Jint   = 0x00010002;
//static JNI_VERSION_1_4:Jint   = 0x00010004;
static JNI_VERSION_1_6:Jint     = 0x00010006;

//#[repr(C)]
//struct JNINativeMethod {
//  name: *mut c_char,
//  signature: *mut c_char,
//  fn_ptr: *mut u8
//}

#[repr(C)]
struct JNINativeInterface {
    reserved0: *mut c_void,
    reserved1: *mut c_void,
    reserved2: *mut c_void,
    reserved3: *mut c_void,

    get_version: fn(env:*mut JNIEnv) -> Jint,

    define_class: fn() -> *mut u8, // not implemented

    find_class: extern fn(env:*mut JNIEnv, name:*const c_char) -> *mut u8,

    from_reflected_method: fn() -> *mut u8, // not implemented
    from_reflected_field: fn() -> *mut u8, // not implemented
    to_reflected_method: fn() -> *mut u8, // not implemented
    get_super_class: fn() -> *mut u8, // not implemented
    is_assignable_from: fn() -> *mut u8, // not implemented
    to_reflected_field: fn() -> *mut u8, // not implemented
    
    throw: fn() -> *mut u8, // not implemented
    throw_new: fn() -> *mut u8, // not implemented
    exception_occured: fn(env:*mut JNIEnv) -> *mut u8,
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
    new_object_a: fn(env:*mut JNIEnv, clazz:*mut u8, method:*mut u8, args:*const u8) -> *mut u8,

    get_object_class: fn() -> *mut u8, // not implemented
    is_instance_of: fn() -> *mut u8, // not implemented

    get_method_id: fn(env:*mut JNIEnv, clazz:*mut u8, name:*const c_char, sig:*const c_char) -> *mut u8,

    call_object_method: fn() -> *mut u8, // not implemented
    call_object_method_v: fn() -> *mut u8, // not implemented
    call_object_method_a: fn(env:*mut JNIEnv, obj:*mut u8, method:*mut u8, args:*const u8) -> *mut u8,

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
    call_void_method_a: fn(env:*mut JNIEnv, obj:*mut u8, method:*mut u8, args:*const u8),

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

    get_static_method_id: fn(env:*mut JNIEnv, clazz:*mut u8, name:*const c_char, sig:*const c_char) -> *mut u8,

    call_static_object_method: fn() -> *mut u8, // not implemented
    call_static_object_method_v: fn() -> *mut u8, // not implemented
    call_static_object_method_a: fn(env:*mut JNIEnv, clazz:*mut u8, method:*mut u8, args:*const u8) -> *mut u8,

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
    call_static_void_method_a: fn(env:*mut JNIEnv, clazz:*mut u8, method:*mut u8, args:*const u8),

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

    new_string_utf: fn(env:*mut JNIEnv, utf:*const c_char) -> *mut u8,
    get_string_utf_length: fn() -> *mut u8, // not implemented
    get_string_utf_chars: fn() -> *mut u8, // not implemented
    release_string_utf_chars: fn() -> *mut u8, // not implemented

    get_array_length: fn() -> *mut u8, // not implemented

    new_object_array: fn(env:*mut JNIEnv, len:Jsize, clazz:*mut u8, init:*mut u8) -> *mut u8,
    get_object_array_element: fn() -> *mut u8, // not implemented
    set_object_array_element: fn(env:*mut JNIEnv, array:*mut u8, index:Jsize, val:*mut u8),

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

    get_java_vm: fn(env:*const JNIEnv, jvm:*mut(*mut JavaVM)) -> *mut u8, // not implemented

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
struct JNIEnv {
    functions: *const JNINativeInterface
}

#[repr(C)]
struct JavaVMOption {
    option_string: *const c_char,
    extra_info: *mut u8
}

impl Drop for JavaVMOption {
    fn drop(&mut self) {
        let size = self.extra_info as uint;
        let align = align_of::<*mut i8>();

        unsafe {
            deallocate(transmute::<_, *mut u8>(self.option_string), size, align)
        };
    }
}

#[repr(C)]
struct JavaVMInitArgs {
    version: Jint,
    n_options: Jint,
    options: *mut JavaVMOption,
    ignore_unrecognized: Jboolean
}

impl Drop for JavaVMInitArgs {
    fn drop(&mut self) {
        let size = size_of::<JavaVMOption>();
        let align = align_of::<*mut JavaVMOption>();

        unsafe {
            deallocate(transmute::<_, *mut u8>(self.options), self.n_options as uint * size, align)
        };
    }
}

//#[repr(C)]
//struct JavaVMAttachArgs {
//  version: Jint,
//  name: *const c_char,
//  group: Jobject
//}

#[repr(C)]
struct JNIInvokeInterface {
    reserved0: *mut c_void,
    reserved1: *mut c_void,
    reserved2: *mut c_void,

    destroy_java_vm: fn(vm:*mut JavaVM) -> Jint,
    attach_current_thread: fn(vm:*mut JavaVM, env:*mut(*mut JNIEnv), args:*const u8) -> Jint,
    
    detach_current_thread: fn(vm:*mut JavaVM) -> Jint, // not implemented
    get_env: fn(vm:*mut JavaVM, env:*mut (*mut u8), version:Jint) -> Jint, // not implemented
    attach_current_thread_as_daemon: fn(vm:*mut JavaVM, env:*mut (*mut u8), args:*mut u8) -> Jint // not implemented
}

#[repr(C)]
struct JavaVM {
    functions: *const JNIInvokeInterface
}




//_JNI_IMPORT_OR_EXPORT_ jint JNICALL JNI_GetDefaultJavaVMInitArgs(void *args);
//_JNI_IMPORT_OR_EXPORT_ jint JNICALL JNI_GetCreatedJavaVMs(JavaVM **, jsize, jsize *);
//JNIEXPORT jint JNICALL JNI_OnLoad(JavaVM *vm, void *reserved);
//JNIEXPORT void JNICALL JNI_OnUnload(JavaVM *vm, void *reserved);

type JNICreateJavaVM = extern "C" fn(*mut(*mut JavaVM), *mut(*mut JNIEnv), *const JavaVMInitArgs) -> Jint;

pub struct JNI {
    libjvm:DynamicLibrary,
    vm_init_args:Box<JavaVMInitArgs>,
    jvm:Box<*mut JavaVM>,
    env:Box<*mut JNIEnv>
}

impl JNI {
    pub fn new(libjvm_path:&Path) -> Result<JNI, String> {
        let libjvm = match DynamicLibrary::open(Some(libjvm_path)) {
            Err(error) => return Err(format!("Could not load JVM library: {}", error)),
            Ok(libjvm) => libjvm
        };

        Ok(JNI {
            libjvm: libjvm,
            vm_init_args: box JavaVMInitArgs {
                version: JNI_VERSION_1_6,
                n_options: 0,
                options: ptr::null_mut(),
                ignore_unrecognized: JNI_FALSE
            },
            jvm: box ptr::null_mut(),
            env: box ptr::null_mut()
        })
    }

    pub fn init_vm_args(&mut self, n_options:uint) {
        let size = size_of::<JavaVMOption>();
        let align = align_of::<*mut JavaVMOption>();

        let unsafe_mem:*mut JavaVMOption = unsafe {
            transmute::<_, *mut JavaVMOption>(allocate(n_options * size, align))
        };

        let mut v:CVec<JavaVMOption> = unsafe {
            CVec::new(unsafe_mem, n_options)
        };

        for i in range(0u, n_options) {
            match v.get_mut(i) {
                Some(opt) => {
                    opt.option_string = ptr::null();
                    opt.extra_info = ptr::null_mut();
                },
                None => panic!("Out of bounds for VM arguments!")
            }
        }

        self.vm_init_args.n_options = n_options as Jint;
        self.vm_init_args.options = unsafe_mem;
    }

    pub fn push_vm_arg(&mut self, index:uint, option:&str) {
        let size = option.len() + 1;
        let align = align_of::<*mut i8>();

        let unsafe_mem:*mut u8 = unsafe {
            allocate(size, align)
        };

        unsafe {
            ptr::zero_memory(unsafe_mem, size);
            ptr::copy_memory(unsafe_mem, option.as_ptr(), size - 1);

            let mut v = CVec::new(self.vm_init_args.options, self.vm_init_args.n_options as uint);

            match v.get_mut(index) {
                Some(opt) => {
                    opt.option_string = transmute::<_, *const i8>(unsafe_mem);
                    // hack: store allocation size in extra_info
                    let len:Jpointer = size as Jpointer;
                    opt.extra_info = transmute::<_, *mut u8>(len);
                },
                None => panic!("Out of bounds for VM arguments!")
            }
        }
    }

    pub fn load_jvm(&mut self) -> Result<&mut JNI, String> {
        let jni_create_java_vm:JNICreateJavaVM = unsafe {
            match self.libjvm.symbol("JNI_CreateJavaVM") {
                Err(error) => return Err(format!("Could not load function JNI_CreateJavaVM: {}", error)),
                Ok(jni_create_java_vm) => transmute::<*mut u8, _>(jni_create_java_vm)
            }
        };

        {
            let args:&JavaVMInitArgs = &(*self.vm_init_args);
            let result = jni_create_java_vm(&mut *self.jvm, &mut *self.env, args);

            if result != JNI_OK {
                return Err(format!("Error calling JNI_CreateJavaVM: error code {:x}", result));
            }
        }

        Ok(self)
    }

    pub fn destroy_jvm(&self) -> Jint {
        let jvm = *self.jvm;

        let call = unsafe {
            (*(*jvm).functions).destroy_java_vm
        };

        call(jvm)
    }

    pub fn attach_current_thread(&mut self) -> Jint {
        let jvm = *self.jvm;

        let call = unsafe {
            (*(*jvm).functions).attach_current_thread
        };

        call(jvm, &mut *(self.env), ptr::null())
    }

    pub fn get_version(&self) -> Jint {
        let env = *self.env;
        
        let call = unsafe {
            (*(*env).functions).get_version
        };

        call(env)
    }

    pub fn find_class(&self, name:&str) -> Jclass {
        let env = *self.env;
        
        let call = unsafe {
            (*(*env).functions).find_class
        };

        unsafe {
            transmute::<_, Jpointer>(call(env, name.to_c_str().as_ptr()))
        }
    }

    pub fn exception_occured(&self) -> Jthrowable {
        let env = *self.env;
        
        let call = unsafe {
            (*(*env).functions).exception_occured
        };

        unsafe {
            transmute::<_, Jpointer>(call(env))
        }
    }

    pub fn exception_describe(&self) {
        let env = *self.env;
        
        let call = unsafe {
            (*(*env).functions).exception_describe
        };

        call(env)
    }

    pub fn exception_clear(&self) {
        let env = *self.env;
        
        let call = unsafe {
            (*(*env).functions).exception_clear
        };

        call(env)
    }

    pub fn new_object_a(&self, clazz:Jclass, method:JmethodID, args:&[Jvalue]) -> Jobject {
        let env = *self.env;

        let call = unsafe {
            (*(*env).functions).new_object_a
        };

        let clazz_ptr = unsafe {
            transmute::<_, *mut u8>(clazz)
        };

        let method_ptr = unsafe {
            transmute::<_, *mut u8>(method)
        };

        let args_ptr = unsafe {
            transmute::<_, *const u8>(args.as_ptr())
        };

        unsafe {
            transmute::<_, Jpointer>(call(env, clazz_ptr, method_ptr, args_ptr))
        }
    }

    pub fn get_method_id(&self, clazz:Jclass, name:&str, sig:&str) -> JmethodID {
        let env = *self.env;

        let call = unsafe {
            (*(*env).functions).get_method_id
        };

        let clazz_ptr = unsafe {
            transmute::<_, *mut u8>(clazz)
        };

        unsafe {
            transmute::<_, Jpointer>(call(env, clazz_ptr, name.to_c_str().as_ptr(), sig.to_c_str().as_ptr()))
        }
    }

    pub fn call_object_method_a(&self, obj:Jobject, method:JmethodID, args:&[Jvalue]) -> Jobject {
        let env = *self.env;

        let call = unsafe {
            (*(*env).functions).call_object_method_a
        };

        let obj_ptr = unsafe {
            transmute::<_, *mut u8>(obj)
        };

        let method_ptr = unsafe {
            transmute::<_, *mut u8>(method)
        };

        let args_ptr = unsafe {
            transmute::<_, *const u8>(args.as_ptr())
        };

        unsafe {
            transmute::<_, Jpointer>(call(env, obj_ptr, method_ptr, args_ptr))
        }
    }

    pub fn call_void_method_a(&self, obj:Jobject, method:JmethodID, args:&[Jvalue]) {
        let env = *self.env;

        let call = unsafe {
            (*(*env).functions).call_void_method_a
        };

        let obj_ptr = unsafe {
            transmute::<_, *mut u8>(obj)
        };

        let method_ptr = unsafe {
            transmute::<_, *mut u8>(method)
        };

        let args_ptr = unsafe {
            transmute::<_, *const u8>(args.as_ptr())
        };

        call(env, obj_ptr, method_ptr, args_ptr)
    }

    pub fn get_static_method_id(&self, clazz:Jclass, name:&str, sig:&str) -> JmethodID {
        let env = *self.env;
        
        let call = unsafe {
            (*(*env).functions).get_static_method_id
        };

        let clazz_ptr = unsafe {
            transmute::<_, *mut u8>(clazz)
        };

        unsafe {
            transmute::<_, Jpointer>(call(env, clazz_ptr, name.to_c_str().as_ptr(), sig.to_c_str().as_ptr()))
        }
    }

    pub fn call_static_object_method_a(&self, clazz:Jclass, method:JmethodID, args:&[Jvalue]) -> Jobject {
        let env = *self.env;

        let call = unsafe {
            (*(*env).functions).call_static_object_method_a
        };

        let clazz_ptr = unsafe {
            transmute::<_, *mut u8>(clazz)
        };

        let method_ptr = unsafe {
            transmute::<_, *mut u8>(method)
        };

        let args_ptr = unsafe {
            transmute::<_, *const u8>(args.as_ptr())
        };

        unsafe {
            transmute::<_, Jpointer>(call(env, clazz_ptr, method_ptr, args_ptr))
        }
    }

    pub fn call_static_void_method_a(&self, clazz:Jclass, method:JmethodID, args:&[Jvalue]) {
        let env = *self.env;

        let call = unsafe {
            (*(*env).functions).call_static_void_method_a
        };

        let clazz_ptr = unsafe {
            transmute::<_, *mut u8>(clazz)
        };

        let method_ptr = unsafe {
            transmute::<_, *mut u8>(method)
        };

        let args_ptr = unsafe {
            transmute::<_, *const u8>(args.as_ptr())
        };

        call(env, clazz_ptr, method_ptr, args_ptr)
    }

    pub fn new_string_utf(&self, utf:&str) -> Jstring {
        let env = *self.env;
        
        let call = unsafe {
            (*(*env).functions).new_string_utf
        };

        unsafe {
            transmute::<_, Jpointer>(call(env, utf.to_c_str().as_ptr()))
        }
    }

    pub fn new_object_array(&self, len:Jsize, clazz:Jclass, init:Jobject) -> JobjectArray {
        let env = *self.env;
        
        let call = unsafe {
            (*(*env).functions).new_object_array
        };

        let clazz_ptr = unsafe {
            transmute::<_, *mut u8>(clazz)
        };

        let init_ptr = unsafe {
            transmute::<_, *mut u8>(init)
        };

        unsafe {
            transmute::<_, Jpointer>(call(env, len, clazz_ptr, init_ptr))
        }
    }

    pub fn set_object_array_element(&self, array:JobjectArray, index:Jsize, val:Jobject) {
        let env = *self.env;
        
        let call = unsafe {
            (*(*env).functions).set_object_array_element
        };

        let array_ptr = unsafe {
            transmute::<_, *mut u8>(array)
        };

        let val_ptr = unsafe {
            transmute::<_, *mut u8>(val)
        };

        call(env, array_ptr, index, val_ptr)
    }

    pub fn is_null(p:Jpointer) -> bool {
        p == 0u64
    }
}
