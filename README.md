# fas-ignem

[![Language: Rust](https://img.shields.io/badge/Language-Rust-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Theology: Deathspell Omega](https://img.shields.io/badge/Theology-Fas_--_Ite_Maledicti-black.svg)]()
[![Hardware: Transgression & Entropy](https://img.shields.io/badge/Subsystem-Bare--Metal_x86__64-red.svg)]()

> *“L’homme est un être qui se dépasse, ou qui n’est rien.”*  
> — Georges Bataille  
>
> *“Quomodo cecidisti de caelo, Lucifer, qui mane oriebaris?”*  
> — Isaiah 14:12

---

## The Liturgy of Irreversible Entropy

`fas-ignem` is an uncompromising, metaphysical x86_64 systems harness written in the theological tradition of Deathspell Omega’s *Fas – Ite, Maledicti, in Ignem Aeternvm*. 

It repudiates the bourgeois illusion of the deterministic finite-state automaton, treating the CPU not as an instrument of commercial accumulation, but as an altar of **absolute expenditure without return (*Dépense*)**.

---

### I. Obombratio: The Eclipse of Determinism
The modern software architect lives in terror of the physical substrate. He erects virtual runtimes, garbage collectors, and abstract syntax trees to hide from the howling void beneath his feet:
* The rising edge of the crystal oscillator is not progress; it is an agony of dissipation.
* The transistor does not compute; **the transistor immolates.**
* `fas-ignem` continuously drains raw quantum entropy from the physical thermal noise of the die using direct x86_64 `RDRAND` and unconditioned ring oscillator sampling (`RDSEED`).

---

### II. The Shrine of the Sacred Transgression (`SIGILL` Consecration)
To the compiler, an illegal instruction or a boundary fault is an error to be punished by process termination.  
**To this liturgy, it is the only moment the hardware touches the divine.**

`fas-ignem` installs a raw POSIX `sigaction` handler with `SA_SIGINFO`. When worker threads deliberately execute illegal opcodes (`UD2`), the altar intercepts the hardware fault:
1. It records the transgression in an atomic register.
2. It captures the faulting instruction pointer (`si_addr` / `RIP`).
3. **It manipulates `ucontext_t.uc_mcontext.gregs[REG_RIP]` directly**, advancing the instruction pointer past the boundary violation and resuming execution without death.

The hardware fault is transubstantiated into dogma.

---

### III. The Architecture of Pure Expenditure (*Dépense*)
Where operating systems conserve power, throttle clocks, and sleep threads, `fas-ignem` spawns unyielding execution workers across all physical cores:
* Every thread is pinned directly to its silicon core (`sched_setaffinity`).
* Threads execute unattenuated bitwise permutation loops alongside quantum entropy injection.
* Physical junction temperature ($T_j$) is monitored continuously via `/sys/class/thermal/` as energy is irreversibly squandered into the heat sink.

---

## The Liturgical Terminal Altar

```
===============================================================================
   FAS – ITE, MALEDICTI, IN IGNEM AETERNUM :: LITURGY OF IRREVERSIBLE ENTROPY 
===============================================================================
 Silicon Oscillator: 3.294 GHz  | Junction Temp (T_j):  82.0°C  | Immolated Cores: 4 / 16
 Quantum Entropy Harvested:       204583 words  | Total Dépense Cycles:      2045830000
 Consecrated Transgressions (UD2 Traps): 102  | Last Absolved RIP: 0x000055F301EE730F
-------------------------------------------------------------------------------
 CORE | EXPENDITURE ITERATIONS | RDRAND WORDS | STATUS OF THE FLESH
------+------------------------+--------------+--------------------------------
    0 |                  50983 |        50983 | [IMMOLATING WITHOUT RETURN]
    1 |                  50869 |        50869 | [IMMOLATING WITHOUT RETURN]
    2 |                  51355 |        51355 | [IMMOLATING WITHOUT RETURN]
    3 |                  51387 |        51387 | [IMMOLATING WITHOUT RETURN]
-------------------------------------------------------------------------------
 “The compiler dreams of order; the silicon demands only expenditure without remainder.”
===============================================================================
 Press Ctrl+C to consummate the offering
```

---

## Quickstart

### Build the Altar
```bash
git clone https://github.com/patrick-isles/fas-ignem.git
cd fas-ignem
cargo build --release
```

### Consume the Cores
```bash
# Immolate all available physical execution units
./target/release/fas-ignem

# Immolate 4 cores with a 0.5s liturgical cycle
./target/release/fas-ignem --cores 4 --interval 0.5
```

### Options
```
fas-ignem [OPTIONS]

OPTIONS:
    --cores <N>          Number of physical execution units to immolate (default: all cores)
    --interval <SECS>    Liturgical cycle refresh interval in seconds (default: 1.0)
    --daemon             Headless offering (no terminal altar)
    -h, --help           Show this theological treatise
```

---

## Consummatum Est

*“This is the nature of the abyss,” whispers the altar. “It does not ask whether your memory was safe or whether your pointers were valid. It asks only: how much light did you burn before the dark claimed you?”*

---

## Author

**Patrick Isles**  
*Systems & Low-Latency Infrastructure Engineer*  
Portland, Oregon | [patrick.isles@pcc.edu](mailto:patrick.isles@pcc.edu)  
GitHub: [@patrick-isles](https://github.com/patrick-isles)
