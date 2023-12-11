mod info;
mod tools;
use crate::tools::color::*;
use info::info::{
    get_battery, get_cpu_info, get_de, get_disk, get_distro, get_gpu, get_kernel, get_memory,
    get_model, get_packages, get_resolution, get_resolution_x11, get_shell, get_terminal,
    get_uptime, get_user, get_wm,
};
use libmacchina::{
    traits::GeneralReadout as _, traits::KernelReadout as _, traits::PackageReadout as _,
    GeneralReadout, KernelReadout, PackageReadout,
};
use tools::{format_data, logo::*, split_by_newline_new};

enum Os {
    Arch,
    Debian,
    Ubuntu,
    Other,
}

fn main() {
    let general_readout = GeneralReadout::new();
    let mut info: Vec<String> = Vec::new();
    let mut os_name: Os = Os::Other;

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

    match get_memory() {
        Ok(mem) => {
            let mem_info = format_data("󰍛 ", &mem, _CYAN);
            info.push(mem_info);
        }
        Err(_) => {}
    }

    match get_resolution(&general_readout) {
        Ok(resolution) => {
            let mut resolution_tmp = resolution.clone();
            if &resolution.len() < &2 {
                match get_resolution_x11() {
                    Ok(val) => resolution_tmp = val,
                    Err(_) => {}
                }
            }
            let res_info = format_data(" ", &resolution_tmp, _CYAN);
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
            let os = distro.split(' ').next().unwrap();
            if os == "Arch" {
                os_name = Os::Arch;
                let disto_info = format_data(" ", &distro, _CYAN);
                info.push(disto_info);
            } else if os == "Ubuntu" {
                os_name = Os::Ubuntu;
                let disto_info = format_data(" ", &distro, _CYAN);
                info.push(disto_info);
            } else if os == "Debian" {
                os_name = Os::Debian;
                let disto_info = format_data(" ", &distro, _CYAN);
                info.push(disto_info);
            } else {
                os_name = Os::Other;
                let disto_info = format_data("󰌽 ", &distro, _CYAN);
                info.push(disto_info);
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
                shell_n = shell.split('\n').next().unwrap();
            }
            let shell_info = format_data(" ", shell_n, _CYAN);
            info.push(shell_info);
        }
        Err(_) => {}
    }

    match get_terminal(&general_readout) {
        Ok(terminal) => {
            let mut terminal_n = "";
            for _i in 0..1 {
                terminal_n = terminal.split('\n').next().unwrap();
            }
            let terminal_info = format_data(" ", terminal_n, _CYAN);
            info.push(terminal_info);
        }
        Err(_) => {}
    }

    let package_readout = PackageReadout::new();
    match get_packages(&package_readout) {
        Ok(packages) => {
            for package in packages {
                let package_info = format_data("󰏖 ", &package, _CYAN);
                info.push(package_info);
            }
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

    let mut logo_info: Vec<String>;
    match os_name {
        Os::Arch => {
            let logo = Archlogo::new(_CYAN).getlogo();
            logo_info = split_by_newline_new(&logo);
        }
        Os::Debian => {
            let logo = Debianlogo::new(_CYAN).getlogo();
            logo_info = split_by_newline_new(&logo);
        }
        Os::Ubuntu => {
            let logo = Ubuntulogo::new(_CYAN).getlogo();
            logo_info = split_by_newline_new(&logo);
        }
        Os::Other => {
            let logo = Otherlogo::new(_CYAN).getlogo();
            logo_info = split_by_newline_new(&logo);
        }
    }
    print_left_to_right(&mut logo_info, &mut info);
}

fn print_left_to_right(left: &mut Vec<String>, right: &mut Vec<String>) {
    let left_len = left.len();
    let right_len = right.len();

    let max_len = if left_len > right_len {
        left_len
    } else {
        let to_push = left[left_len - 1].clone();
        for _i in 0..(right_len - left_len) {
            left.push(to_push.clone());
        }
        right_len
    };

    for i in 0..max_len {
        crossterm::execute!(std::io::stdout(), crossterm::terminal::DisableLineWrap)
            .unwrap_or_default();
        if i < left.len() {
            print!("{}", left[i]);
        }
        if i < right.len() {
            print!("{}", right[i]);
        }

        println!();
        crossterm::execute!(std::io::stdout(), crossterm::terminal::EnableLineWrap)
            .unwrap_or_default();
    }
}
