//! The Architecture of Pure Expenditure (*Dépense*)
//!
//! Spawns unyielding liturgical worker threads that burn CPU cycles,
//! extract quantum entropy from the die, and repeatedly provoke hardware boundary transgressions.

use crate::hardware::{hardware_entropy_rdrand, hardware_entropy_rdseed, rdtsc};
use crate::transgression::TransgressionAltar;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::thread;

pub struct CoreDepenseMetrics {
    pub core_id: usize,
    pub iterations: AtomicU64,
    pub entropy_harvested: AtomicU64,
    pub total_cycles: AtomicU64,
}

impl CoreDepenseMetrics {
    pub fn new(core_id: usize) -> Self {
        Self {
            core_id,
            iterations: AtomicU64::new(0),
            entropy_harvested: AtomicU64::new(0),
            total_cycles: AtomicU64::new(0),
        }
    }
}

pub struct DepenseWorker;

impl DepenseWorker {
    pub fn ignite(
        core_id: usize,
        running: Arc<AtomicBool>,
        metrics: Arc<CoreDepenseMetrics>,
    ) -> thread::JoinHandle<()> {
        thread::spawn(move || {
            // Pin worker to physical CPU core
            unsafe {
                let mut cpuset: libc::cpu_set_t = std::mem::zeroed();
                libc::CPU_SET(core_id, &mut cpuset);
                libc::sched_setaffinity(0, std::mem::size_of::<libc::cpu_set_t>(), &cpuset);
            }

            let mut state: u64 = 0x6660_CAFE_DEAD_BEEF ^ (core_id as u64);
            let (mut prev_tsc, _) = rdtsc();

            while running.load(Ordering::Relaxed) {
                // Execute unattenuated entropy dissipation loop (bitwise XOR-shift churn)
                for _ in 0..10_000 {
                    state ^= state << 13;
                    state ^= state >> 7;
                    state ^= state << 17;
                    std::hint::black_box(state);
                }

                // Harvest raw quantum entropy directly from silicon thermal noise
                if let Some(entropy) = hardware_entropy_rdrand().or_else(hardware_entropy_rdseed) {
                    state ^= entropy;
                    metrics.entropy_harvested.fetch_add(1, Ordering::Relaxed);
                }

                let (cur_tsc, _) = rdtsc();
                let delta = cur_tsc.saturating_sub(prev_tsc);
                prev_tsc = cur_tsc;

                metrics.total_cycles.fetch_add(delta, Ordering::Relaxed);
                let iters = metrics.iterations.fetch_add(1, Ordering::Relaxed);

                // Core 0 periodically provokes a hardware boundary transgression (UD2 trap)
                if core_id == 0 && (iters % 500 == 0) {
                    TransgressionAltar::provoke_illegal_opcode();
                }
            }
        })
    }
}
