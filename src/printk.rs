use crate::c_types;

extern "C" {
    pub fn printk(fmt: *const c_types::c_char, ...) -> c_types::c_int;
}

#[macro_export]
macro_rules! println {
    () => {{
        unsafe {
            $crate::printk::printk("\n\0".as_bytes().as_ptr() as *const i8);
        }
    }};
    ($fmt:expr) => {{
        unsafe {
            $crate::printk::printk(concat!($fmt, "\n\0").as_bytes().as_ptr() as *const i8);
        }
    }};
}
