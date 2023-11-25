mod info;
mod tools;
use crate::tools::color::*;
use info::info::{
    get_battery, get_cpu_info, get_de, get_disk, get_distro, get_gpu, get_kernel, get_memory,
    get_model, get_packages, get_resolution, get_shell, get_terminal, get_uptime, get_user, get_wm,
};
use libmacchina::{
    traits::GeneralReadout as _, traits::KernelReadout as _, traits::MemoryReadout as _,
    traits::PackageReadout as _, GeneralReadout, KernelReadout, MemoryReadout, PackageReadout,
};
use tools::{
    format_data,
    logo::{Archlogo, Logo},
    split_by_newline_new,
};

fn main() {
    let general_readout = GeneralReadout::new();
    let mut info: Vec<String> = Vec::new();

    let hardware_info = "┌───────── Hardware Information ─────────┐ ".to_string();
    info.push(hardware_info);

    match get_model(&general_readout) {
        Ok(model) => {
            let model_info = format_data("󰌢 ", &model, _CYAN);
            info.push(model_info);
        }
        Err(_) => {}
    }

    match get_cpu_info(&general_readout) {
        Ok(cpu) => {
            let cpu_info = format_data(" ", &cpu, _CYAN);
            info.push(cpu_info);
        }
        Err(_) => {}
    }

    match get_gpu(&general_readout) {
        Ok(gpu) => {
            for item in gpu {
                let gpu_info = format_data(" ", &item, _CYAN);
                info.push(gpu_info);
            }
        }
        Err(_) => {}
    }

    match get_disk(&general_readout) {
        Ok(disk) => {
            let disk_info = format_data(" ", &disk, _CYAN);
            info.push(disk_info);
        }
        Err(_) => {}
    }

    let memory_readout = MemoryReadout::new();
    match get_memory(&memory_readout) {
        Ok(mem) => {
            let mem_info = format_data("󰍛 ", &mem, _CYAN);
            info.push(mem_info);
        }
        Err(_) => {}
    }

    match get_resolution(&general_readout) {
        Ok(resolution) => {
            let res_info = format_data(" ", &resolution, _CYAN);
            info.push(res_info);
        }
        Err(_) => {}
    }

    match get_battery() {
        Ok(battery) => {
            let battery_info = format_data(" ", &battery, _CYAN);
            info.push(battery_info);
        }
        Err(_) => {}
    }

    let software_info = "├───────── Software Information ─────────┤".to_string();
    info.push(software_info);

    match get_user(&general_readout) {
        Ok(user) => {
            let user_info = format_data(" ", &user, _CYAN);
            info.push(user_info);
        }
        Err(_) => {}
    }

    match get_distro(&general_readout) {
        Ok(distro) => {
            if &distro == "Arch Linux" {
                let disto_info = format_data(" ", &distro, _CYAN);
                info.push(disto_info);
            } else {
                //todo
            }
        }
        Err(_) => {}
    }

    let kernel_readout = KernelReadout::new();
    match get_kernel(&kernel_readout) {
        Ok(kernel) => {
            let kernel_info = format_data(" ", &kernel, _CYAN);
            info.push(kernel_info);
        }
        Err(_) => {}
    }

    match get_wm(&general_readout) {
        Ok(wm) => {
            let wm_info = format_data(" ", &wm, _CYAN);
            info.push(wm_info);
        }
        Err(_) => {}
    }
    match get_de(&general_readout) {
        Ok(de) => {
            let de_info = format_data(" ", &de, _CYAN);
            info.push(de_info);
        }
        Err(_) => {}
    }

    match get_shell(&general_readout) {
        Ok(shell) => {
            let mut shell_n = "";
            for _i in 0..1 {
                shell_n = shell.split("\n").next().unwrap();
            }
            let shell_info = format_data(" ", &shell_n, _CYAN);
            info.push(shell_info);
        }
        Err(_) => {}
    }

    match get_terminal(&general_readout) {
        Ok(terminal) => {
            let mut terminal_n = "";
            for _i in 0..1 {
                terminal_n = terminal.split("\n").next().unwrap();
            }
            let terminal_info = format_data(" ", &terminal_n, _CYAN);
            info.push(terminal_info);
        }
        Err(_) => {}
    }

    let package_readout = PackageReadout::new();
    match get_packages(&package_readout) {
        Ok(package) => {
            let package_info = format_data("󰏖 ", &package, _CYAN);
            info.push(package_info);
        }
        Err(_) => {}
    }

    match get_uptime(&general_readout) {
        Ok(time) => {
            let time_info = format_data("󰅐 ", &time, _CYAN);
            info.push(time_info);
        }
        Err(_) => {}
    }

    let end_info = "└─────────────────────────────────────────┘".to_string();
    info.push(end_info);

    let colorinfo = get_color();
    info.push(colorinfo);

    let logo = Archlogo::new(_CYAN);
    let arch_logo = logo.getlogo();
    let arcginfo = split_by_newline_new(&arch_logo);
    print_left_to_right(arcginfo, info);
}

fn print_left_to_right(left: Vec<String>, right: Vec<String>) {
    let left_len = left.len();
    let right_len = right.len();
    let max_len = if left_len > right_len {
        left_len
    } else {
        right_len
    };

    for i in 0..max_len {
        if i < left_len {
            print!("{}", left[i]);
        }
        if i < right_len {
            print!("{}", right[i]);
        }

        println!();
    }
}
