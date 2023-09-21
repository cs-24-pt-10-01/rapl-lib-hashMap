use libc::getpid;

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
fn read_msr() {}
