#![no_std]
#![feature(lang_items, allocator_api)]

pub mod allocator;
pub mod c_types;
pub mod kernel;
pub mod kernel_module;
pub mod kernel_result;
pub mod printk;

pub use self::kernel_module::KernelModule;
pub use self::kernel_result::KernelResult;
pub use self::printk::*;

#[global_allocator]
static ALLOCATOR: allocator::KernelAllocator = allocator::KernelAllocator;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

#[no_mangle]
pub extern "C" fn __aeabi_unwind_cpp_pr0() {}
