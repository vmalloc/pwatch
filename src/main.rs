extern crate errno;
extern crate libc;

use std::ffi::CString;
use std::io::Write;
use std::path::{Path};
use libc::{c_char, execve, prctl, PR_SET_PDEATHSIG, SIGTERM};
use errno::errno;


fn resolve_executable(s: &str) -> String {

    let path = Path::new(s);

    if !path.is_absolute() {

        if let Some(paths) = std::env::var_os("PATH") {

            for dir in std::env::split_paths(&paths) {

                let filename = dir.join(&path);

                if filename.is_file() {
                    return filename.to_str().unwrap().to_owned();
                }

            }
        }
    }
    s.to_owned()
}


fn main() {

    let executable = std::env::args().skip(1).next().unwrap();

    let mut args: Vec<CString> = Vec::new();
    args.push(CString::new(
        resolve_executable(&executable)
    ).unwrap());
    args.extend(std::env::args()
                .skip(2)
                .map(|arg| CString::new(arg).unwrap())
                .collect::<Vec<CString>>());

    let mut c_args = args.iter()
        .map(|arg| arg.as_ptr())
        .collect::<Vec<*const c_char>>();
    c_args.push(std::ptr::null());

    let mut c_env: Vec<*const c_char> = Vec::new();
    c_env.push(std::ptr::null());

    unsafe {

        let retval = prctl(PR_SET_PDEATHSIG, SIGTERM);

        if retval != 0 {
            writeln!(std::io::stderr(), "Error from prctl: {}", errno()).unwrap();
            std::process::exit(1);
        }

        execve(c_args[0], c_args.as_ptr(), c_env.as_ptr());
        writeln!(std::io::stderr(), "Error executing program: {}", errno()).unwrap();
        std::process::exit(1);

    }
}
