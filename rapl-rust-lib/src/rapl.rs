use libc::{c_void, getpid, perror, pread};
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
fn open_msr() {}

// https://github.com/greensoftwarelab/Energy-Languages/blob/master/RAPL/rapl.c#L38
fn read_msr(fd: i32, which: i64) -> i64 {
    let data: i64 = 0;
    let data_ptr = data as *mut c_void;

    if unsafe { pread(fd, data_ptr, size_of::<i64>(), which) } != size_of::<i64>() as isize {
        let ayy = CString::new("rdmsr:pread").unwrap();
        unsafe { perror(ayy.as_ptr()) };
    }

    //println!("val: {}", val);

    data
}

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

const MSR_RAPL_POWER_UNIT: i64 = 0x606;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_msr() {
        let fd = 0;
        let result = read_msr(fd, MSR_RAPL_POWER_UNIT);

        assert_eq!(result, 1234);
    }
}
