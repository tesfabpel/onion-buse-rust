extern crate libc;

use std::io::Write;
use std::ffi::CString;
use std::path::Path;
use std::path::PathBuf;

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

fn file_add_suffix(orig: &str, suffix: &str) -> *const Path {
    let path = Path::new(orig);
    let dirname = path.parent();
    let filename = path.file_name().unwrap();

    let mut pb = PathBuf::new();
    if dirname.is_some() {
        pb.push(dirname.unwrap());
    }
    pb.push(filename);
    pb.push(suffix);

    return pb.as_path();
}

fn print_usage() {
    writeln!(&mut std::io::stderr(),
        "Usage: onionbuse <ro file> [<snapshot file>] <virtual file>").unwrap();
}

fn main() {
    let original_file;
    let snapshot_file;
    let log_file;

    let buse_file;

    let argv: Vec<_> = std::env::args().collect();
    let argc = argv.len();

    if argc != 3
       && argc != 4 {
           print_usage();
           std::process::exit(-1);
    }

    if argc == 3 {
        original_file = argv[1].clone();
        buse_file = argv[2].clone();

        snapshot_file = "TODO".to_owned();
    }
    else if argc == 4 {
        original_file = argv[1].clone();
		snapshot_file = argv[2].clone();
		buse_file = argv[3].clone();
    }

    log_file = "TODO".to_owned();
}
