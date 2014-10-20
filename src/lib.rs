#![crate_name = "jni"]
#![experimental]
#![crate_type = "rlib"]
#![crate_type = "dylib"]
#![license = "MIT/ASL2"]
#![doc(html_logo_url = "http://www.rust-lang.org/logos/rust-logo-128x128-blk-v2.png",
       html_favicon_url = "http://www.rust-lang.org/favicon.ico",
       html_root_url = "http://doc.rust-lang.org/0.12.0/",
       html_playground_url = "http://play.rust-lang.org/")]
#![feature(globs, phase)]
#![feature(import_shadowing)]
//#![deny(missing_doc)]

extern crate alloc;
extern crate libc;

use alloc::heap::{allocate, deallocate};
use libc::{c_char, c_long, c_longlong, c_void};
use std::c_vec::CVec;
use std::dynamic_lib::DynamicLibrary;
use std::{mem, ptr};
use std::c_str::CString;

// JNI Types (jni_md.h)

pub type Jint = c_long;
pub type Jlong = c_longlong;
pub type Jbyte = i8;

pub type Jpointer = u64;

// JNI Types

pub type Jboolean = u8;
pub type Jchar = u16;
pub type Jshort = i16;
pub type Jfloat = f32;
pub type Jdouble = f64;
pub type Jsize = Jint;

pub fn jni_pointer_is_null(p:Jpointer) -> bool {
	p == 0u64
}

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

#[repr(C)]
pub struct JNINativeMethod {
	name: *mut c_char,
	signature: *mut c_char,
	fn_ptr: *mut u8
}

#[repr(C)]
struct JNINativeInterface {
	reserved0: *mut c_void,
	reserved1: *mut c_void,
	reserved2: *mut c_void,
	reserved3: *mut c_void,

	get_version: fn(env:*mut JNIEnv) -> Jint,

	define_class: fn(env:*const JNIEnv, name:*const c_char, loader:Jobject, buf:*const Jbyte, len:Jsize) -> Jclass,

	find_class: extern fn(env:*mut JNIEnv, name:*const c_char) -> *mut u8,

	from_reflected_method: fn() -> Jint, // not implemented
	from_reflected_field: fn() -> Jint, // not implemented
	to_reflected_method: fn() -> Jint, // not implemented
	get_super_class: fn() -> Jint, // not implemented
	is_assignable_from: fn() -> Jint, // not implemented
	to_reflected_field: fn() -> Jint, // not implemented
	
	throw: fn() -> Jint, // not implemented
	throw_new: fn() -> Jint, // not implemented
	exception_occured: fn() -> Jint, // not implemented
	exception_describe: fn() -> Jint, // not implemented
	exception_clear: fn() -> Jint, // not implemented
	fatal_error: fn() -> Jint, // not implemented

	push_local_frame: fn() -> Jint, // not implemented
	pop_local_frame: fn() -> Jint, // not implemented

	new_global_ref: fn() -> Jint, // not implemented
	delete_global_ref: fn() -> Jint, // not implemented
	delete_local_ref: fn() -> Jint, // not implemented
	is_same_object: fn() -> Jint, // not implemented
	new_local_ref: fn() -> Jint, // not implemented
	ensure_local_capacity: fn() -> Jint, // not implemented

	alloc_object: fn() -> Jint, // not implemented
	new_object: fn() -> Jint, // not implemented
	new_object_v: fn() -> Jint, // not implemented
	new_object_a: fn() -> Jint, // not implemented

	get_object_class: fn() -> Jint, // not implemented
	is_instance_of: fn() -> Jint, // not implemented

	get_method_id: fn() -> Jint, // not implemented

	call_object_method: fn() -> Jint, // not implemented
	call_object_method_v: fn() -> Jint, // not implemented
	call_object_method_a: fn() -> Jint, // not implemented

	call_boolean_method: fn() -> Jint, // not implemented
	call_boolean_method_v: fn() -> Jint, // not implemented
	call_boolean_method_a: fn() -> Jint, // not implemented

	call_byte_method: fn() -> Jint, // not implemented
	call_byte_method_v: fn() -> Jint, // not implemented
	call_byte_method_a: fn() -> Jint, // not implemented
	
	call_char_method: fn() -> Jint, // not implemented
	call_char_method_v: fn() -> Jint, // not implemented
	call_char_method_a: fn() -> Jint, // not implemented

	call_short_method: fn() -> Jint, // not implemented
	call_short_method_v: fn() -> Jint, // not implemented
	call_short_method_a: fn() -> Jint, // not implemented

	call_int_method: fn() -> Jint, // not implemented
	call_int_method_v: fn() -> Jint, // not implemented
	call_int_method_a: fn() -> Jint, // not implemented

	call_long_method: fn() -> Jint, // not implemented
	call_long_method_v: fn() -> Jint, // not implemented
	call_long_method_a: fn() -> Jint, // not implemented

	call_float_method: fn() -> Jint, // not implemented
	call_float_method_v: fn() -> Jint, // not implemented
	call_float_method_a: fn() -> Jint, // not implemented

	call_double_method: fn() -> Jint, // not implemented
	call_double_method_v: fn() -> Jint, // not implemented
	call_double_method_a: fn() -> Jint, // not implemented

	call_void_method: fn() -> Jint, // not implemented
	call_void_method_v: fn() -> Jint, // not implemented
	call_void_method_a: fn() -> Jint, // not implemented

	call_nonvirtual_object_method: fn() -> Jint, // not implemented
	call_nonvirtual_object_method_v: fn() -> Jint, // not implemented
	call_nonvirtual_object_method_a: fn() -> Jint, // not implemented

	call_nonvirtual_boolean_method: fn() -> Jint, // not implemented
	call_nonvirtual_boolean_method_v: fn() -> Jint, // not implemented
	call_nonvirtual_boolean_method_a: fn() -> Jint, // not implemented

	call_nonvirtual_byte_method: fn() -> Jint, // not implemented
	call_nonvirtual_byte_method_v: fn() -> Jint, // not implemented
	call_nonvirtual_byte_method_a: fn() -> Jint, // not implemented
	
	call_nonvirtual_char_method: fn() -> Jint, // not implemented
	call_nonvirtual_char_method_v: fn() -> Jint, // not implemented
	call_nonvirtual_char_method_a: fn() -> Jint, // not implemented

	call_nonvirtual_short_method: fn() -> Jint, // not implemented
	call_nonvirtual_short_method_v: fn() -> Jint, // not implemented
	call_nonvirtual_short_method_a: fn() -> Jint, // not implemented

	call_nonvirtual_int_method: fn() -> Jint, // not implemented
	call_nonvirtual_int_method_v: fn() -> Jint, // not implemented
	call_nonvirtual_int_method_a: fn() -> Jint, // not implemented

	call_nonvirtual_long_method: fn() -> Jint, // not implemented
	call_nonvirtual_long_method_v: fn() -> Jint, // not implemented
	call_nonvirtual_long_method_a: fn() -> Jint, // not implemented

	call_nonvirtual_float_method: fn() -> Jint, // not implemented
	call_nonvirtual_float_method_v: fn() -> Jint, // not implemented
	call_nonvirtual_float_method_a: fn() -> Jint, // not implemented

	call_nonvirtual_double_method: fn() -> Jint, // not implemented
	call_nonvirtual_double_method_v: fn() -> Jint, // not implemented
	call_nonvirtual_double_method_a: fn() -> Jint, // not implemented

	call_nonvirtual_void_method: fn() -> Jint, // not implemented
	call_nonvirtual_void_method_v: fn() -> Jint, // not implemented
	call_nonvirtual_void_method_a: fn() -> Jint, // not implemented

	get_field_id: fn() -> Jint, // not implemented

	get_object_field: fn() -> Jint, // not implemented
	get_boolean_field: fn() -> Jint, // not implemented
	get_byte_field: fn() -> Jint, // not implemented
	get_char_field: fn() -> Jint, // not implemented
	get_short_field: fn() -> Jint, // not implemented
	get_int_field: fn() -> Jint, // not implemented
	get_long_field: fn() -> Jint, // not implemented
	get_float_field: fn() -> Jint, // not implemented
	get_double_field: fn() -> Jint, // not implemented

	set_object_field: fn() -> Jint, // not implemented
	set_boolean_field: fn() -> Jint, // not implemented
	set_byte_field: fn() -> Jint, // not implemented
	set_char_field: fn() -> Jint, // not implemented
	set_short_field: fn() -> Jint, // not implemented
	set_int_field: fn() -> Jint, // not implemented
	set_long_field: fn() -> Jint, // not implemented
	set_float_field: fn() -> Jint, // not implemented
	set_double_field: fn() -> Jint, // not implemented

	get_static_method_id: fn(env:*mut JNIEnv, clazz:Jpointer, name:*const c_char, sig:*const c_char) -> JmethodID,

	call_static_object_method: fn() -> Jint, // not implemented
	call_static_object_method_v: fn() -> Jint, // not implemented
	call_static_object_method_a: fn() -> Jint, // not implemented

	call_static_boolean_method: fn() -> Jint, // not implemented
	call_static_boolean_method_v: fn() -> Jint, // not implemented
	call_static_boolean_method_a: fn() -> Jint, // not implemented

	call_static_byte_method: fn() -> Jint, // not implemented
	call_static_byte_method_v: fn() -> Jint, // not implemented
	call_static_byte_method_a: fn() -> Jint, // not implemented
	
	call_static_char_method: fn() -> Jint, // not implemented
	call_static_char_method_v: fn() -> Jint, // not implemented
	call_static_char_method_a: fn() -> Jint, // not implemented

	call_static_short_method: fn() -> Jint, // not implemented
	call_static_short_method_v: fn() -> Jint, // not implemented
	call_static_short_method_a: fn() -> Jint, // not implemented

	call_static_int_method: fn() -> Jint, // not implemented
	call_static_int_method_v: fn() -> Jint, // not implemented
	call_static_int_method_a: fn() -> Jint, // not implemented

	call_static_long_method: fn() -> Jint, // not implemented
	call_static_long_method_v: fn() -> Jint, // not implemented
	call_static_long_method_a: fn() -> Jint, // not implemented

	call_static_float_method: fn() -> Jint, // not implemented
	call_static_float_method_v: fn() -> Jint, // not implemented
	call_static_float_method_a: fn() -> Jint, // not implemented

	call_static_double_method: fn() -> Jint, // not implemented
	call_static_double_method_v: fn() -> Jint, // not implemented
	call_static_double_method_a: fn() -> Jint, // not implemented

	call_static_void_method: fn() -> Jint, // not implemented
	call_static_void_method_v: fn() -> Jint, // not implemented
	call_static_void_method_a: fn() -> Jint, // not implemented

	get_static_field_id: fn() -> Jint, // not implemented

	get_static_object_field: fn() -> Jint, // not implemented
	get_static_boolean_field: fn() -> Jint, // not implemented
	get_static_byte_field: fn() -> Jint, // not implemented
	get_static_char_field: fn() -> Jint, // not implemented
	get_static_short_field: fn() -> Jint, // not implemented
	get_static_int_field: fn() -> Jint, // not implemented
	get_static_long_field: fn() -> Jint, // not implemented
	get_static_float_field: fn() -> Jint, // not implemented
	get_static_double_field: fn() -> Jint, // not implemented

	set_static_object_field: fn() -> Jint, // not implemented
	set_static_boolean_field: fn() -> Jint, // not implemented
	set_static_byte_field: fn() -> Jint, // not implemented
	set_static_char_field: fn() -> Jint, // not implemented
	set_static_short_field: fn() -> Jint, // not implemented
	set_static_int_field: fn() -> Jint, // not implemented
	set_static_long_field: fn() -> Jint, // not implemented
	set_static_float_field: fn() -> Jint, // not implemented
	set_static_double_field: fn() -> Jint, // not implemented

	new_string: fn() -> Jint, // not implemented
	get_string_length: fn() -> Jint, // not implemented
	get_string_chars: fn() -> Jint, // not implemented
	release_string_chars: fn() -> Jint, // not implemented

	new_string_utf: fn() -> Jint, // not implemented
	get_string_utf_length: fn() -> Jint, // not implemented
	get_string_utf_chars: fn() -> Jint, // not implemented
	release_string_utf_chars: fn() -> Jint, // not implemented

	get_array_length: fn() -> Jint, // not implemented

	new_object_array: fn() -> Jint, // not implemented
	get_object_array_element: fn() -> Jint, // not implemented
	set_object_array_element: fn() -> Jint, // not implemented

	new_boolean_array: fn() -> Jint, // not implemented
	new_byte_array: fn() -> Jint, // not implemented
	new_char_array: fn() -> Jint, // not implemented
	new_short_array: fn() -> Jint, // not implemented
	new_int_array: fn() -> Jint, // not implemented
	new_long_array: fn() -> Jint, // not implemented
	new_float_array: fn() -> Jint, // not implemented
	new_double_array: fn() -> Jint, // not implemented

	get_boolean_array_elements: fn() -> Jint, // not implemented
	get_byte_array_elements: fn() -> Jint, // not implemented
	get_char_array_elements: fn() -> Jint, // not implemented
	get_short_array_elements: fn() -> Jint, // not implemented
	get_int_array_elements: fn() -> Jint, // not implemented
	get_long_array_elements: fn() -> Jint, // not implemented
	get_float_array_elements: fn() -> Jint, // not implemented
	get_double_array_elements: fn() -> Jint, // not implemented

	release_boolean_array_elements: fn() -> Jint, // not implemented
	release_byte_array_elements: fn() -> Jint, // not implemented
	release_char_array_elements: fn() -> Jint, // not implemented
	release_short_array_elements: fn() -> Jint, // not implemented
	release_int_array_elements: fn() -> Jint, // not implemented
	release_long_array_elements: fn() -> Jint, // not implemented
	release_float_array_elements: fn() -> Jint, // not implemented
	release_double_array_elements: fn() -> Jint, // not implemented

	get_boolean_array_region: fn() -> Jint, // not implemented
	get_byte_array_region: fn() -> Jint, // not implemented
	get_char_array_region: fn() -> Jint, // not implemented
	get_short_array_region: fn() -> Jint, // not implemented
	get_int_array_region: fn() -> Jint, // not implemented
	get_long_array_region: fn() -> Jint, // not implemented
	get_float_array_region: fn() -> Jint, // not implemented
	get_double_array_region: fn() -> Jint, // not implemented

	set_boolean_array_region: fn() -> Jint, // not implemented
	set_byte_array_region: fn() -> Jint, // not implemented
	set_char_array_region: fn() -> Jint, // not implemented
	set_short_array_region: fn() -> Jint, // not implemented
	set_int_array_region: fn() -> Jint, // not implemented
	set_long_array_region: fn() -> Jint, // not implemented
	set_float_array_region: fn() -> Jint, // not implemented
	set_double_array_region: fn() -> Jint, // not implemented

	register_natives: fn() -> Jint, // not implemented
	unregister_natives: fn() -> Jint, // not implemented

	monitor_enter: fn() -> Jint, // not implemented
	monitor_exit: fn() -> Jint, // not implemented

	get_java_vm: fn(env:*const JNIEnv, jvm:*mut(*mut JavaVM)) -> Jint, // not implemented

	get_string_region: fn() -> Jint, // not implemented
	get_string_utf_region: fn() -> Jint, // not implemented

	get_primitive_array_critical: fn() -> Jint, // not implemented
	release_primitive_array_critical: fn() -> Jint, // not implemented

	get_string_critical: fn() -> Jint, // not implemented
	release_string_critical: fn() -> Jint, // not implemented

	new_weak_global_ref: fn() -> Jint, // not implemented
	delete_weak_global_ref: fn() -> Jint, // not implemented

	exception_check: fn() -> Jint, // not implemented

	new_direct_byte_buffer: fn() -> Jint, // not implemented
	get_direct_buffer_address: fn() -> Jint, // not implemented
	get_direct_buffer_capacity: fn() -> Jint, // not implemented

	get_object_ref_type: fn() -> Jint, // not implemented
}

#[repr(C)]
pub struct JNIEnv {
	functions: *const JNINativeInterface
}

#[repr(C)]
pub struct JavaVMOption {
	pub option_string: *const c_char,
	pub extra_info: *const u8
}

#[repr(C)]
pub struct JavaVMInitArgs {
	pub version: Jint,
	pub n_options: Jint,
	pub options: *mut JavaVMOption,
	pub ignore_unrecognized: Jboolean
}

impl Drop for JavaVMInitArgs {
	fn drop(&mut self) {
		unsafe {
			let unsafe_mem = mem::transmute::<_, _>(self.options);
			deallocate(unsafe_mem, self.n_options as uint * mem::size_of::<JavaVMOption>(), mem::align_of::<JavaVMOption>());
		}
	}
}

impl JavaVMInitArgs {
	pub fn new(n_options:uint) -> Result<JavaVMInitArgs, String> {
		unsafe {
			let unsafe_mem = allocate(n_options * mem::size_of::<JavaVMOption>(), mem::align_of::<JavaVMOption>());

			Ok(JavaVMInitArgs {
				version: JNI_VERSION_1_6,
				n_options: n_options as Jint,
				options: mem::transmute::<_, _>(unsafe_mem),
				ignore_unrecognized: JNI_FALSE
			})
		}
	}

	pub fn push(&mut self, index:uint, option:&String) {
		unsafe {
			let mut v = CVec::new(self.options, self.n_options as uint);
			let opt = v.get_mut(index);
			match opt {
				Some(opt) => {
					opt.option_string = option.to_c_str().unwrap();
					opt.extra_info = ptr::null();
				},
				None => fail!("Out of bounds for VM arguments!")
			}
		}
	}
}

#[repr(C)]
pub struct JavaVMAttachArgs {
	pub version: Jint,
	pub name: *const c_char,
	pub group: Jobject
}

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
pub struct JavaVM {
	functions: *const JNIInvokeInterface
}

//_JNI_IMPORT_OR_EXPORT_ jint JNICALL JNI_GetDefaultJavaVMInitArgs(void *args);
//_JNI_IMPORT_OR_EXPORT_ jint JNICALL JNI_GetCreatedJavaVMs(JavaVM **, jsize, jsize *);
//JNIEXPORT jint JNICALL JNI_OnLoad(JavaVM *vm, void *reserved);
//JNIEXPORT void JNICALL JNI_OnUnload(JavaVM *vm, void *reserved);

type JNICreateJavaVM = extern "C" fn(*mut(*mut JavaVM), *mut(*mut JNIEnv), *const JavaVMInitArgs) -> Jint;

pub struct JNI {
	libjvm:DynamicLibrary,
	jvm:Box<*mut JavaVM>,
	env:Box<*mut JNIEnv>
}

impl JNI {
	pub fn new(libjvm_path:&Path) -> Result<JNI, String> {
		let libjvm = match DynamicLibrary::open(Some(libjvm_path)) {
			Err(error) => return Err(format!("Could not load JVM library: {}", error)),
			Ok(libjvm) => libjvm
		};

		Ok(JNI {libjvm: libjvm, jvm: box ptr::null_mut(), env: box ptr::null_mut()})
	}

	pub fn load_jvm(&mut self, args:&JavaVMInitArgs) -> Result<&mut JNI, String> {
		assert!(self.is_null());

		let jni_create_java_vm:JNICreateJavaVM = unsafe {
			match self.libjvm.symbol("JNI_CreateJavaVM") {
				Err(error) => return Err(format!("Could not load function JNI_CreateJavaVM: {}", error)),
				Ok(jni_create_java_vm) => mem::transmute::<*mut u8, _>(jni_create_java_vm)
			}
		};

		for i in range(0u, args.n_options as uint) {
			unsafe {
				let mut v = CVec::new(args.options, args.n_options as uint);
				let opt = v.get_mut(i);
				match opt {
					Some(opt) => {
						match CString::new(opt.option_string, false).as_str() {
							Some(s) => println!("VM arg: {:s}", s),
							None => {}
						}
					},
					None => {}
				}
			}
		}

		let result = jni_create_java_vm(&mut *self.jvm, &mut *self.env, args);

		if result != JNI_OK {
			return Err(format!("Error calling JNI_CreateJavaVM: error code {}", result));
		}

		assert!(!self.is_null());
		assert!(self.check_null_functions());

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
		assert!(!jvm.is_null());

		let call = unsafe {
			(*(*jvm).functions).attach_current_thread
		};

		call(jvm, &mut *(self.env), ptr::null())
	}

	pub fn get_version(&self) -> Jint {
		let env = *self.env;
		assert!(!env.is_null());
		
		let call = unsafe {
			(*(*env).functions).get_version
		};

		let call_ptr = unsafe {
			mem::transmute::<_, *const u8>(call)
		};

		assert!(!call_ptr.is_null());

		call(env)
	}

	pub fn find_class(&self, name:&str) -> Jclass {
		let env = *self.env;
		assert!(!env.is_null());
		
		let call = unsafe {
			(*(*env).functions).find_class
		};

		let call_ptr = unsafe {
			mem::transmute::<_, *const u8>(call)
		};

		assert!(!call_ptr.is_null());

		println!("find_class: {:s}", name);

		let result:*mut u8 = call(env, name.to_c_str().as_ptr());
		println!("find_class result: {}", result);

		unsafe { mem::transmute::<_, Jpointer>(result) }
	}

	pub fn get_static_method_id(&self, clazz:&Jclass, name:&str, sig:&str) -> JmethodID {
		let env = *self.env;
		assert!(!env.is_null());
		
		let call = unsafe {
			(*(*env).functions).get_static_method_id
		};

		let call_ptr = unsafe {
			mem::transmute::<_, *const u8>(call)
		};

		assert!(!call_ptr.is_null());

		call(env, *clazz, name.to_c_str().as_ptr(), sig.to_c_str().as_ptr())
	}

	pub fn is_null(&self) -> bool {
		self.jvm.is_null() || self.env.is_null()
	}

	fn check_null_functions(&self) -> bool {
		let env = *self.env;
		
		let call = unsafe {
			(*(*env).functions).find_class
		};

		let call_ptr = unsafe {
			mem::transmute::<_, *const u8>(call)
		};
		
		!call_ptr.is_null()
	}
}

pub static JNI_VERSION_1_1:Jint = 0x00010001;
pub static JNI_VERSION_1_2:Jint = 0x00010002;
pub static JNI_VERSION_1_4:Jint = 0x00010004;
pub static JNI_VERSION_1_6:Jint = 0x00010006;
