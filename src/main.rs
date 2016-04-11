extern crate libc;

use std::ffi::CString;

#[repr(C)]
struct BuseOps {
    read: *mut extern fn(*mut libc::c_void, u32, u64, *mut libc::c_void) -> libc::c_int,
    write: *mut extern fn(*const libc::c_void, u32, u64, *mut libc::c_void) -> libc::c_int,
    disc: *mut extern fn(*mut libc::c_void),
    flush: *mut extern fn(*mut libc::c_void) -> libc::c_int,
    trim: *mut extern fn(u64, u32, *mut libc::c_void) -> libc::c_int,

    size: u64,
}

//#[link(name = "buse", kind = "static")]
extern {
    fn buse_main(dev_file: *const libc::c_char,
        aop: *const BuseOps,
        userdata: *mut libc::c_void)
        -> libc::c_int;
}

fn main() {
    println!("Hello, world!");
}
