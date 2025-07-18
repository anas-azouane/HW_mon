# System Monitor (Rust)

A lightweight, cross-platform system monitoring tool written in Rust, providing real-time Hardware usage statistics.

---

## Features

- **CPU Monitoring**
  - Global CPU usage (%)
  - Per-core usage breakdown
- **Memory Monitoring**
  - Total, used, free, and available memory (in GB)
  - Memory usage percentage
- **Simple CLI Output**
  - Refreshes every second
- **Modular & Extensible**
  - Separate APIs for CPU and memory monitoring

---

## Installation

### Prerequisites

- [Rust (stable)](https://www.rust-lang.org/tools/install)

### Steps

1. **Clone the repository**:
   ```sh
   git clone https://github.com/anas-azouane/HW_mon.git 
   cd HW_mon
    ```
2. **Build & Run**:
   ```sh
   cargo run
    ```
