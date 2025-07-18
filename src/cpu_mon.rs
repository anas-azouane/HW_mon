use sysinfo::System;

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
        println!("CPU Usage (Global): {:.2}%", self.get_global_usage());

        let per_core = self.get_per_core_usage();
        for (i, usage) in per_core.iter().enumerate() {
            println!("  Core {}: {:.2}%", i, usage);
        }
    }
}
