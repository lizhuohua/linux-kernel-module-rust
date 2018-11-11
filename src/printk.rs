use core::{cmp, fmt};
use crate::c_types;

extern "C" {
    pub fn printk(fmt: *const c_types::c_char, ...) -> c_types::c_int;
}

const LOG_LINE_MAX: usize = 1024 - 32;

pub struct LogLineWriter {
    data: [u8; LOG_LINE_MAX],
    pos: usize,
}

impl LogLineWriter {
    pub fn new() -> LogLineWriter {
        LogLineWriter {
            data: [0u8; LOG_LINE_MAX],
            pos: 0,
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        return &self.data[..self.pos];
    }
}

impl fmt::Write for LogLineWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let copy_len = cmp::min(LOG_LINE_MAX - self.pos, s.as_bytes().len());
        self.data[self.pos..self.pos + copy_len].copy_from_slice(&s.as_bytes()[..copy_len]);
        self.pos += copy_len;
        return Ok(());
    }
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
    ($fmt:expr, $($arg:tt)*) => ({
        use ::core::fmt;
        let mut writer = $crate::printk::LogLineWriter::new();
        let _ = fmt::write(&mut writer, format_args!(concat!($fmt, "\n\0"), $($arg)*)).unwrap();
        unsafe {
            $crate::printk::printk(writer.as_bytes().as_ptr() as *const i8);
        }
    });
}
