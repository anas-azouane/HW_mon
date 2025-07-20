mod cpu_mon;
mod mem_mon;
mod disk_mon;
mod net_mon;

use cpu_mon::CpuMon;
use mem_mon::MemMon;
use disk_mon::DiskMon;
use net_mon::NetMon;
use std::thread::{self, JoinHandle};
use std::time::Duration;

fn main() {
    let mut handles: Vec<JoinHandle<()>> = Vec::new();

    handles.push(thread::spawn(|| {
        let mut cpu_monitor = CpuMon::new();
        loop {
            cpu_monitor.cpu_results();
            thread::sleep(Duration::from_secs(1));
        }
    }));

    handles.push(thread::spawn(|| {
        let mut memory_monitor = MemMon::new();
        loop {
            memory_monitor.mem_results();
            thread::sleep(Duration::from_secs(1));
        }
    }));
    
    handles.push(thread::spawn(|| {
        let mut disk_monitor = DiskMon::new();
        loop {
            disk_monitor.disk_results();
            thread::sleep(Duration::from_secs(1));
        }
    }));
    handles.push(thread::spawn(|| {
        let mut network = NetMon::new();
        loop {
            network.get_net_traffic();
            thread::sleep(Duration::from_secs(1));
        }
    }));
    for handle in handles {
        handle.join().unwrap();
    }
}

