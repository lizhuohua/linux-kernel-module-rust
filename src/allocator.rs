use core::alloc::{GlobalAlloc, Layout};

use crate::c_types;
use crate::kernel;
// use crate::println;

pub struct KernelAllocator;

unsafe impl GlobalAlloc for KernelAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // println!("Rust is allocating kernel memeory...");
        return kernel::krealloc(
            0 as *const c_types::c_void,
            layout.size(),
            kernel::GFP_KERNEL,
        ) as *mut u8;
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        // println!("Rust is releasing kernel memeory...");
        kernel::kfree(ptr as *const c_types::c_void);
    }
}

#[lang = "oom"]
extern "C" fn oom(_err: Layout) -> ! {
    panic!("Out of memory!");
}
