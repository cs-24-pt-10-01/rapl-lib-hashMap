use anyhow::Result;
use std::ffi::CString;
use sysinfo::{CpuExt, System, SystemExt};
use windows::{
    core::PCSTR,
    Win32::{
        Foundation::{CloseHandle, GENERIC_READ, HANDLE},
        Security::{GetTokenInformation, TokenElevation, TOKEN_ELEVATION, TOKEN_QUERY},
        Storage::FileSystem::{CreateFileA, FILE_ATTRIBUTE_NORMAL, FILE_SHARE_READ, OPEN_EXISTING},
        System::{
            Threading::{GetCurrentProcess, OpenProcessToken},
            IO::DeviceIoControl,
        },
    },
};

// RAPL Intel: https://github.com/tfett/RAPL/blob/master/rapl-read.c
// RAPL AMD: https://me.sakana.moe/2023/09/06/measuring-cpu-power-consumption/
// Read MSR on Windows: https://github.com/LibreHardwareMonitor/LibreHardwareMonitor/blob/cada6b76b009105aadd9bb2821a7c4cae5cca431/WinRing0/OpenLibSys.c#L313
// Windows RAPL Driver: https://github.com/hubblo-org/windows-rapl-driver/tree/master

// AMD
const AMD_MSR_PWR_UNIT: u32 = 0xC0010299;
const AMD_MSR_CORE_ENERGY: u32 = 0xC001029A;
const AMD_MSR_PACKAGE_ENERGY: u32 = 0xC001029B;

const AMD_TIME_UNIT_MASK: u64 = 0xF0000;
const AMD_ENERGY_UNIT_MASK: u64 = 0x1F00;
const AMD_POWER_UNIT_MASK: u64 = 0xF;

// Intel
const MSR_RAPL_POWER_UNIT: u32 = 0x606;

/*
#define IOCTL_OLS_READ_MSR \
    CTL_CODE(OLS_TYPE, 0x821, METHOD_BUFFERED, FILE_ANY_ACCESS)
*/
const IOCTL_OLS_READ_MSR: u32 = 0x9C402084;

/*
Sample with making driver service and starting it:

#include <windows.h>

int main() {
    SC_HANDLE scm, service;

    scm = OpenSCManager(NULL, NULL, SC_MANAGER_ALL_ACCESS);
    if (scm == NULL) {
        // Handle error
        return 1;
    }

    service = CreateService(scm, L"YourDriverName", L"Your Driver Display Name",
        SERVICE_ALL_ACCESS, SERVICE_KERNEL_DRIVER, SERVICE_DEMAND_START, SERVICE_ERROR_NORMAL,
        L"Path to your driver file", NULL, NULL, NULL, NULL, NULL);

    if (service == NULL) {
        // Handle error
        CloseServiceHandle(scm);
        return 2;
    }

    StartService(service, 0, NULL);

    CloseServiceHandle(service);
    CloseServiceHandle(scm);

    return 0;
}
*/

fn open_driver() -> Result<HANDLE> {
    let driver_name = CString::new("\\\\.\\WinRing0_1_2_0").expect("failed to create driver name");
    Ok(unsafe {
        CreateFileA(
            PCSTR(driver_name.as_ptr() as *const u8), // File path
            GENERIC_READ.0,                           // Access mode (read-only in this example)
            FILE_SHARE_READ,                          // Share mode (0 for exclusive access)
            None,                                     // Security attributes (can be None)
            OPEN_EXISTING,                            // Creation disposition
            FILE_ATTRIBUTE_NORMAL,                    // File attributes (normal for regular files)
            None,                                     // Template file (not used here)
        )
    }?)
}

fn read_msr(h_device: HANDLE, msr: u32) -> Result<u64> {
    let input_data: [u8; 4] = msr.to_le_bytes();

    let output_data: [u8; 8] = [0; 8];
    let mut lp_bytes_returned: u32 = 0;
    unsafe {
        DeviceIoControl(
            h_device,
            IOCTL_OLS_READ_MSR,
            Some(input_data.as_ptr() as _),
            input_data.len() as u32,
            Some(output_data.as_ptr() as _),
            output_data.len() as u32,
            Some(&mut lp_bytes_returned as _),
            None,
        )
    }?;

    println!("lp_bytes_returned: {}", lp_bytes_returned);
    Ok(u64::from_le_bytes(output_data))
}

fn main() -> Result<()> {
    // TODO: Logging, multiple cores (maybe only possible to read all cores at once, although Linux seems to have multiple since MSR for each CPU), multiple CPU support (Intel)
    if !is_admin() {
        eprintln!("this program must run as administrator");
        return Ok(());
    }

    let sys = System::new_all();
    match sys.cpus().first().expect("failed getting CPU").vendor_id() {
        "GenuineIntel" => println!("Intel CPU detected"),
        "AuthenticAMD" => println!("AMD CPU detected"),
        _ => {
            println!("unknown CPU detected");
            return Ok(());
        }
    }

    // TODO: Install driver ourselves: https://github.com/LibreHardwareMonitor/LibreHardwareMonitor/blob/cada6b76b009105aadd9bb2821a7c4cae5cca431/LibreHardwareMonitorLib/Hardware/KernelDriver.cs#L40
    let h_device = open_driver().expect("failed to open driver handle");

    let output_number = read_msr(h_device, AMD_MSR_PWR_UNIT).expect("failed to read MSR register");
    println!("output_number: {}", output_number);

    let time_unit = ((output_number & AMD_TIME_UNIT_MASK) >> 16) as f64;
    let energy_unit = ((output_number & AMD_ENERGY_UNIT_MASK) >> 8) as f64;
    let power_unit = (output_number & AMD_POWER_UNIT_MASK) as f64;
    println!(
        "time_unit: {}, energy_unit: {}, power_unit: {}, absolute_unit: 69420",
        time_unit, energy_unit, power_unit
    );

    let time_unit_d = time_unit.powf(0.5);
    let energy_unit_d = energy_unit.powf(0.5);
    let power_unit_d = power_unit.powf(0.5);

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
