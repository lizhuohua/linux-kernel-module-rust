use crate::kernel_result::*;

pub trait KernelModule : Sized {
    fn init() -> KernelResult<Self>;
}
