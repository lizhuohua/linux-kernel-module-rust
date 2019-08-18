#![no_std]
#![feature(alloc)]

extern crate alloc;

use crate::alloc::boxed::Box;
use crate::alloc::vec; // import vec! macro
use core::mem;
use linux_kernel_module::bindings;
use linux_kernel_module::c_types;
use linux_kernel_module::println;

struct Sysctl {
    // We must store `table` here, otherwise it will be dropped after `register`. It would be a
    // use-after-free
    table: Box<[bindings::ctl_table]>,
    header: *mut bindings::ctl_table_header,
}

struct SysctlModule {
    sysctl: Sysctl,
}

extern "C" {
    pub fn register_sysctl(
        path: *const c_types::c_char,
        table: *mut bindings::ctl_table,
    ) -> *mut bindings::ctl_table_header;
}

extern "C" {
    pub fn unregister_sysctl_table(table: *mut bindings::ctl_table_header);
}

extern "C" {
    pub fn proc_dointvec_minmax(
        arg1: *mut bindings::ctl_table,
        arg2: c_types::c_int,
        arg3: *mut c_types::c_void,
        arg4: *mut usize,
        arg5: *mut bindings::loff_t,
    ) -> c_types::c_int;
}

static mut DATA: i32 = 1;
static mut ZERO: i32 = 0;
static mut TWO: i32 = 2;

impl Sysctl {
    fn register(path: &'static str, name: &'static str) -> linux_kernel_module::KernelResult<Self> {
        let mut tmp = bindings::ctl_table::default();
        tmp.procname = name.as_bytes().as_ptr() as *const i8;
        tmp.maxlen = mem::size_of::<i32>() as i32;
        tmp.mode = 0o644;
        tmp.data = unsafe { &mut DATA as *mut i32 as *mut c_types::c_void };
        tmp.proc_handler = Some(proc_dointvec_minmax);
        tmp.extra1 = unsafe { &mut ZERO as *mut i32 as *mut c_types::c_void };
        tmp.extra2 = unsafe { &mut TWO as *mut i32 as *mut c_types::c_void };
        let mut table = vec![tmp, unsafe { mem::zeroed() }].into_boxed_slice();
        //println!(
        //"procname={}, data={}, maxlen={}, mode={}, proc_handler={:?}, extra1={}, extra2={}, addr of table={:?}, addr of table[0]={:?}",
        //unsafe { *table[0].procname },
        //unsafe { *(table[0].data as *mut i32) },
        //table[0].maxlen,
        //table[0].mode,
        //table[0].proc_handler.unwrap(),
        //unsafe { *(table[0].extra1 as *mut i32) },
        //unsafe { *(table[0].extra2 as *mut i32) },
        //table.as_ptr(),
        //&table[0] as *const bindings::ctl_table as *const i32,
        //);
        let header =
            unsafe { register_sysctl(path.as_bytes().as_ptr() as *const i8, table.as_mut_ptr()) };
        if header as i32 == 0 {
            println!("Error while registring sysctl");
        }
        Ok(Sysctl {
            table: table,
            header: header,
        })
    }
}

impl linux_kernel_module::KernelModule for SysctlModule {
    fn init() -> linux_kernel_module::KernelResult<Self> {
        println!("Hello from Rust!");
        let sysctl = Sysctl::register("rust/example\0", "test\0")?;
        Ok(SysctlModule { sysctl: sysctl })
    }
}

impl Drop for SysctlModule {
    fn drop(&mut self) {
        println!("Goodbye from Rust!");
        // println!("now header={}", self.sysctl.header as u32);
        unsafe {
            bindings::unregister_sysctl_table(self.sysctl.header);
        }
    }
}

static mut MODULE: Option<SysctlModule> = None;

#[no_mangle]
pub extern "C" fn init_module() -> c_types::c_int {
    match <SysctlModule as linux_kernel_module::KernelModule>::init() {
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

#[no_mangle]
#[link_section = ".modinfo"]
pub static MODINFO: [u8; 12] = *b"license=GPL\0";
