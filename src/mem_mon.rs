use sysinfo::System;

pub struct MemMon {
    sys: System,
}

impl MemMon {
    pub fn new() -> Self {
        let mut sys = System::new_all();
        sys.refresh_memory(); // Initial refresh
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
}
