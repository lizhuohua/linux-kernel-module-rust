#![no_std]
#![feature(alloc)]

extern crate alloc;

use crate::alloc::boxed::Box;
use linux_device_driver::bindings;
use linux_device_driver::c_types;
use linux_device_driver::println;

struct CharDevModule {
    major: c_types::c_int,
    name: &'static str,
    fops: Box<bindings::file_operations>,
}

extern "C" {
    pub fn __register_chrdev(
        major: c_types::c_uint,
        baseminor: c_types::c_uint,
        count: c_types::c_uint,
        name: *const c_types::c_char,
        fops: *const bindings::file_operations,
    ) -> c_types::c_int;
}

extern "C" {
    pub fn __unregister_chrdev(
        major: c_types::c_uint,
        baseminor: c_types::c_uint,
        count: c_types::c_uint,
        name: *const c_types::c_char,
    );
}

extern "C" {
    pub fn copy_to_user_wrapper(
        arg1: *mut c_types::c_void,
        arg2: *const c_types::c_void,
        arg3: c_types::c_ulong,
    ) -> c_types::c_ulong;
}

extern "C" fn my_read(
    arg1: *mut bindings::file,
    arg2: *mut c_types::c_char,
    arg3: usize,
    arg4: *mut bindings::loff_t,
) -> isize {
    unsafe {
        copy_to_user_wrapper(
            arg2 as *mut c_types::c_void,
            "y".as_ptr() as *const c_types::c_void,
            1,
        );
    }
    1
}

trait CharDevice: Sized {
    fn init(name: &'static str) -> linux_device_driver::KernelResult<Self>;
}

impl CharDevModule {
    fn register(&mut self) -> &mut Self {
        self.fops.read = Some(my_read);
        self
    }
    fn build(&mut self) -> i32 {
        self.major = unsafe {
            __register_chrdev(
                0,
                0,
                256,
                self.name.as_bytes().as_ptr() as *const i8,
                &*self.fops,
            )
        };
        self.major
    }
}

impl CharDevice for CharDevModule {
    fn init(name: &'static str) -> linux_device_driver::KernelResult<Self> {
        println!("Hello CharDev from Rust!");
        let fops = bindings::file_operations::default();
        Ok(CharDevModule {
            fops: Box::new(fops),
            major: 0,
            name: name,
        })
    }
}

impl Drop for CharDevModule {
    fn drop(&mut self) {
        println!("Goodbye CharDev from Rust!");
        unsafe {
            __unregister_chrdev(
                self.major as u32,
                0,
                256,
                self.name.as_bytes().as_ptr() as *const i8,
            )
        }
    }
}

static mut MODULE: Option<CharDevModule> = None;

#[no_mangle]
pub extern "C" fn init_module() -> c_types::c_int {
    match <CharDevModule as CharDevice>::init("yes\0") {
        Ok(mut m) => {
            m.register().build();
            unsafe {
                MODULE = Some(m);
            }
            return 0;
        }
        Err(_e) => {
            return 1;
        }
    }
}

#[no_mangle]
pub extern "C" fn cleanup_module() {
    unsafe {
        MODULE = None;
    }
}

#[link_section = ".modinfo"]
pub static MODINFO: [u8;12] = [108, 105, 099, 101, 110, 115, 101, 061, 071, 080, 076, 0];
