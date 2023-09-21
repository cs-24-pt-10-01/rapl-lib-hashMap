use std::mem::size_of;

use libc::{c_void, getpid, pread};

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
fn read_msr() -> i64 {
    let fd = 0;
    let which = 0;
    let data: i64 = 0;
    let dataaa = data as *mut c_void;

    unsafe { pread(fd, dataaa, size_of::<i64>(), which) };

    data
}

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_msr() {
        assert_eq!(read_msr(), 0);
    }
}
