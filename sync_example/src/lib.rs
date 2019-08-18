#![no_std]
#![feature(alloc)]
#![feature(const_fn)]
#![feature(min_const_fn)]

// pub mod sync;

extern crate alloc;
use crate::alloc::string::{String, ToString};
use lazy_static::*;
use linux_kernel_module::c_types;
use linux_kernel_module::println;
use linux_kernel_module::sync;
use linux_kernel_module::sync::Spinlock;

struct HelloWorldModule {
    message: String,
}

lazy_static! {
    static ref GLOBAL: Spinlock<i32> = Spinlock::new(0);
}

fn global_synchronization_example() {
    let mut global = GLOBAL.lock();
    *global = 1;
}

impl linux_kernel_module::KernelModule for HelloWorldModule {
    fn init() -> linux_kernel_module::KernelResult<Self> {
        global_synchronization_example();
        let spinlock_data = sync::Spinlock::new(100);
        println!("Data {} is locked by a spinlock", *spinlock_data.lock());
        let mutex_data = sync::Mutex::new(50);
        let mut data = mutex_data.lock();
        println!("Data {} is locked by a mutex", *data);
        *data = 100;
        println!("Now data is {}", *data);
        sync::drop(data);
        println!("Hello from Rust!");
        Ok(HelloWorldModule {
            message: "Hello World!".to_string(),
        })
    }
}

impl Drop for HelloWorldModule {
    fn drop(&mut self) {
        println!("My message is {}", self.message);
        println!("Goodbye from Rust!");
    }
}

static mut MODULE: Option<HelloWorldModule> = None;

#[no_mangle]
pub extern "C" fn init_module() -> c_types::c_int {
    match <HelloWorldModule as linux_kernel_module::KernelModule>::init() {
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
pub static MODINFO: [u8; 12] = *b"license=GPL\0";
