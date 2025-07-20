use sysinfo::System;
use notify_rust::Notification;

pub struct CpuMon {
    sys: System,
}

impl CpuMon {
    pub fn new() -> Self {
        let mut sys = System::new_all();
        sys.refresh_cpu_all(); // Initial refresh
        Self { sys }
    }

    pub fn refresh(&mut self) {
        self.sys.refresh_cpu_all();
    }

    pub fn get_global_usage(&self) -> f32 {
        self.sys.global_cpu_usage()
    }

    pub fn get_per_core_usage(&self) -> Vec<f32> {
        self.sys.cpus().iter().map(|cpu| cpu.cpu_usage()).collect()
    }
    pub fn cpu_results(&mut self){
        self.refresh();
        let usage = self.get_global_usage();
        println!("CPU Usage (Global): {:.2}%", self.get_global_usage());
        let per_core = self.get_per_core_usage();
        for (i, usage) in per_core.iter().enumerate() {
            println!("  Core {}: {:.2}%", i, usage);
        }
        if usage > 90.0 {
            Notification::new()
                .summary("HW_mon: CPU Usage")
                .body(&format!("CPU usage is at {:.2}%", usage))
                .show().unwrap();
        }
    }
}
