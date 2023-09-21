#[no_mangle]
pub extern "C" fn start_rapl() -> usize {
    start_rapl_impl()
}

fn start_rapl_impl() -> usize {
    123
}

#[no_mangle]
pub extern "C" fn end_rapl() -> usize {
    456
}
