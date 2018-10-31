#![no_std]

use linux_device_driver::bindings;
use linux_device_driver::c_types;
use linux_device_driver::println;

struct CharDevModule {
    major: c_types::c_int,
    name: &'static str,
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

static mut FOPS: Option<bindings::file_operations> = None;

unsafe fn get_fops() -> &'static mut bindings::file_operations {
    match FOPS {
        Some(ref mut x) => &mut *x,
        None => panic!(),
    }
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
    pub fn _copy_to_user(
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
        _copy_to_user(
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

impl CharDevice for CharDevModule {
    fn init(name: &'static str) -> linux_device_driver::KernelResult<Self> {
        println!("Hello CharDev from Rust!");
        unsafe {
            let mut tmp = bindings::file_operations::default();
            tmp.read = Some(my_read);
            FOPS = Some(tmp);
        }
        let major = unsafe {
            __register_chrdev(0, 0, 256, name.as_bytes().as_ptr() as *const i8, get_fops())
        };
        Ok(CharDevModule {
            major: major,
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
        Ok(m) => {
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
