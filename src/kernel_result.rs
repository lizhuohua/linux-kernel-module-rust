//use failure::Error;

//#[derive(Debug, Fail)]
pub enum KernelError {
}

pub type KernelResult<T> = Result<T, KernelError>;
