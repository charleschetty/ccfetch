use libc::{c_ulong, statvfs};
use std::io::{self, BufRead, BufReader};
use std::os::raw::c_char;
use std::{ffi::CString, fs::File, path::Path};
use std::{fs, mem};

use crate::tools::{get_device_name_pci, read_pci_devices};

pub fn get_cpu_info() -> Result<String, String> {
    let cpuinfo = fs::read_to_string("/proc/cpuinfo").map_err(|e| e.to_string())?;

    let mut number_processor = 0;
    let mut num_cpu_cores: Option<&str> = None;
    let mut model_name: String = String::new();
    let mut is_get_cpu_core = false;
    let mut is_get_model_name = false;

    for line in cpuinfo.lines() {
        if line.starts_with("model name") {
            if !is_get_model_name {
                model_name = line.split(':').nth(1).unwrap().trim().to_string();
                is_get_model_name = true;
            }
            number_processor += 1;
        }
        if !is_get_cpu_core && line.starts_with("cpu cores") {
            num_cpu_cores = line.split_whitespace().last();
            is_get_cpu_core = true;
        }
    }

    let num_cpu_cores = num_cpu_cores.unwrap_or("unknown cpu cores");

    let cpu_info = format!("{} ({}/{})", model_name, num_cpu_cores, number_processor);
    Ok(cpu_info)
}

pub fn get_model() -> Result<String, String> {
    let product_name: String;
    let product_version: String;

    match fs::read_to_string("/sys/class/dmi/id/product_name") {
        Ok(model) => product_name = model.trim().to_owned(),
        Err(_) => return Err("cannot read product name".to_string()),
    }

    match fs::read_to_string("/sys/class/dmi/id/product_version") {
        Ok(model) => product_version = model.trim().to_owned(),
        Err(_) => return Err("cannot read product version".to_string()),
    }

    let cpu_info = format!("{} {}", product_name, product_version); // Correct usage of format!
    Ok(cpu_info)
}

pub fn get_gpu() -> io::Result<Vec<String>> {
    let devices = read_pci_devices()?;
    let mut gpus = Vec::new();
    for (vendor, device) in devices {
        match get_device_name_pci(vendor, device)? {
            Some(name) => {
                gpus.push(name);
            }
            None => println!("Device not found."),
        }
    }
    Ok(gpus)
}

pub fn get_disk() -> Result<String, String> {
    let path = "/";
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

    let path = Path::new("/proc/mounts");
    let file = File::open(&path).map_err(|_| "can not open /proc/mounts".to_string())?;
    let reader = BufReader::new(file);

    let mut type_of_filesystem = String::new();
    for line in reader.lines() {
        let line = line.map_err(|_| "can not read /proc/mounts".to_string())?;
        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.get(1) == Some(&"/") {
            type_of_filesystem = parts.get(2).unwrap_or(&"").to_string();
            break;
        }
    }

    let info = format!("{used_space_gb}G / {total_space_gb}G ({percent}%) - {type_of_filesystem}");

    Ok(info)
}

pub fn get_memory() -> Result<String, String> {
    let meminfo = fs::read_to_string("/proc/meminfo")
        .map_err(|e| format!("can not read /proc/meminfo: {}", e))?;

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
        .map_err(|e| format!("can not read /proc/meminfo: {}", e))?;

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
            for entry in entries {
                if let Ok(entry) = entry {
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
        Err(e) => Err(format!("can not read: {}", e)),
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

    let output = format!("{}% [{}]", capacity, status);

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
    match std::env::var("TERM") {
        Ok(term) => Ok(term),
        Err(_) => Err("Failed to get terminal info".to_string()),
    }
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

pub fn get_uptime() -> Result<String, String> {
    let uptime_info = match fs::read_to_string("/proc/uptime") {
        Ok(info) => info,
        Err(_) => return Err("Failed to read /proc/uptime".to_string()),
    };
    let parts: Vec<&str> = uptime_info.split_whitespace().collect();

    let uptime_seconds;
    if let Some(uptime_seconds_) = parts.get(0) {
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
