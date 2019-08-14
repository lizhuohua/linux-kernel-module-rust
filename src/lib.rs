#![no_std]
#![feature(lang_items, allocator_api)]

pub mod allocator;
pub mod c_types;
pub mod kernel;
pub mod kernel_module;
pub mod kernel_result;
pub mod printk;
pub mod bindings;
pub mod sync;
// pub mod panic;

pub use self::kernel_module::KernelModule;
pub use self::kernel_result::KernelResult;
pub use self::kernel_result::KernelError;

#[global_allocator]
static ALLOCATOR: allocator::KernelAllocator = allocator::KernelAllocator;

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

#[no_mangle]
pub extern "C" fn __aeabi_unwind_cpp_pr0() {}
