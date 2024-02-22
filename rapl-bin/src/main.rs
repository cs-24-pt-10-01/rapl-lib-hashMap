use anyhow::{Ok, Result};
use rapl_lib::ffi::{start_rapl_rust, stop_rapl_rust};
use std::{thread, time::Duration};


fn main() -> Result<()> {
    // Call start_rapl() to initialize the RAPL driver on Windows
    const attack: u64 = 20;

    for i in 0..100 {
        for i in 0..attack {
            start_rapl_rust("test");
        }
        for i in 0..attack {
            stop_rapl_rust("test");
        }
        thread::sleep(Duration::from_millis(1));
    }
    Ok(())
}

/*
// AMD unit masks
let time_unit = ((output_number & AMD_TIME_UNIT_MASK) >> 16) as f64;
let energy_unit = ((output_number & AMD_ENERGY_UNIT_MASK) >> 8) as f64;
let power_unit = (output_number & AMD_POWER_UNIT_MASK) as f64;
println!(
    "time_unit: {}, energy_unit: {}, power_unit: {}",
    time_unit, energy_unit, power_unit
);

// AMD converted unit masks
let time_unit_d = time_unit.powf(0.5);
let energy_unit_d = energy_unit.powf(0.5);
let power_unit_d = power_unit.powf(0.5);
println!(
    "time_unit_d: {}, energy_unit_d: {}, power_unit_d: {}",
    time_unit_d, energy_unit_d, power_unit_d
);
*/
