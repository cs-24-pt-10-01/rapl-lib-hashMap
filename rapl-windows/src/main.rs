use anyhow::Result;
use std::ffi::CString;
use windows::{
    core::PCSTR,
    Win32::{
        Foundation::{CloseHandle, GENERIC_READ, HANDLE},
        Media::DeviceManager::IOCTL_MTP_CUSTOM_COMMAND,
        Security::{GetTokenInformation, TokenElevation, TOKEN_ELEVATION, TOKEN_QUERY},
        Storage::FileSystem::{CreateFileA, FILE_ATTRIBUTE_NORMAL, FILE_SHARE_READ, OPEN_EXISTING},
        System::{
            Threading::{GetCurrentProcess, OpenProcessToken},
            IO::DeviceIoControl,
        },
    },
};

const AMD_MSR_PWR_UNIT: u32 = 0xC0010299;
const AMD_MSR_CORE_ENERGY: u32 = 0xC001029A;
const AMD_MSR_PACKAGE_ENERGY: u32 = 0xC001029B;

/*
#define IOCTL_OLS_READ_MSR \
    CTL_CODE(OLS_TYPE, 0x821, METHOD_BUFFERED, FILE_ANY_ACCESS)
*/
const IOCTL_OLS_READ_MSR: u32 = 0x9C402084;

fn main() -> Result<()> {
    if !is_admin() {
        println!("please run this program as administrator");
        return Ok(());
    }

    let driver_name = CString::new("\\\\.\\WinRing0_1_2_0").expect("failed to create driver name");
    let h_device = unsafe {
        CreateFileA(
            PCSTR(driver_name.as_ptr() as *const u8), // File path
            GENERIC_READ.0,                           // Access mode (read-only in this example)
            FILE_SHARE_READ,                          // Share mode (0 for exclusive access)
            None,                                     // Security attributes (can be NULL)
            OPEN_EXISTING,                            // Creation disposition
            FILE_ATTRIBUTE_NORMAL,                    // File attributes (normal for regular files)
            None,                                     // Template file (not used here)
        )
    }
    .expect("failed to open driver");

    let input_data: [u8; 4] = [1, 2, 3, 4];
    unsafe { DeviceIoControl(h_device, IOCTL_OLS_READ_MSR, None, 0, None, 0, None, None) }
        .expect("failed to send IOCTL_MTP_CUSTOM_COMMAND");

    unsafe { CloseHandle(h_device) }.expect("failed to close driver handle");

    Ok(())
}

// check if running as admin using the windows crate
fn is_admin() -> bool {
    let mut h_token = HANDLE::default();
    unsafe { OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut h_token as _) }.unwrap();

    let mut token_elevation = TOKEN_ELEVATION { TokenIsElevated: 0 };
    let token_elevation_ptr = &mut token_elevation as *mut TOKEN_ELEVATION;
    let mut cb_size = std::mem::size_of::<TOKEN_ELEVATION>() as u32;

    unsafe {
        GetTokenInformation(
            h_token,
            TokenElevation,
            Some(token_elevation_ptr as _),
            cb_size,
            &mut cb_size as _,
        )
        .unwrap();
    }

    token_elevation.TokenIsElevated != 0
}
