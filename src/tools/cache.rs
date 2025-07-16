use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Default)]
struct Hardwarecache {
    cache_cpu: Vec<String>,
    cache_gpu: Vec<String>,
    cache_model: String,
}

struct CacheNeedNew {
    cpu: bool,
    gpu: bool,
    model: bool,
}
impl Default for CacheNeedNew {
    fn default() -> Self {
        Self {
            cpu: true,
            gpu: true,
            model: true,
        }
    }
}

pub struct HardwareCacheInfo {
    cache_dir: PathBuf,
    hardware_cache: Hardwarecache,
    need_a_new_one: CacheNeedNew,
    new_hardware_cache: Hardwarecache,
    os_last_open_time: u64,
}

impl HardwareCacheInfo {
    pub fn new() -> Self {
        let home_dir = unsafe { std::env::home_dir().unwrap_unchecked() };
        let global_cache_dir = home_dir.join(".cache/ccfetch");
        if !global_cache_dir.exists() {
            unsafe { fs::create_dir_all(&global_cache_dir).unwrap_unchecked() };
        }

        let path = "/sys";
        let metadata_os_sys = unsafe { fs::metadata(path).unwrap_unchecked() };
        let last_modify_time_os_sys = unsafe { metadata_os_sys.modified().unwrap_unchecked() };
        let dur = last_modify_time_os_sys
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap();

        let os_sys_time_asu64: u64 = dur.as_secs() * 1000 + dur.subsec_nanos() as u64 / 1000000;
        Self {
            cache_dir: global_cache_dir,
            os_last_open_time: os_sys_time_asu64,
            hardware_cache: Hardwarecache::default(),
            new_hardware_cache: Hardwarecache::default(),
            need_a_new_one: CacheNeedNew::default(),
        }
    }

    pub fn read_hardware(&mut self) {
        let hardware_path: PathBuf = self.cache_dir.join("hardware_info");
        if !hardware_path.exists() {
            unsafe { fs::File::create(&hardware_path).unwrap_unchecked() };
            return;
        }
        let hardware_cache = unsafe { fs::read_to_string(hardware_path).unwrap_unchecked() };
        let mut the_lines = hardware_cache.lines();
        if the_lines
            .next()
            .unwrap_or("1 0")
            .split_whitespace()
            .last()
            .unwrap_or("0")
            != self.os_last_open_time.to_string()
        {
            return;
        }

        for line in the_lines {
            if line.starts_with("gpu") {
                let gpu_info = line.strip_prefix("gpu ").unwrap();
                self.hardware_cache.cache_gpu.push(gpu_info.to_owned());
                self.need_a_new_one.gpu = false;
            } else if line.starts_with("cpu") {
                let cpu_info = line.strip_prefix("cpu ").unwrap();
                self.hardware_cache.cache_cpu.push(cpu_info.to_owned());
                self.need_a_new_one.cpu = false;
            } else if line.starts_with("model ") {
                let model_info = line.strip_prefix("model ").unwrap();
                self.hardware_cache.cache_model = model_info.to_owned();
                self.need_a_new_one.model = false;
            }
        }
    }

    pub fn get_cpus(&self) -> Option<Vec<String>> {
        if self.need_a_new_one.cpu {
            None
        } else {
            Some(self.hardware_cache.cache_cpu.clone())
        }
    }
    pub fn get_gpus(&self) -> Option<Vec<String>> {
        if self.need_a_new_one.gpu {
            None
        } else {
            Some(self.hardware_cache.cache_gpu.clone())
        }
    }
    pub fn get_model(&self) -> Option<String> {
        if self.need_a_new_one.model {
            None
        } else {
            Some(self.hardware_cache.cache_model.clone())
        }
    }

    pub fn add_cached_cpu(&mut self, cpus: &[String]) {
        self.new_hardware_cache.cache_cpu = cpus.to_owned();
    }
    pub fn add_cached_gpu(&mut self, gpus: &[String]) {
        self.new_hardware_cache.cache_gpu = gpus.to_owned();
    }
    pub fn add_cached_model(&mut self, model: &String) {
        self.new_hardware_cache.cache_model = model.to_owned();
    }

    pub fn store_hardware(&self) {
        if !self.need_a_new_one.model && !self.need_a_new_one.cpu && !self.need_a_new_one.gpu {
            return;
        }

        let mut output = String::new();
        let timeinfo = format!("time {}\n", self.os_last_open_time);
        output.push_str(&timeinfo);

        let modelinfo = {
            if self.need_a_new_one.model {
                format!("model {}\n", self.new_hardware_cache.cache_model)
            } else {
                format!("model {}\n", self.hardware_cache.cache_model)
            }
        };
        output.push_str(&modelinfo);

        let the_cpu_info_vec = {
            if self.need_a_new_one.cpu {
                &self.new_hardware_cache.cache_cpu
            } else {
                &self.hardware_cache.cache_cpu
            }
        };
        for item in the_cpu_info_vec {
            let cpu_info = format!("cpu {item}\n");
            output.push_str(&cpu_info);
        }

        let the_gpu_info_vec = {
            if self.need_a_new_one.gpu {
                &self.new_hardware_cache.cache_gpu
            } else {
                &self.hardware_cache.cache_gpu
            }
        };
        for item in the_gpu_info_vec {
            let gpu_info = format!("gpu {item}\n");
            output.push_str(&gpu_info);
        }
        // println!("out {}", output);
        let hardware_path: PathBuf = self.cache_dir.join("hardware_info");
        fs::write(hardware_path, output).unwrap();
    }
}



// todo , need flatpak, snap ,nix, emerge.... 
// need to find other way of handling different distro
#[derive(Default)]
struct Package {
    last_modify_time: u64,
    name: String,
    count: u64,
}

impl Package {}

struct PackageCache {
    cache_dir: PathBuf,
    // use ahash myabe
    packages_cache: HashMap<String, Package>,
}

impl PackageCache {
    pub fn new() -> Self {
        let home_dir = unsafe { std::env::home_dir().unwrap_unchecked() };
        let global_cache_dir = home_dir.join(".cache/ccfetch");
        Self {
            cache_dir: global_cache_dir,
            packages_cache: HashMap::new(),
        }
    }
    pub fn count_dpkg(&mut self) -> Option<String> {
        let path = "/var/lib/dpkg/status";
        let metadata_status = fs::metadata(path).unwrap();
        let last_modified_system_status = metadata_status.modified().unwrap();
        let dur = last_modified_system_status
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap();

        let dpkg_last_modified_time: u64 =
            dur.as_secs() * 1000 + dur.subsec_nanos() as u64 / 1000000;

        let cache_dpkg = self.cache_dir.join("dpkg.txt");

        let dpkg_packge_cache = Package {
            last_modify_time: dpkg_last_modified_time,
            name: "dpkg.txt".to_string(),
            count: 0,
        };
        if Path::new(&cache_dpkg).exists() {
            let reader_cache = unsafe { fs::read_to_string(&cache_dpkg).unwrap_unchecked() };
            let line = reader_cache.trim_end();
            let parts: Vec<&str> = line.split_whitespace().collect();
            let cache_last_modify = parts.first().unwrap();
            if *cache_last_modify == dpkg_last_modified_time.to_string() {
                let count = parts.get(1).unwrap();
                return Some(format!("{count} (dpkg)"));
            } else {
                self.packages_cache
                    .insert("dpkg.txt".to_string(), dpkg_packge_cache);
            }
        } else {
            self.packages_cache
                .insert("dpkg.txt".to_string(), dpkg_packge_cache);
        }
        None
    }

    pub fn add_cache_dpkg(&mut self, count: u64) {
        self.packages_cache.get_mut("dpkg.txt").unwrap().count = count;
    }

    pub fn store_packages(&self) {

    }
}
