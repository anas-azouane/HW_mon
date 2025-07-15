mod cpu_mon;
mod mem_mon;

use cpu_mon::CpuMon;
use mem_mon::MemMon;
use std::thread::sleep;
use std::time::Duration;
fn main() {
    let mut cpu_monitor = CpuMon::new();
    let mut memory_monitor = MemMon::new();

    loop {
        // Refresh and display CPU stats
        cpu_monitor.refresh();
        println!("CPU Usage (Global): {:.2}%", cpu_monitor.get_global_usage());
        
        let per_core = cpu_monitor.get_per_core_usage();
        for (i, usage) in per_core.iter().enumerate() {
            println!("  Core {}: {:.2}%", i, usage);
        }

        // Refresh and display Memory stats
        memory_monitor.refresh();
        println!("Memory Usage: {:.2}%", memory_monitor.get_usage());
        
        let (used, total, free, available) = memory_monitor.get_stats_gb();
        println!("Used: {:.2} GB / Total: {:.2} GB", used, total);
        println!("Free: {:.2} GB | Available: {:.2} GB", free, available);
        println!("---");

        sleep(Duration::from_secs(1));
    }
}
