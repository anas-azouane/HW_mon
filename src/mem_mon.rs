use sysinfo::System;
use notify_rust::Notification;

pub struct MemMon {
    sys: System,
}

impl MemMon {
    pub fn new() -> Self {
        let mut sys = System::new_all();
        sys.refresh_memory(); 
        Self { sys }
    }

    pub fn refresh(&mut self) {
        self.sys.refresh_memory();
    }

    pub fn get_usage(&self) -> f32 {
        (self.sys.used_memory() as f32 / self.sys.total_memory() as f32) * 100.0
    }

    pub fn get_stats_gb(&self) -> (f32, f32, f32, f32) {
        let used = self.sys.used_memory() as f32 / (1024.0 * 1024.0 * 1024.0);
        let total = self.sys.total_memory() as f32 / (1024.0 * 1024.0 * 1024.0);
        let free = self.sys.free_memory() as f32 / (1024.0 * 1024.0 * 1024.0);
        let available = self.sys.available_memory() as f32 / (1024.0 * 1024.0 * 1024.0);
        (used, total, free, available)
    }

    pub fn mem_results(&mut self){
        self.refresh();
        let usage = self.get_usage();
        println!("Memory Usage: {:.2}%", self.get_usage()); 
        let (used, total, free, available) = self.get_stats_gb();
        println!("  Used: {:.2} GB / Total: {:.2} GB", used, total);
        println!("  Free: {:.2} GB | Available: {:.2} GB", free, available);
        if usage > 90.0 {
            Notification::new()
                .summary("HW_mon: High Memory Usage")
                .body(&format!("Memory usage is at {:.2}%", usage))
                .show().unwrap();
        }

    }
}

