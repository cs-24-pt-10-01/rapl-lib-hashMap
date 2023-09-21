use libc::{c_void, getpid, open, perror, pread, EIO, ENXIO, O_RDONLY};
use std::{ffi::CString, mem::size_of};

// Impl this:
// https://github.com/greensoftwarelab/Energy-Languages/blob/master/RAPL/rapl.c#L14

pub fn start_rapl_impl() -> usize {
    unsafe { getpid() };

    123
}

// https://github.com/greensoftwarelab/Energy-Languages/blob/master/RAPL/rapl.c#L157
fn rapl_init() {}

// https://github.com/greensoftwarelab/Energy-Languages/blob/master/RAPL/rapl.c#L64
fn detect_cpu() {}

// https://github.com/greensoftwarelab/Energy-Languages/blob/master/RAPL/rapl.c#L14
fn open_msr(core: i32) -> i32 {
    let path = CString::new(format!("/dev/cpu/{}/msr", core)).unwrap();
    let fd = unsafe { open(path.as_ptr(), O_RDONLY) };

    println!("fd: {}", fd);

    if fd < 0 {
        let errno = unsafe { *libc::__errno_location() };
        if errno == ENXIO {
            println!("rdmsr: No CPU {}", core);
        } else if errno == EIO {
            println!("rdmsr: CPU {} doesn't support MSRs", core);
        } else {
            let pread_err = CString::new("rdmsr:open").unwrap();
            unsafe { perror(pread_err.as_ptr()) };
        }
    }
    fd
}

// https://github.com/greensoftwarelab/Energy-Languages/blob/master/RAPL/rapl.c#L38
fn read_msr(fd: i32, which: i64) -> i64 {
    let data: i64 = 0;
    let data_ptr = data as *mut c_void;

    if unsafe { pread(fd, data_ptr, size_of::<i64>(), which) } != size_of::<i64>() as isize {
        let pread_err = CString::new("rdmsr:pread").unwrap();
        unsafe { perror(pread_err.as_ptr()) };
    }

    //println!("val: {}", val);

    data
}

const MSR_RAPL_POWER_UNIT: i64 = 0x606;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_msr() {
        let fd = open_msr(0);

        let result = read_msr(fd, MSR_RAPL_POWER_UNIT);

        assert_eq!(result, 1234);
    }
}
