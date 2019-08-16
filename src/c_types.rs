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

#![allow(non_camel_case_types)]

pub type c_int = i32;
pub type c_char = i8;
// pub type c_long = i64;
pub type c_long = i32;
pub type c_longlong = i64;
pub type c_short = i16;
pub type c_uchar = u8;
pub type c_uint = u32;
// pub type c_ulong = u64;
pub type c_ulong = u32;
pub type c_ulonglong = u64;
pub type c_ushort = u16;
pub type c_schar = i8;

#[repr(u8)]
pub enum c_void {
    #[doc(hidden)]
    __nothing_to_see_here,
    #[doc(hidden)]
    __move_along,
}
