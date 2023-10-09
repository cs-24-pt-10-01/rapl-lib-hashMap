use super::RaplError;
use once_cell::sync::OnceCell;
use std::{fs::File, os::unix::prelude::FileExt};

// Running it for now: sudo ./target/debug/rapl-bin

static CPU0_MSR_FD: OnceCell<File> = OnceCell::new();

pub fn start_rapl_impl() {}

// https://github.com/greensoftwarelab/Energy-Languages/blob/master/RAPL/rapl.c#L14
fn open_msr(core: u32) -> File {
    File::open(format!("/dev/cpu/{}/msr", core)).unwrap()
}

// https://github.com/greensoftwarelab/Energy-Languages/blob/master/RAPL/rapl.c#L38
pub fn read_msr(msr_offset: u64) -> Result<u64, RaplError> {
    let f = CPU0_MSR_FD.get_or_init(|| open_msr(0));

    let mut output_data: [u8; 8] = [0; 8];

    // TODO: Consider just seek here instead, same impl for Windows then
    f.read_at(&mut output_data, msr_offset).unwrap();

    Ok(u64::from_le_bytes(output_data))
}
