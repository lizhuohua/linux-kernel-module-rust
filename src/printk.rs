// Copyright (C) 2019 Alex Gaynor, Geoffrey Thomas, and other project authors
// 
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; either version 2 of the License, or
// (at your option) any later version.
// 
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
// 
// You should have received a copy of the GNU General Public License along
// with this program; if not, write to the Free Software Foundation, Inc.,
// 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.

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
