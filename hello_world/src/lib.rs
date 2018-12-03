#![no_std]
#![feature(alloc)]

extern crate alloc;
use crate::alloc::string::{String, ToString};
use linux_device_driver::c_types;
use linux_device_driver::println;

struct HelloWorldModule {
    message: String,
}

impl linux_device_driver::KernelModule for HelloWorldModule {
    fn init() -> linux_device_driver::KernelResult<Self> {
        println!("Hello from Rust!");
        Ok(HelloWorldModule {
            message: "Hello World!".to_string(),
        })
    }
}

impl Drop for HelloWorldModule {
    fn drop(&mut self) {
        println!("Goodbye from Rust!");
    }
}

static mut MODULE: Option<HelloWorldModule> = None;

#[no_mangle]
pub extern "C" fn init_module() -> c_types::c_int {
    match <HelloWorldModule as linux_device_driver::KernelModule>::init() {
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

#[link_section = ".modinfo"]
pub static MODINFO: [u8;12] = *b"license=GPL\0";
