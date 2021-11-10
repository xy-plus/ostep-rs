extern crate nix;

pub use nix::errno::Errno;
pub use nix::sys::wait::{self, WaitStatus};
pub use nix::unistd::{ForkResult, Pid};
use std::convert::Infallible;
use std::ffi::CStr;

pub fn fork() -> Result<ForkResult, Errno> {
    unsafe { nix::unistd::fork() }
}

pub fn getpid() -> Pid {
    nix::unistd::getpid()
}

pub fn wait() -> Result<WaitStatus, Errno> {
    nix::sys::wait::wait()
}

pub fn sleep(seconds: u32) -> u32 {
    nix::unistd::sleep(seconds)
}

pub fn execvp<S: AsRef<CStr>>(filename: &CStr, args: &[S]) -> Result<Infallible, Errno> {
    nix::unistd::execvp(filename, args)
}
