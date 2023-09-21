use crate::rapl::start_rapl_impl;

#[no_mangle]
pub extern "C" fn start_rapl() -> usize {
    start_rapl_impl()
}

#[no_mangle]
pub extern "C" fn end_rapl() -> usize {
    456
}
