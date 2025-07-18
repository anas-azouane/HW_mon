# System Monitor (Rust)

A simple, lightweight, multithreaded system monitoring tool written in Rust. It provides real-time hardware usage statistics, including CPU, memory, and disk monitoring.

---

## Features

- **CPU Monitoring**
  - Global CPU usage (%)
  - Per-core usage breakdown
- **Memory Monitoring**
  - Total, used, free, and available memory (in GB)
  - Memory usage percentage
- **Disk Monitoring**
  - Total and available disk space
  - Disk usage percentage
- **Multithreaded Architecture**
  - Each monitoring component runs in its own thread
- **Simple CLI Output**
  - Refreshes every second
- **Modular & Extensible**
  - Separate APIs for CPU, memory, and disk monitoring

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
