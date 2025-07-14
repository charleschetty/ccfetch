use libc::{c_ulong, statvfs};
use rpm_pkg_count::count;
use std::collections::HashMap;
use std::io::{self, BufRead, BufReader, Write};
use std::os::raw::c_char;
use std::path::PathBuf;
use std::{ffi::CString, fs::File, path::Path};
use std::{fs, mem};

use crate::tools::get_parent;
use crate::tools::pci::{
    get_device_name_pci, get_gpu_vendor_name, read_drm_devices_and_find_gpu,
    read_pci_devices_and_find_gpu,
};
pub fn get_cpu_info() -> Result<Vec<String>, String> {
    let logical_cpus: usize = {
        let mut set: libc::cpu_set_t = unsafe { mem::zeroed() };
        if unsafe { libc::sched_getaffinity(0, mem::size_of::<libc::cpu_set_t>(), &mut set) } == 0 {
            let mut count: u32 = 0;
            for i in 0..libc::CPU_SETSIZE as usize {
                if unsafe { libc::CPU_ISSET(i, &set) } {
                    count += 1
                }
            }
            count as usize
        } else {
            let cpus = unsafe { libc::sysconf(libc::_SC_NPROCESSORS_ONLN) };
            if cpus < 1 {
                1
            } else {
                cpus as usize
            }
        }
    };

    let cpuinfo;
    let home_dir = unsafe { std::env::home_dir().unwrap_unchecked() };
    let cache_dir = home_dir.join(".cache");

    if !cache_dir.exists() {
        unsafe { fs::create_dir_all(&cache_dir).unwrap_unchecked() };
    }

    let cache_cpuinfo = cache_dir.join("cpuinfo");
    let proc_cpuinfo = "/proc/cpuinfo";

    if Path::new(&cache_cpuinfo).exists() {
        cpuinfo = unsafe { fs::read_to_string(cache_cpuinfo).unwrap_unchecked() };
    } else {
        cpuinfo = unsafe { fs::read_to_string(proc_cpuinfo).unwrap_unchecked() };
        let mut file_a = fs::File::create(cache_cpuinfo).unwrap();
        file_a.write_all(cpuinfo.as_bytes()).unwrap();
    }

    let mut model_name = String::new();
    let mut phy_cores = String::new();
    let mut _get_name = false;
    let mut _get_physical_cores = false;
    for line in cpuinfo.lines() {
        if _get_physical_cores && _get_name {
            break;
        }
        if line.starts_with("model name") {
            model_name = line.split(':').nth(1).unwrap().trim().to_string();
            _get_name = true;
        } else if line.starts_with("cpu cores") {
            phy_cores = line.split(':').nth(1).unwrap().trim().to_string();
            _get_physical_cores = true;
        }
    }
    let mut packages: usize = 0;
    for line in cpuinfo.lines() {
        if line.starts_with("physical id") {
            let physical_id_tmp_str = line.split_whitespace().last().unwrap();
            if physical_id_tmp_str.parse::<usize>().unwrap() > packages {
                packages += 1;
            }
        }
    }
    packages += 1;

    let mut cpus = Vec::with_capacity(packages);
    for _ in 0..packages {
        let cpu_info = format!("{} ({}/{})", model_name, phy_cores, logical_cpus / packages);
        cpus.push(cpu_info);
    }
    Ok(cpus)
}

pub fn get_model() -> Result<String, String> {
    let product_name: String = match fs::read_to_string("/sys/class/dmi/id/product_name") {
        Ok(model) => model.trim().to_owned(),
        Err(_) => return Err("cannot read product name".to_string()),
    };

    let product_version: String = match fs::read_to_string("/sys/class/dmi/id/product_version") {
        Ok(model) => model.trim().to_owned(),
        Err(_) => return Err("cannot read product version".to_string()),
    };

    let cpu_info = format!("{product_name} {product_version}"); // Correct usage of format!
    Ok(cpu_info)
}

pub fn get_gpu() -> Result<Vec<String>, String> {
    let devices = match read_drm_devices_and_find_gpu() {
        Ok(devs) => devs,
        Err(_) => match read_pci_devices_and_find_gpu() {
            Ok(val) => val,
            Err(_) => return Err("no gpus".to_string()),
        },
    };
    let mut gpus = Vec::new();

    let mut device_map = HashMap::<(String, String), u8>::new();
    for item in &devices {
        if device_map.contains_key(item) {
            *device_map.get_mut(item).unwrap() += 1;
        } else {
            device_map.insert(item.clone(), 1);
        }
    }

    for ((vendor, device), count) in device_map {
        let vender_name = get_gpu_vendor_name(&vendor);
        match get_device_name_pci(&vendor, &device) {
            Ok((Some(vender), Some(name))) => {
                if vender_name == "Unknown Vendor" {
                    gpus.push(format!("{vender} {name}"));
                } else {
                    for _ in 0..count {
                        gpus.push(format!("{vender_name} {name}"));
                    }
                }
            }
            _ => {
                gpus.push(format!("Unknown Device {vendor}:{device}"));
                return Err("Device not found.".to_string());
            }
        }
    }
    Ok(gpus)
}

fn get_disk_state(path: String) -> Result<(u64, u64, u64), String> {
    let c_path = CString::new(path).expect("CString::new failed");

    let mut stat: statvfs = unsafe { mem::zeroed() };

    if unsafe { statvfs(c_path.as_ptr() as *const c_char, &mut stat) } != 0 {
        return Err("can not get disk information".to_string());
    }

    let total_space: c_ulong = stat.f_blocks * stat.f_frsize;
    let free_space: c_ulong = stat.f_bfree * stat.f_frsize;
    let used_space: c_ulong = total_space - free_space;

    let percent = if total_space > 0 {
        used_space * 100 / total_space
    } else {
        0
    };

    let total_space_gb = total_space / (1024 * 1024 * 1024);
    let used_space_gb = used_space / (1024 * 1024 * 1024);
    Ok((used_space_gb, total_space_gb, percent))
}
pub fn get_disk() -> Result<Vec<String>, String> {
    let path = Path::new("/proc/mounts");
    let file = File::open(path).map_err(|_| "can not open /proc/mounts".to_string())?;
    let reader = BufReader::new(file);
    let mut disks_info = Vec::new();

    let mut disks: HashMap<String, bool> = HashMap::new();
    for line in reader.lines() {
        let line = line.map_err(|_| "can not read /proc/mounts".to_string())?;
        let parts: Vec<&str> = line.split_whitespace().collect();

        let dev_name = parts.first().unwrap().to_string();
        if dev_name.starts_with("/dev") {
            // println!("{}", line);
            let dev_type = parts.get(2).unwrap().to_string();
            if dev_type == "zsf" || dev_type == "btrfs" || dev_type == "ext4" {
                match disks.get(&dev_name) {
                    Some(_) => {}
                    None => {
                        disks.insert(dev_name, true);

                        let path_this = parts.get(1).unwrap().to_string();
                        let res_tmp = get_disk_state(path_this).unwrap();
                        let total = res_tmp.1;
                        let used = res_tmp.0;
                        let percent_this = res_tmp.2;
                        let info = format!("{used}G / {total}G ({percent_this}%) - {dev_type}");
                        disks_info.push(info);
                    }
                }
            }
        } else {
            continue;
        }
    }

    Ok(disks_info)
}

pub fn get_memory() -> Result<String, String> {
    let meminfo = fs::read_to_string("/proc/meminfo")
        .map_err(|e| format!("can not read /proc/meminfo: {e}"))?;

    let mut total_memory = 0;
    let mut free_memory = 0;
    let mut number_read = 0;

    for line in meminfo.lines() {
        if line.starts_with("MemTotal:") {
            if let Some(value) = line.split_whitespace().nth(1) {
                total_memory = value.parse::<u64>().unwrap_or(0);
                number_read += 1;
            }
        } else if line.starts_with("MemAvailable:") {
            if let Some(value) = line.split_whitespace().nth(1) {
                free_memory = value.parse::<u64>().unwrap_or(0);
                number_read += 1;
            }
        }
        if number_read >= 2 {
            break;
        }
    }

    let used_memory = total_memory - free_memory;
    let percent = if total_memory > 0 {
        used_memory * 100 / total_memory
    } else {
        0
    };

    let used_memory_mb = used_memory / 1024;
    let total_memory_mb = total_memory / 1024;

    let info = format!("{used_memory_mb}M / {total_memory_mb}M ({percent}%)");
    Ok(info)
}

pub fn get_swap() -> Result<String, String> {
    let meminfo = fs::read_to_string("/proc/meminfo")
        .map_err(|e| format!("can not read /proc/meminfo: {e}"))?;

    let mut total_swap = 0;
    let mut free_swap = 0;
    let mut number_read = 0;

    for line in meminfo.lines() {
        if line.starts_with("SwapTotal:") {
            if let Some(value) = line.split_whitespace().nth(1) {
                total_swap = value.parse::<u64>().unwrap_or(0);
                number_read += 1;
            }
        } else if line.starts_with("SwapFree:") {
            if let Some(value) = line.split_whitespace().nth(1) {
                free_swap = value.parse::<u64>().unwrap_or(0);
                number_read += 1;
            }
        }
        if number_read >= 2 {
            break;
        }
    }

    if number_read < 2 || total_swap == 0 {
        return Err("no swap".to_string());
    }

    let used_swap = total_swap - free_swap;
    let percent = if total_swap > 0 {
        used_swap * 100 / total_swap
    } else {
        0
    };

    let used_memory_mb = used_swap / 1024;
    let total_memory_mb = total_swap / 1024;

    let info = format!("{used_memory_mb}M / {total_memory_mb}M ({percent}%)");
    Ok(info)
}

pub fn get_resolution() -> Result<String, String> {
    let drm_path = "/sys/class/drm/";

    match fs::read_dir(drm_path) {
        Ok(entries) => {
            for entry in entries.flatten() {
                {
                    let path = entry.path();
                    if path.is_dir() && path.file_name().unwrap().to_str().unwrap().contains("card")
                    {
                        let modes_path = path.join("modes");
                        if let Ok(content) = fs::read_to_string(modes_path) {
                            for line in content.lines() {
                                if line.contains("x") {
                                    return Ok(line.to_string());
                                }
                            }
                        }
                    }
                }
            }
            Err("can not find the resolution".to_string())
        }
        Err(e) => Err(format!("can not read: {e}")),
    }
}

pub fn get_battery() -> Result<String, std::io::Error> {
    let power_supply_path = "/sys/class/power_supply/";
    let battery_path = fs::read_dir(power_supply_path)?
        .filter_map(Result::ok)
        .find(|entry| entry.file_name().to_str().unwrap_or("").starts_with("BAT"))
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, "can not find battery"))?
        .path();

    let capacity_path = battery_path.join("capacity");
    let status_path = battery_path.join("status");

    let capacity = fs::read_to_string(capacity_path)?.trim().to_string();

    let status = fs::read_to_string(status_path)?.trim().to_string();

    let output = format!("{capacity}% [{status}]");

    Ok(output)
}

pub fn get_user() -> Result<String, String> {
    match std::env::var("USER").or_else(|_| std::env::var("LOGNAME")) {
        Ok(username) => Ok(username),
        Err(_) => Err("Failed to get username".to_string()),
    }
}

pub fn get_distro() -> Result<String, String> {
    let os_release_info = match fs::read_to_string("/etc/os-release") {
        Ok(info) => info,
        Err(_) => return Err("failed to read /etc/os-release".to_string()),
    };

    for line in os_release_info.lines() {
        if line.starts_with("NAME=") {
            let name = line.trim_start_matches("NAME=").trim_matches('"');
            return Ok(name.to_string());
        }
    }
    Err("ERROR".to_string())
}

pub fn get_kernel() -> Result<String, String> {
    let version_info = match fs::read_to_string("/proc/version") {
        Ok(path) => path,
        Err(_) => return Err("Failed to read /proc/version".to_string()),
    };

    let version_parts: Vec<&str> = version_info.split_whitespace().collect();
    if version_parts.len() > 2 {
        Ok(version_parts[2].to_string())
    } else {
        Err("ERROR".to_string())
    }
}

pub fn get_wm() -> Result<String, String> {
    match std::env::var("XDG_CURRENT_DESKTOP") {
        Ok(desktop) => Ok(desktop),
        Err(_) => Err("Failed to get desktop environment info:".to_string()),
    }
}

pub fn get_shell() -> Result<String, String> {
    match std::env::var("SHELL") {
        Ok(shell_path) => {
            if let Some(shell_name) = Path::new(&shell_path).file_name() {
                Ok(shell_name.to_string_lossy().to_string())
            } else {
                Err("Failed to extract shell name.".to_string())
            }
        }
        Err(_) => Err("Failed to get shell info".to_string()),
    }
}

pub fn get_terminal() -> Result<String, String> {
    let mut terminal_pid = unsafe { libc::getppid() };
    let shells = [
        "sh", "su", "nu", "bash", "fish", "dash", "tcsh", "zsh", "ksh", "csh",
    ];

    loop {
        let path = PathBuf::from("/proc")
            .join(terminal_pid.to_string())
            .join("comm");
        if let Ok(terminal_name) = fs::read_to_string(path) {
            let terminal_name = terminal_name.trim();
            if !shells.contains(&terminal_name) {
                return Ok(terminal_name.to_string());
            }
            terminal_pid = match get_parent(terminal_pid) {
                Some(pid) => pid,
                None => return Err("can not detect terminal information".to_string()),
            };
        } else {
            break;
        }
    }
    Err("can not detect terminal information".to_string())
}

pub fn count_pacman() -> Result<String, String> {
    let pacman_dir = Path::new("/var/lib/pacman/local");
    if pacman_dir.is_dir() {
        if let Ok(read_dir) = fs::read_dir(pacman_dir) {
            return Ok(format!("{} (pacman)", read_dir.count() - 1));
        };
    }

    Err("Error".to_string())
}

pub fn count_dpkg() -> io::Result<String> {
    let path = "/var/lib/dpkg/status";

    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut count = 0;
    let mut in_package = false;

    for line in reader.lines() {
        let line = line?;
        if line.starts_with("Package: ") {
            in_package = true;
        } else if in_package && line.is_empty() {
            count += 1;
            in_package = false;
        }
    }

    if in_package {
        count += 1;
    }
    Ok(format!("{count} (dpkg)"))
}

pub fn count_rpm() -> io::Result<String> {
    let pkg_count = unsafe { count() };

    match pkg_count {
        Some(n) => Ok(n.to_string()),
        None => Err(io::Error::other(
            "Could not count RPM packages (librpm unavailable)",
        )),
    }
}

pub fn get_uptime() -> Result<String, String> {
    let uptime_info = match fs::read_to_string("/proc/uptime") {
        Ok(info) => info,
        Err(_) => return Err("Failed to read /proc/uptime".to_string()),
    };
    let parts: Vec<&str> = uptime_info.split_whitespace().collect();

    let uptime_seconds;
    if let Some(uptime_seconds_) = parts.first() {
        let uptime_seconds_f64: f64 = uptime_seconds_.parse().unwrap();
        uptime_seconds = uptime_seconds_f64 as u64;
    } else {
        return Err("Failed to get uptime information.".to_string());
    }

    let days = if uptime_seconds > 86400 {
        let days_pre = uptime_seconds / 60 / 60 / 24;
        days_pre.to_string() + "d"
    } else {
        "".to_string()
    };

    let hours = if uptime_seconds > 3600 {
        let hours_pre = (uptime_seconds / 60 / 60) % 24;
        hours_pre.to_string() + "h"
    } else {
        "".to_string()
    };

    let minutes = if uptime_seconds > 60 {
        let minutes_pre = (uptime_seconds / 60) % 60;
        minutes_pre.to_string() + "m"
    } else {
        "0m".to_string()
    };

    let seconds = (uptime_seconds % 60).to_string() + "s";
    Ok(format!("{days} {hours} {minutes} {seconds}")
        .trim_start()
        .to_owned())
}
