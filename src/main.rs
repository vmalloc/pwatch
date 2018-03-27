extern crate errno;
extern crate libc;

use std::ffi::CString;
use std::io::Write;
use libc::{c_char, execve, prctl, PR_SET_PDEATHSIG, SIGTERM};
use errno::errno;

fn main() {
    let args = std::env::args()
        .skip(1)
        .map(|arg| CString::new(arg).unwrap())
        .collect::<Vec<CString>>();

    let mut c_args = args.iter()
        .map(|arg| arg.as_ptr())
        .collect::<Vec<*const c_char>>();
    c_args.push(std::ptr::null());

    let mut c_env: Vec<*const c_char> = Vec::new();
    c_env.push(std::ptr::null());

    unsafe {

        let retval = prctl(PR_SET_PDEATHSIG, SIGTERM);

        if retval != 0 {
            writeln!(std::io::stderr(), "Error from prctl: {}", errno());
            std::process::exit(1);
        }

        execve(c_args[0], c_args.as_ptr(), c_env.as_ptr());
        writeln!(std::io::stderr(), "Error executing program: {}", errno());
        std::process::exit(1);

    }
}
