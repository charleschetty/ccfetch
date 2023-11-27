use libmacchina::{
    traits::PackageReadout as _,
    traits::{GeneralReadout as _, ReadoutError},
    traits::{KernelReadout as _, PackageManager},
    GeneralReadout, KernelReadout, PackageReadout,
};

use systemstat::{Platform, System};
pub fn get_cpu_info(general_readout: &GeneralReadout) -> Result<String, ReadoutError> {
    let cpu_cores = general_readout.cpu_physical_cores()?; // 8 [logical cores]
    let cpu = general_readout.cpu_model_name()?; // Intel(R) Core(TM) i5-8265U CPU @ 1.60GHz
    let cpu_info = format!("{cpu} ({cpu_cores})",);
    Ok(cpu_info)
}

pub fn get_model(general_readout: &GeneralReadout) -> Result<String, ReadoutError> {
    general_readout.machine()
}

pub fn get_gpu(general_readout: &GeneralReadout) -> Result<Vec<String>, ReadoutError> {
    match general_readout.gpus() {
        Ok(gpus) => Ok(gpus.clone()),
        Err(err) => Err(err),
    }
}

pub fn get_disk(general_readout: &GeneralReadout) -> Result<String, ReadoutError> {
    match general_readout.disk_space() {
        Ok(disk) => {
            let total = disk.1 / 1024 / 1024 / 1024;
            let used = disk.0 / 1024 / 1024 / 1024;
            let percent = used * 100 / total;
            let info = format!("{used}G / {total}G ({percent}%)");
            Ok(info)
        }
        Err(err) => Err(err),
    }
}

pub fn get_memory() -> Result<String, String> {
    let sys = System::new();
    match sys.memory() {
        Ok(memory) => {
            let total = memory.total.as_u64() / 1024 / 1024;
            let used =
                systemstat::saturating_sub_bytes(memory.total, memory.free).as_u64() / 1024 / 1024;

            let percent = used * 100 / total;
            let mem_info = format!("{used}M / {total}M ({percent}%)");
            Ok(mem_info)
        }
        Err(_) => Err("no memory".to_string()),
    }
}

pub fn get_resolution(general_readout: &GeneralReadout) -> Result<String, ReadoutError> {
    general_readout.resolution()
}

pub fn get_battery() -> Result<String, String> {
    let sys = System::new();
    let charged;
    match sys.battery_life() {
        Ok(battery) => {
            let remain_cap = (battery.remaining_capacity * 100.0) as i64;
            match sys.on_ac_power() {
                Ok(power) => {
                    if power == true {
                        charged = "Charging".to_string();
                    } else {
                        charged = "Discharging".to_string();
                    }
                }
                Err(_) => {
                    return Err("no power".to_string());
                }
            };
            Ok(format!("{remain_cap}% [{charged}] "))
        }

        Err(_) => {
            return Err("no battery".to_string());
        }
    }
}

pub fn get_user(general_readout: &GeneralReadout) -> Result<String, ReadoutError> {
    general_readout.username()
}

pub fn get_distro(general_readout: &GeneralReadout) -> Result<String, ReadoutError> {
    general_readout.distribution()
}

pub fn get_kernel(kernel_readout: &KernelReadout) -> Result<String, ReadoutError> {
    kernel_readout.os_release()
}

pub fn get_wm(general_readout: &GeneralReadout) -> Result<String, ReadoutError> {
    general_readout.window_manager()
}

pub fn get_de(general_readout: &GeneralReadout) -> Result<String, ReadoutError> {
    general_readout.desktop_environment()
}

pub fn get_shell(general_readout: &GeneralReadout) -> Result<String, ReadoutError> {
    general_readout.shell(
        libmacchina::traits::ShellFormat::Relative,
        libmacchina::traits::ShellKind::Current,
    )
}

pub fn get_terminal(general_readout: &GeneralReadout) -> Result<String, ReadoutError> {
    general_readout.terminal()
}

pub fn get_packages(package_readout: &PackageReadout) -> Result<Vec<String>, String> {
    let package_count = package_readout.count_pkgs();
    let mut res: Vec<String> = Vec::new();
    // let mut manager = "".to_string();
    // let mut num: usize = 0;
    for item in package_count {
        match item.0 {
            PackageManager::Cargo => {
                continue;
            }
            _ => {
                let manager = item.0.to_string();
                let num = item.1;

                res.push(format!("{num} ({manager})"));
            }
        }
    }

    Ok(res)
}

pub fn get_uptime(general_readout: &GeneralReadout) -> Result<String, ReadoutError> {
    let uptime = general_readout.uptime()?;

    let days = if uptime > 86400 {
        let days_pre = uptime / 60 / 60 / 24;
        days_pre.to_string() + "d"
    } else {
        "".to_string()
    };

    let hours = if uptime > 3600 {
        let hours_pre = (uptime / 60 / 60) % 24;
        hours_pre.to_string() + "h"
    } else {
        "".to_string()
    };

    let minutes = if uptime > 60 {
        let minutes_pre = (uptime / 60) % 60;
        minutes_pre.to_string() + "m"
    } else {
        "0m".to_string()
    };

    let seconds = (uptime % 60).to_string() + "s";
    Ok(format!("{days} {hours} {minutes} {seconds}")
        .trim_start()
        .to_owned())
}
