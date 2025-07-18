use sysinfo::Disks;

pub struct DiskMon {
    disks: Disks,
}

impl DiskMon {
    pub fn new() -> Self {
        let disks = Disks::new_with_refreshed_list();
        Self { disks }
    }

    pub fn refresh(&mut self) {
        self.disks.refresh(true); 
    }

    pub fn get_stats(&self) -> (f32, f32, f32) {
        let mut total_space = 0u64;
        let mut available_space = 0u64;

        for disk in self.disks.iter() {
            total_space += disk.total_space();
            available_space += disk.available_space();
        }

        let used_space = total_space.saturating_sub(available_space);

        
        let gb = 1024.0 * 1024.0 * 1024.0;
        (
            used_space as f32 / gb,
            available_space as f32 / gb,
            total_space as f32 / gb,
        )
    }

    pub fn disk_results(&mut self) {
        self.refresh();
        let (used, available, total) = self.get_stats();
        println!(
            "Disk usage: {:.2} GB used, {:.2} GB available, {:.2} GB total",
            used, available, total
        );
    }
}

