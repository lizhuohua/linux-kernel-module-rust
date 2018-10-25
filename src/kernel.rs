#![allow(non_camel_case_types)]

use crate::c_types;

pub type gfp_t = c_types::c_uint;
pub const GFP_KERNEL: gfp_t = 20971712;

extern "C" {
    pub fn krealloc(arg1: *const c_types::c_void, arg2: usize, arg3: gfp_t)
        -> *mut c_types::c_void;
}

extern "C" {
    pub fn kfree(arg1: *const c_types::c_void);
}
