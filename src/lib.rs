#![crate_name = "jni"]
extern crate libc;
extern crate shared_library;

pub mod classpath;
pub mod consts;
pub mod ffi;
pub mod types;

use libc::size_t;
use libc::funcs::c95::stdlib::malloc;
use shared_library::dynamic_library::DynamicLibrary;
use std::mem::transmute;
use std::path::Path;
use std::ptr;

use consts::*;
use ffi::{JavaVMInitArgs, JavaVMOption, JavaVM, JNIEnv};
use types::*;

pub struct JNI {
    libjvm:DynamicLibrary,
    vm_init_args:Box<ffi::JavaVMInitArgs>,
    vm_options:Vec<ffi::JavaVMOption>,
    jvm:Box<*mut ffi::JavaVM>,
    env:Box<*mut ffi::JNIEnv>
}

impl Drop for JNI {
    fn drop(&mut self) {
        println!("Destroying JVM instance ...");
        if !self.jvm.is_null() {
            ffi::destroy_java_vm(*self.jvm);
        }
        ffi::free_jvm_options(&self.vm_options);
    }
}

type JNICreateJavaVM = extern "C" fn(*mut(*mut JavaVM), *mut(*mut JNIEnv), *const JavaVMInitArgs) -> Jint;

impl JNI {
    pub fn new(libjvm_path:&Path) -> Result<JNI, String> {
        let libjvm = match DynamicLibrary::open(Some(libjvm_path)) {
            Err(error) => return Err(format!("Could not load JVM library '{}': {}", libjvm_path.display(), error)),
            Ok(libjvm) => libjvm
        };

        println!("JVM library loaded.");

        Ok(JNI {
            libjvm: libjvm,
            vm_init_args: Box::new(JavaVMInitArgs {
                version: JNI_VERSION_1_6,
                n_options: 0,
                options: ptr::null_mut(),
                ignore_unrecognized: JNI_FALSE
            }),
            vm_options: Vec::new(),
            jvm: Box::new(ptr::null_mut()),
            env: Box::new(ptr::null_mut())
        })
    }

    pub fn init_vm_args(&mut self, n_options:usize) {
        let mut v:Vec<JavaVMOption> = Vec::with_capacity(n_options);

        unsafe {
            v.set_len(n_options);
        }

        for i in 0..n_options {
            match v.get_mut(i) {
                Some(opt) => {
                    opt.option_string = ptr::null();
                    opt.extra_info = ptr::null_mut();
                },
                None => panic!("Out of bounds for VM arguments!")
            }
        }

        self.vm_init_args.n_options = n_options as Jint;
        self.vm_init_args.options = v.as_mut_ptr();
        self.vm_options = v;
    }

    pub fn push_vm_arg(&mut self, index:usize, option:&str) {
        let size = option.len() + 1;

        let unsafe_mem:*mut u8 = unsafe {
            transmute::<_, *mut u8>(malloc(size as size_t))
        };

        unsafe {
            ptr::write_bytes(unsafe_mem, 0, size);
            ptr::copy_nonoverlapping(option.as_ptr(), unsafe_mem, size - 1);

            let ref mut v:Vec<JavaVMOption> = self.vm_options;

            match v.get_mut(index) {
                Some(opt) => {
                    opt.option_string = transmute::<_, *const i8>(unsafe_mem);
                    opt.extra_info = ptr::null_mut();
                },
                None => panic!("Out of bounds for VM arguments!")
            }
        }
    }

    pub fn create_java_vm(&mut self) -> Result<&mut JNI, String> {
        let jni_create_java_vm:JNICreateJavaVM = unsafe {
            match self.libjvm.symbol("JNI_CreateJavaVM") {
                Err(error) => return Err(format!("Could not find symbol JNI_CreateJavaVM: {}", error)),
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

    pub fn get_jvm(&mut self) -> &mut JavaVM {
        unsafe {
            &mut (*(*self.jvm))
        }
    }

    pub fn get_env(&mut self) -> &mut JNIEnv {
        unsafe {
            &mut (*(*self.env))
        }
    }
}
