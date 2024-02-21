use crate::rapl;
use std::ffi::{c_char, CStr};


#[no_mangle]
pub extern "C" fn start_rapl(id: *const c_char) {
    let rust_str = unsafe { CStr::from_ptr(id).to_str().unwrap() };
    rapl::start_rapl(rust_str);
}

#[no_mangle]
pub extern "C" fn stop_rapl(id: *const c_char) {
    let rust_str = unsafe { CStr::from_ptr(id).to_str().unwrap() };
    rapl::stop_rapl(rust_str);
}


// Rust specific calls:

pub fn start_rapl_rust(id: &str){
    rapl::start_rapl(id);
}

pub fn stop_rapl_rust(id: &str){
    rapl::stop_rapl(id);
}
