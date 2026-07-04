# 🍏 MacAnatomyTUI

```
      ___           ___           ___     
     /__/\         /  /\         /  /\    
    |  |::\       /  /::\       /  /:/    
    |  |:|:\     /  /:/\:\     /  /:/     
  __|__|:|\:\   /  /:/~/::\   /  /:/  ___ 
 /__/::::| \:\ /__/:/ /:/\:\ /__/:/  /  /\
 \  \:\~~~\__\/ \  \:\/:/__\/ \  \:\ /  /:/
  \  \:\         \  \::/       \  \:\  /:/ 
   \  \:\         \  \:\        \  \:\/:/  
    \  \:\         \  \:\        \  \::/   
     \__\/          \__\/         \__\/    
 __________________________________________
|                                          |
|   Interactive Apple Silicon TUI Map      |
|__________________________________________|
```

**MacAnatomyTUI** is an interactive, high-tech terminal utility built in Rust that visualizes the layout of Apple Silicon System-on-Chip (SoC) architectures (e.g., CPU, GPU, Unified Memory, Neural Engine) and explains their hardware terms in simple layman's language using your Mac's live system metrics.

---

## ⚡ Tech Stack & Architecture

- **Core Engine:** [Rust](https://www.rust-lang.org/) — Ensures lightning-fast system polling, memory safety, and minimal CPU footprint.
- **TUI Framework:** [Ratatui](https://ratatui.rs/) — Implements rich visual layouts, stateful lists, real-time graphs, and flexible borders.
- **Terminal Backend:** [Crossterm](https://github.com/crossterm-rs/crossterm) — Allows cross-platform control, raw terminal initialization, and input events parsing.
- **Telemetry Layer:** [Sysinfo](https://github.com/GuillaumeGomez/sysinfo) — Interacts directly with macOS APIs to extract host specs, active memory footprint, CPU core loads, and disk bandwidth.

---

## 🛠️ Features

*   **Interactive SoC Package Map:** Visually select CPU Cores, GPU Cores, Unified Memory (RAM), Neural Engine, SSD Controller, and Secure Enclave.
*   **Layman Analogies & Explanations:** Real-time translation of complex silicon terms to simple analogies (e.g., *Unified Memory as a Chef's countertop*).
*   **Live macOS Telemetry:** Real-time gauge bars tracking CPU loads, Memory Pressure, and SSD space allocation directly inside the TUI.
*   **Safety & Integrity Monitoring:** Failure symptoms explained for every single component to help you troubleshoot your hardware.

---

## 🚀 Getting Started

### Prerequisites

- A Mac running macOS (designed for Apple Silicon M1/M2/M3 chips, but works on Intel Macs as well).
- [Rust & Cargo](https://rustup.rs/) installed.

### Build and Launch

Clone the repository and run the application:

```bash
git clone https://github.com/saiyash07/MacAnatomyTUI.git
cd MacAnatomyTUI
cargo run --release
```

### Controls

| Key | Action |
|---|---|
| `Tab` / `Right Arrow` / `Down Arrow` | Move Selection Forward |
| `Shift+Tab` / `Left Arrow` / `Up Arrow` | Move Selection Backward |
| `Q` / `Esc` | Quit Application Safely |

---

## 📄 License

This project is licensed under the [MIT License](LICENSE).
