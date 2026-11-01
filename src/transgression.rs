//! The Liturgy of Transgression: Consecration of Hardware Faults
//!
//! Intercepts CPU boundary exceptions (`SIGILL`, `SIGSEGV`) through raw `sigaction`
//! handlers, transubstantiating illegal opcodes and memory violations into liturgical dogma.

use libc::{c_int, c_void, sigaction, siginfo_t, ucontext_t, SA_SIGINFO, SIGILL, SIGSEGV};
use std::sync::atomic::{AtomicU64, Ordering};

pub static ILL_TRANSGRESSIONS: AtomicU64 = AtomicU64::new(0);
pub static SEGV_TRANSGRESSIONS: AtomicU64 = AtomicU64::new(0);
pub static LAST_FAULT_ADDR: AtomicU64 = AtomicU64::new(0);

// On Linux x86_64, gregs index 16 corresponds to REG_RIP
#[cfg(target_arch = "x86_64")]
const RIP_INDEX: usize = 16;

/// Liturgical signal handler that catches illegal opcodes (UD2) and skips past them.
#[cfg(target_arch = "x86_64")]
unsafe extern "C" fn liturgical_signal_handler(
    sig: c_int,
    info: *mut siginfo_t,
    ucontext: *mut c_void,
) {
    if !info.is_null() {
        let fault_addr = (*info).si_addr() as u64;
        LAST_FAULT_ADDR.store(fault_addr, Ordering::Relaxed);
    }

    let uc = ucontext as *mut ucontext_t;

    match sig {
        SIGILL => {
            ILL_TRANSGRESSIONS.fetch_add(1, Ordering::Relaxed);
            // On x86_64, UD2 instruction is exactly 2 bytes (0x0F, 0x0B).
            // Advance the instruction pointer (RIP) by 2 bytes to resume execution at next instruction.
            if !uc.is_null() {
                (*uc).uc_mcontext.gregs[RIP_INDEX] += 2;
            }
        }
        SIGSEGV => {
            SEGV_TRANSGRESSIONS.fetch_add(1, Ordering::Relaxed);
        }
        _ => {}
    }
}

pub struct TransgressionAltar;

impl TransgressionAltar {
    /// Installs the liturgical signal handler for hardware faults.
    pub fn consecrate() {
        unsafe {
            let mut sa: sigaction = std::mem::zeroed();
            sa.sa_sigaction = liturgical_signal_handler as *const () as usize;
            sa.sa_flags = SA_SIGINFO;

            sigaction(SIGILL, &sa, std::ptr::null_mut());
        }
    }

    /// Provokes an illegal opcode (`ud2`) in inline assembly.
    /// Caught by the liturgical signal handler and forgiven without process death.
    #[inline(always)]
    pub fn provoke_illegal_opcode() {
        unsafe {
            std::arch::asm!("ud2", options(nomem, nostack));
        }
    }
}
