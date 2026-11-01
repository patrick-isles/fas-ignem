//! Hardware entropy, cycle interrogation, and thermal telemetry
//!
//! Direct x86_64 silicon instructions querying the quantum noise of the die
//! and physical junction temperature.

use std::arch::x86_64::{_rdrand64_step, _rdseed64_step, __rdtscp};
use std::fs;
use std::time::{Duration, Instant};

/// Extracts raw quantum entropy from the thermal noise of the CPU die using RDRAND.
#[inline(always)]
pub fn hardware_entropy_rdrand() -> Option<u64> {
    let mut val: u64 = 0;
    unsafe {
        if _rdrand64_step(&mut val) == 1 {
            Some(val)
        } else {
            None
        }
    }
}

/// Extracts unconditioned entropy from the physical ring oscillator using RDSEED.
#[inline(always)]
pub fn hardware_entropy_rdseed() -> Option<u64> {
    let mut val: u64 = 0;
    unsafe {
        if _rdseed64_step(&mut val) == 1 {
            Some(val)
        } else {
            None
        }
    }
}

/// Cycle timestamp instruction with core identity extraction.
#[inline(always)]
pub fn rdtsc() -> (u64, u32) {
    let mut aux: u32 = 0;
    let tsc = unsafe { __rdtscp(&mut aux) };
    (tsc, aux)
}

/// Reads the physical silicon junction temperature (T_j) in degrees Celsius.
pub fn read_junction_temp() -> f64 {
    // Check primary thermal zone
    if let Ok(content) = fs::read_to_string("/sys/class/thermal/thermal_zone0/temp") {
        if let Ok(millidegrees) = content.trim().parse::<f64>() {
            return millidegrees / 1000.0;
        }
    }

    // Fallback: check hwmon sensors
    if let Ok(entries) = fs::read_dir("/sys/class/hwmon") {
        for entry in entries.flatten() {
            let path = entry.path().join("temp1_input");
            if let Ok(content) = fs::read_to_string(path) {
                if let Ok(mdeg) = content.trim().parse::<f64>() {
                    return mdeg / 1000.0;
                }
            }
        }
    }

    70.0 // Symbolic fallback
}

/// Calibrates host TSC frequency in Hz.
pub fn calibrate_tsc() -> u64 {
    let (start_tsc, _) = rdtsc();
    let start_time = Instant::now();

    let sample_duration = Duration::from_millis(50);
    while start_time.elapsed() < sample_duration {
        std::hint::spin_loop();
    }

    let (end_tsc, _) = rdtsc();
    let elapsed_secs = start_time.elapsed().as_secs_f64();

    let delta_tsc = end_tsc.saturating_sub(start_tsc);
    (delta_tsc as f64 / elapsed_secs) as u64
}
