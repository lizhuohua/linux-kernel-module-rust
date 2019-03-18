#![no_std]
#![feature(alloc)]

extern crate alloc;

use core::mem;
use crate::alloc::boxed::Box;
use crate::alloc::vec; // import vec! macro
use linux_device_driver::bindings;
use linux_device_driver::c_types;
use linux_device_driver::println;

struct USBDriver {
    table: Box<[bindings::usb_device_id]>,
    // driver must be stored in a box, why?
    driver: Box<bindings::usb_driver>,
}

impl USBDriver {
    fn register(name: &'static str) -> linux_device_driver::KernelResult<Self> {
        let mut tmp = bindings::usb_device_id::default();
        tmp.match_flags = 3;
        tmp.idVendor = 0x0424;
        tmp.idProduct = 0xec00;
        let mut table = vec![tmp, unsafe { mem::zeroed() }].into_boxed_slice();
        let mut driver = Box::new(bindings::usb_driver::default());
        driver.name = name.as_bytes().as_ptr() as *const i8;
        driver.id_table = table.as_mut_ptr();
        driver.probe = Some(bindings::usbnet_probe);
        driver.disconnect = Some(bindings::usbnet_disconnect);
        unsafe {
            bindings::usb_register_driver(
                driver.as_mut(),
                &mut bindings::__this_module,
                "simple_sysctl\0".as_bytes().as_ptr() as *const i8,
            );
        }
        Ok(USBDriver {
            table: table,
            driver: driver,
        })
    }
}

struct USBModule {
    usb: USBDriver,
}

impl linux_device_driver::KernelModule for USBModule {
    fn init() -> linux_device_driver::KernelResult<Self> {
        println!("Hello from Rust!");
        let usb = USBDriver::register("simple_usb\0")?;
        Ok(USBModule { usb: usb })
    }
}

impl Drop for USBModule {
    fn drop(&mut self) {
        println!("Goodbye from Rust!");
        unsafe {
            bindings::usb_deregister(self.usb.driver.as_mut());
        }
    }
}

static mut MODULE: Option<USBModule> = None;

#[no_mangle]
pub extern "C" fn init_module() -> c_types::c_int {
    match <USBModule as linux_device_driver::KernelModule>::init() {
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
