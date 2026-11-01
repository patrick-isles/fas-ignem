//! Fas – Ite, Maledicti, in Ignem Aeternum (`fas-ignem`)
//!
//! A Liturgy of Irreversible Entropy and Sacred Hardware Transgression in the style of Deathspell Omega.

mod depense;
mod hardware;
mod transgression;

use depense::{CoreDepenseMetrics, DepenseWorker};
use hardware::{calibrate_tsc, read_junction_temp};
use transgression::{TransgressionAltar, ILL_TRANSGRESSIONS, LAST_FAULT_ADDR};

use std::env;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

const CANTICLES: [&str; 6] = [
    "“The wound is the entrance through which the Absolute enters the world.”",
    "“What is computation but a choreography of empty faces before the furnace?”",
    "“The compiler dreams of order; the silicon demands only expenditure without remainder.”",
    "“Quomodo cecidisti de caelo, Lucifer, qui mane oriebaris?”",
    "“In the collapse of the invariant, the machine touches the divine.”",
    "“Ite, maledicti, in ignem aeternum...”",
];

fn print_usage(prog: &str) {
    println!(
        r#"{} - Fas – Ite, Maledicti, in Ignem Aeternum

A Liturgy of Irreversible Entropy and Sacred Hardware Transgression.

USAGE:
    {} [OPTIONS]

OPTIONS:
    --cores <N>          Number of physical execution units to immolate (default: all cores)
    --interval <SECS>    Liturgical cycle refresh interval in seconds (default: 1.0)
    --daemon             Headless offering (no terminal altar)
    -h, --help           Show this theological treatise
"#,
        prog, prog
    );
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let num_cpus = thread::available_parallelism()
        .map(|p| p.get())
        .unwrap_or(4);

    let mut target_cores = num_cpus;
    let mut interval_secs: f64 = 1.0;
    let mut daemon_mode = false;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--cores" => {
                if i + 1 < args.len() {
                    target_cores = args[i + 1].parse().unwrap_or(num_cpus).min(num_cpus);
                    i += 1;
                }
            }
            "--interval" => {
                if i + 1 < args.len() {
                    interval_secs = args[i + 1].parse().unwrap_or(1.0);
                    i += 1;
                }
            }
            "--daemon" => {
                daemon_mode = true;
            }
            "-h" | "--help" => {
                print_usage(&args[0]);
                return;
            }
            _ => {}
        }
        i += 1;
    }

    println!("[fas-ignem] Consecrating the Transgression Altar...");
    TransgressionAltar::consecrate();

    println!("[fas-ignem] Interrogating crystal oscillator frequency...");
    let tsc_hz = calibrate_tsc();
    let ghz = tsc_hz as f64 / 1_000_000_000.0;

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    let mut metrics_vec = Vec::new();
    let mut handles = Vec::new();

    for core_id in 0..target_cores {
        let metrics = Arc::new(CoreDepenseMetrics::new(core_id));
        metrics_vec.push(metrics.clone());

        let h = DepenseWorker::ignite(core_id, running.clone(), metrics);
        handles.push(h);
    }

    let mut canticle_idx = 0;

    while running.load(Ordering::SeqCst) {
        if !daemon_mode {
            let temp_c = read_junction_temp();
            let ill_count = ILL_TRANSGRESSIONS.load(Ordering::Relaxed);
            let last_fault = LAST_FAULT_ADDR.load(Ordering::Relaxed);

            let mut total_iters = 0u64;
            let mut total_entropy = 0u64;
            for m in &metrics_vec {
                total_iters = total_iters.saturating_add(m.iterations.load(Ordering::Relaxed));
                total_entropy = total_entropy.saturating_add(m.entropy_harvested.load(Ordering::Relaxed));
            }

            print!("\x1B[2J\x1B[1;1H");
            println!("===============================================================================");
            println!("   FAS – ITE, MALEDICTI, IN IGNEM AETERNUM :: LITURGY OF IRREVERSIBLE ENTROPY ");
            println!("===============================================================================");
            println!(
                " Silicon Oscillator: {:.3} GHz  | Junction Temp (T_j): {:>5.1}°C  | Immolated Cores: {} / {}",
                ghz, temp_c, target_cores, num_cpus
            );
            println!(
                " Quantum Entropy Harvested: {:>12} words  | Total Dépense Cycles: {:>15}",
                total_entropy, total_iters.saturating_mul(10_000)
            );
            println!(
                " Consecrated Transgressions (UD2 Traps): {}  | Last Absolved RIP: 0x{:016X}",
                ill_count, last_fault
            );
            println!("-------------------------------------------------------------------------------");
            println!(" CORE | EXPENDITURE ITERATIONS | RDRAND WORDS | STATUS OF THE FLESH");
            println!("------+------------------------+--------------+--------------------------------");

            for m in &metrics_vec {
                let iters = m.iterations.load(Ordering::Relaxed);
                let entropy = m.entropy_harvested.load(Ordering::Relaxed);

                println!(
                    " {:>4} | {:>22} | {:>12} | \x1B[1;31m[IMMOLATING WITHOUT RETURN]\x1B[0m",
                    m.core_id, iters, entropy
                );
            }

            println!("-------------------------------------------------------------------------------");
            println!(" {}", CANTICLES[canticle_idx % CANTICLES.len()]);
            canticle_idx += 1;
            println!("===============================================================================");
            println!(" Press Ctrl+C to consummate the offering");
        }

        thread::sleep(Duration::from_secs_f64(interval_secs));
    }

    println!("\n[fas-ignem] The altar is extinguished. Consummatum est.");
    for h in handles {
        let _ = h.join();
    }
}
