use anyhow::Result;
use std::ffi::CString;
use windows::{
    core::PCSTR,
    Win32::{
        Foundation::{GENERIC_READ, HANDLE},
        Storage::FileSystem::{
            CreateFileA, ReadFile, FILE_ATTRIBUTE_NORMAL, FILE_GENERIC_READ, FILE_SHARE_READ,
            OPEN_EXISTING,
        },
    },
};

const AMD_MSR_PWR_UNIT: u32 = 0xC0010299;
const AMD_MSR_CORE_ENERGY: u32 = 0xC001029A;
const AMD_MSR_PACKAGE_ENERGY: u32 = 0xC001029B;

fn main() -> Result<()> {
    let test_file = CString::new("testy").unwrap();

    let hFile = unsafe {
        CreateFileA(
            PCSTR(test_file.as_ptr() as *const u8), // File path
            FILE_GENERIC_READ.0,                    // Access mode (read-only in this example)
            FILE_SHARE_READ,                        // Share mode (0 for exclusive access)
            None,                                   // Security attributes (can be NULL)
            OPEN_EXISTING,                          // Creation disposition
            FILE_ATTRIBUTE_NORMAL,                  // File attributes (normal for regular files)
            None,                                   // Template file (not used here)
        )
    }
    .unwrap();

    let aweraer = HANDLE(123);
    unsafe { ReadFile(aweraer, None, None, None).unwrap() };

    Ok(())
}
