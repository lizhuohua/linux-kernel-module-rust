use core::mem;
use core::panic::PanicInfo;
pub use crate::bindings;
pub use crate::println;

extern "C" {
    fn bug_helper() -> !;
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
