use sysinfo::{Disks, System};

pub struct SystemStats {
    pub os_name: String,
    pub os_version: String,
    pub host_name: String,
    pub cpu_name: String,
    pub cpu_cores: usize,
    pub total_memory: u64,
    pub used_memory: u64,
    pub memory_percentage: f64,
    pub overall_cpu_usage: f32,
    pub core_usages: Vec<f32>,
    pub total_disk: u64,
    pub used_disk: u64,
    pub disk_percentage: f64,
    pub uptime: u64,
}

impl SystemStats {
    pub fn new() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();

        let os_name = System::name().unwrap_or_else(|| "macOS".to_string());
        let os_version = System::os_version().unwrap_or_else(|| "Unknown".to_string());
        let host_name = System::host_name().unwrap_or_else(|| "MacBook".to_string());

        let cpus = sys.cpus();
        let cpu_name = if !cpus.is_empty() {
            cpus[0].brand().to_string()
        } else {
            "Apple Silicon".to_string()
        };
        let cpu_cores = cpus.len();

        let mut stats = Self {
            os_name,
            os_version,
            host_name,
            cpu_name,
            cpu_cores,
            total_memory: 0,
            used_memory: 0,
            memory_percentage: 0.0,
            overall_cpu_usage: 0.0,
            core_usages: Vec::new(),
            total_disk: 0,
            used_disk: 0,
            disk_percentage: 0.0,
            uptime: 0,
        };
        stats.update(&mut sys);
        stats
    }

    pub fn update(&mut self, sys: &mut System) {
        sys.refresh_cpu();
        sys.refresh_memory();

        self.total_memory = sys.total_memory();
        self.used_memory = sys.used_memory();
        if self.total_memory > 0 {
            self.memory_percentage = (self.used_memory as f64 / self.total_memory as f64) * 100.0;
        }

        let cpus = sys.cpus();
        self.core_usages = cpus.iter().map(|cpu| cpu.cpu_usage()).collect();
        if !self.core_usages.is_empty() {
            self.overall_cpu_usage = self.core_usages.iter().sum::<f32>() / self.core_usages.len() as f32;
        }

        // Get main disk space using sysinfo 0.30 Disks structure
        let disks = Disks::new_with_refreshed_list();
        let mut total_d = 0;
        let mut available_d = 0;
        for disk in &disks {
            if disk.mount_point().to_str() == Some("/") {
                total_d = disk.total_space();
                available_d = disk.available_space();
                break;
            }
        }
        if total_d == 0 && !disks.is_empty() {
            total_d = disks[0].total_space();
            available_d = disks[0].available_space();
        }

        self.total_disk = total_d;
        self.used_disk = total_d.saturating_sub(available_d);
        if self.total_disk > 0 {
            self.disk_percentage = (self.used_disk as f64 / self.total_disk as f64) * 100.0;
        }

        self.uptime = System::uptime();
    }
}
