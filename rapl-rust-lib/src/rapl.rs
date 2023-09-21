// Impl this:
// https://github.com/greensoftwarelab/Energy-Languages/blob/master/RAPL/rapl.c#L14

use libc::getpid;

pub fn start_rapl_impl() -> usize {
    unsafe { getpid() };

    123
}

fn rapl_init() {}

fn detect_cpu() {}

fn open_msr() {}

fn read_msr() {}
