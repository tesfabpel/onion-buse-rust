extern crate libc;

use std::ptr;
use std::io::prelude::*;
use std::fs::File;
use std::fs::OpenOptions;
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

impl Default for BuseOps {
    fn default() -> BuseOps {
        BuseOps {
            read: ptr::null_mut(),
            write: ptr::null_mut(),
            disc: ptr::null_mut(),
            flush: ptr::null_mut(),
            trim: ptr::null_mut(),
            size: 0
        }
    }
}

//#[link(name = "buse", kind = "static")]
extern {
    fn buse_main(dev_file: *const libc::c_char,
        aop: *const BuseOps,
        userdata: *mut libc::c_void)
        -> libc::c_int;
}

fn file_add_suffix(path: &Path, suffix: &str) -> PathBuf {
    let dirname = path.parent();
    let filename = &path.file_name().unwrap().to_string_lossy().to_owned();

    let mut pb = PathBuf::new();
    if dirname.is_some() {
        let dn = dirname.unwrap();
        let dn2 = dn.to_string_lossy();
        if !(dn2.is_empty()) {
            pb.push(dn);
        }
    }
    else {
        pb.push(".");
    }
    pb.push(filename.to_string()+suffix);

    //return Path::new(Box::new(pb.into_os_string().into_string().unwrap()));
    return pb;
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
        original_file = PathBuf::from(&argv[1]);
        buse_file = PathBuf::from(&argv[2]);

        snapshot_file = file_add_suffix(&original_file, ".snap");
    }
    else if argc == 4 {
        original_file = PathBuf::from(&argv[1]);
		snapshot_file = PathBuf::from(&argv[2]);
		buse_file = PathBuf::from(&argv[3]);
    }
    else {
        panic!();
    }

    log_file = file_add_suffix(&snapshot_file, ".snap");

    let _ofd = OpenOptions::new()
        .read(true)
        .open(original_file);

    let ofd;
    match _ofd {
        Result::Ok(val) => ofd = val,
        Result::Err(err) =>
          panic!("ofd: {:?}", err),
    }

    let _sfd = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(snapshot_file);

    let sfd;
    match _sfd {
        Result::Ok(val) => sfd = val,
        Result::Err(err) =>
          panic!("ofd: {:?}", err),
    }

    let _lfd = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(log_file);

    let lfd;
    match _lfd {
        Result::Ok(val) => lfd = val,
        Result::Err(err) =>
          panic!("ofd: {:?}", err),
    }

    let bops = BuseOps {
        .. Default::default()
    };
    let buse_file_c = buse_file.to_string_lossy().to_mut().as_ptr() as *const i8;
    unsafe {
        let res = buse_main(buse_file_c, &bops, ptr::null_mut());
    }
}
