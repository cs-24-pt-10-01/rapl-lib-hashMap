#[cfg(amd)]
use crate::rapl::windows::amd::{AMD_MSR_PACKAGE_ENERGY, AMD_MSR_PWR_UNIT};

#[cfg(intel)]
use crate::rapl::windows::intel::{MSR_RAPL_PKG, MSR_RAPL_POWER_UNIT};

use csv::{Writer, WriterBuilder};
use once_cell::sync::OnceCell;
use std::{
    ffi::CString,
    fs::{File, OpenOptions},
    sync::Once,
};
use thiserror::Error;
use windows::{
    core::PCSTR,
    Win32::{
        Foundation::{GENERIC_READ, HANDLE},
        Security::{GetTokenInformation, TokenElevation, TOKEN_ELEVATION, TOKEN_QUERY},
        Storage::FileSystem::{CreateFileA, FILE_ATTRIBUTE_NORMAL, FILE_SHARE_READ, OPEN_EXISTING},
        System::{
            Threading::{GetCurrentProcess, OpenProcessToken},
            IO::DeviceIoControl,
        },
    },
};

// RAPL Intel: https://github.com/tfett/RAPL/blob/master/rapwl-read.c
// RAPL AMD: https://me.sakana.moe/2023/09/06/measuring-cpu-power-consumption/
// Read MSR on Windows: https://github.com/LibreHardwareMonitor/LibreHardwareMonitor/blob/cada6b76b009105aadd9bb2821a7c4cae5cca431/WinRing0/OpenLibSys.c#L313
// Windows RAPL Driver: https://github.com/hubblo-org/windows-rapl-driver/tree/master

#[derive(Error, Debug)]
pub enum RaplError {
    #[error("windows error")]
    Windows(#[from] windows::core::Error),
}

/*
#define IOCTL_OLS_READ_MSR \
    CTL_CODE(OLS_TYPE, 0x821, METHOD_BUFFERED, FILE_ANY_ACCESS)
*/
const IOCTL_OLS_READ_MSR: u32 = 0x9C402084;

#[cfg(amd)]
mod amd {
    pub const AMD_MSR_PWR_UNIT: u32 = 0xC0010299;
    //const AMD_MSR_CORE_ENERGY: u32 = 0xC001029A;
    pub const AMD_MSR_PACKAGE_ENERGY: u32 = 0xC001029B;

    /*
    const AMD_TIME_UNIT_MASK: u64 = 0xF0000;
    const AMD_ENERGY_UNIT_MASK: u64 = 0x1F00;
    const AMD_POWER_UNIT_MASK: u64 = 0xF;
    */
}

#[cfg(intel)]
mod intel {
    pub const MSR_RAPL_POWER_UNIT: u32 = 0x606;
    pub const MSR_RAPL_PKG: u32 = 0x611;
    /*
    const MSR_RAPL_PP0: u32 = 0x639;
    const MSR_RAPL_PP1: u32 = 0x641;
    const MSR_RAPL_DRAM: u32 = 0x619;

    const INTEL_TIME_UNIT_MASK: u64 = 0xF000;
    const INTEL_ENGERY_UNIT_MASK: u64 = 0x1F00;
    const INTEL_POWER_UNIT_MASK: u64 = 0x0F;

    const INTEL_TIME_UNIT_OFFSET: u64 = 0x10;
    const INTEL_ENGERY_UNIT_OFFSET: u64 = 0x08;
    const INTEL_POWER_UNIT_OFFSET: u64 = 0;
    */
}

static mut RAPL_START: u64 = 0;
//static RAPL_STOP: AtomicU64 = AtomicU64::new(0);

static RAPL_INIT: Once = Once::new();
static RAPL_DRIVER: OnceCell<HANDLE> = OnceCell::new();
static RAPL_POWER_UNITS: OnceCell<u64> = OnceCell::new();

static mut CSV_WRITER: Option<Writer<File>> = None;

fn read_rapl_power_unit() -> Result<u64, RaplError> {
    #[cfg(intel)]
    {
        read_msr(MSR_RAPL_POWER_UNIT)
    }

    #[cfg(amd)]
    {
        read_msr(AMD_MSR_PWR_UNIT)
    }
}

fn read_rapl_pkg_energy_stat() -> Result<u64, RaplError> {
    #[cfg(intel)]
    {
        read_msr(MSR_RAPL_PKG)
    }

    #[cfg(amd)]
    {
        read_msr(AMD_MSR_PACKAGE_ENERGY)
    }
}

fn get_cpu_type() -> &'static str {
    #[cfg(intel)]
    {
        "Intel"
    }

    #[cfg(amd)]
    {
        "AMD"
    }
}

pub fn start_rapl_impl() {
    // Initialize RAPL driver on first call
    RAPL_INIT.call_once(|| {
        // Check if running as admin due to driver requirement
        if !is_admin() {
            panic!("not running as admin");
        }

        let h_device = open_driver()
            .expect("failed to open driver handle, make sure the driver is installed and running");
        RAPL_DRIVER.get_or_init(|| h_device);

        // Read power unit and store it the power units variable
        let pwr_unit = read_rapl_power_unit().expect("failed to read RAPL power unit");
        RAPL_POWER_UNITS.get_or_init(|| pwr_unit);
    });

    // Read MSR based on the processor type
    let rapl_pkg_energy_start_val =
        read_rapl_pkg_energy_stat().expect("failed to read pkg energy stat");

    // Safety: RAPL_START is only accessed in this function and only from a single thread
    unsafe { RAPL_START = rapl_pkg_energy_start_val };
}

// Get all drivers: sc query type=driver
// Stop manually in CMD: sc stop R0LibreHardwareMonitor
// Delete manually in CMD: sc delete R0LibreHardwareMonitor

pub fn stop_rapl_impl() {
    // Read the RAPL PKG end value
    let rapl_pkg_energy_end_val =
        read_rapl_pkg_energy_stat().expect("failed to read pkg energy stat");

    // Load in the RAPL start value
    // Safety: RAPL_START is only accessed in this function and only from a single thread
    let rapl_start_val = unsafe { RAPL_START };

    let cpu_type = get_cpu_type();

    // Open the file to write to CSV. First argument is CPU type, second is RAPL power units

    /*
    // TODO: Revise if we can even use timestamps

    let current_time = SystemTime::now();
    let duration_since_epoch = current_time
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let timestamp = duration_since_epoch.as_millis();
    */

    let wtr = match unsafe { CSV_WRITER.as_mut() } {
        Some(wtr) => wtr,
        None => {
            // Open the file to write to CSV. First argument is CPU type, second is RAPL power units
            let file = OpenOptions::new()
                .append(true)
                .create(true)
                .open(format!(
                    "{}_{}.csv",
                    cpu_type,
                    RAPL_POWER_UNITS.get().unwrap()
                ))
                .unwrap();

            // Create the CSV writer
            let mut wtr = WriterBuilder::new().from_writer(file);
            wtr.write_record(["PkgStart", "PkgEnd"]).unwrap();

            // Store the CSV writer in a static variable
            unsafe { CSV_WRITER = Some(wtr) };

            // Return a mutable reference to the CSV writer
            unsafe { CSV_WRITER.as_mut().unwrap() }
        }
    };

    wtr.serialize((rapl_start_val, rapl_pkg_energy_end_val))
        .unwrap();
    wtr.flush().unwrap();
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

fn open_driver() -> Result<HANDLE, RaplError> {
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

fn read_msr(msr: u32) -> Result<u64, RaplError> {
    // Get the driver handle
    let rapl_driver = *RAPL_DRIVER.get().expect("RAPL driver not initialized");

    // Convert the MSR to a little endian byte array
    let input_data: [u8; 4] = msr.to_le_bytes();

    // Create an empty byte array to store the output
    let output_data: [u8; 8] = [0; 8];
    let mut lp_bytes_returned: u32 = 0;

    // Call the driver to read the MSR
    unsafe {
        DeviceIoControl(
            rapl_driver,
            IOCTL_OLS_READ_MSR,
            Some(input_data.as_ptr() as _),
            input_data.len() as u32,
            Some(output_data.as_ptr() as _),
            output_data.len() as u32,
            Some(&mut lp_bytes_returned as _),
            None,
        )
    }?;

    // TODO: Consider using lp_bytes_returned for error handling or logging it, it is supposed to return 8 bytes on success
    //println!("lp_bytes_returned: {}", lp_bytes_returned);
    Ok(u64::from_le_bytes(output_data))
}

/*
// Experimental. This was not a great success because Windows takes too long deleting + recreating the driver
// TODO: Consider documenting this or revisiting it later

fn install_driver() -> Result<(), RaplError> {
    let scm =
        unsafe { OpenSCManagerA(PCSTR::null(), PCSTR::null(), SC_MANAGER_ALL_ACCESS) }.unwrap();

    let driver_name = CString::new("R0LibreHardwareMonitor").expect("failed to create driver name");
    let driver_path =
        CString::new("C:\\Users\\Jakob\\Documents\\GitHub\\cs-23-pt-9-01\\rapl-rust-test\\LibreHardwareMonitor.sys").expect("failed to create driver path");

    let created_driver_service = unsafe {
        CreateServiceA(
            scm,
            PCSTR(driver_name.as_ptr() as *const u8),
            PCSTR(driver_name.as_ptr() as *const u8),
            SERVICE_ALL_ACCESS,
            SERVICE_KERNEL_DRIVER,
            SERVICE_DEMAND_START,
            SERVICE_ERROR_NORMAL,
            PCSTR(driver_path.as_ptr() as *const u8),
            None,
            None,
            None,
            None,
            None,
        )
    }
    .unwrap();

    unsafe { StartServiceA(created_driver_service, None) }.unwrap();

    unsafe { CloseServiceHandle(created_driver_service) }.unwrap();
    unsafe { CloseServiceHandle(scm) }.unwrap();

    Ok(())
}

fn stop_and_delete_driver() -> Result<(), RaplError> {
    let driver_name = CString::new("R0LibreHardwareMonitor").expect("failed to create driver name");
    let scm =
        unsafe { OpenSCManagerA(PCSTR::null(), PCSTR::null(), SC_MANAGER_ALL_ACCESS) }.unwrap();

    if let Ok(driverr) = unsafe {
        OpenServiceA(
            scm,
            PCSTR(driver_name.as_ptr() as *const u8),
            SERVICE_ALL_ACCESS,
        )
    } {
        // Stop the driver
        let mut service_status: SERVICE_STATUS = Default::default();
        unsafe {
            ControlService(
                driverr,
                SERVICE_CONTROL_STOP,
                &mut service_status as *mut SERVICE_STATUS,
            )
        }
        .unwrap();

        unsafe { DeleteService(driverr) }.unwrap();
        unsafe { CloseServiceHandle(driverr) }.unwrap();
    }
    unsafe { CloseServiceHandle(scm) }.unwrap();

    Ok(())
}
*/

// TODO: Install driver ourselves: https://github.com/LibreHardwareMonitor/LibreHardwareMonitor/blob/cada6b76b009105aadd9bb2821a7c4cae5cca431/LibreHardwareMonitorLib/Hardware/KernelDriver.cs#L40
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
