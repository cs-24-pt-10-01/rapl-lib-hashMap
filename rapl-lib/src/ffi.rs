#[cfg(target_os = "linux")]
mod rapl_impl {
    pub use crate::rapl::linux::{start_rapl_impl, stop_rapl_impl};
}

#[cfg(target_os = "windows")]
mod rapl_impl {
    pub use crate::rapl::windows::{start_rapl_impl, stop_rapl_impl};
}

#[no_mangle]
pub extern "C" fn start_rapl() {
    rapl_impl::start_rapl_impl();
}

#[no_mangle]
pub extern "C" fn stop_rapl() {
    rapl_impl::stop_rapl_impl();
}
