mod info;
mod tools;
use crate::tools::color::*;
use info::info::{get_battery, get_memory, get_swap};
use info::info::{
    get_cpu_info, get_disk, get_distro, get_gpu, get_kernel, get_model, get_resolution, get_shell,
    get_terminal, get_uptime, get_user, get_wm,
};
use tools::split_by_newline_new;
use tools::{format_data, logo::*};

enum Os {
    Arch,
    Debian,
    Ubuntu,
    Other,
}

fn main() {
    let mut info: Vec<String> = Vec::new();
    let mut os_name: Os = Os::Other;

    let hardware_info = "┌───────── Hardware Information ─────────┐ ".to_string();
    info.push(hardware_info);

    match get_model() {
        Ok(model) => {
            let model_info = format_data("󰌢 ", &model, _CYAN);
            info.push(model_info);
        }
        Err(_) => {}
    }

    match get_cpu_info() {
        Ok(cpu) => {
            for item in cpu {
                let cpu_info = format_data(" ", &item, _CYAN);
                info.push(cpu_info);
            }
        }
        Err(_) => {}
    }

    match get_gpu() {
        Ok(gpu) => {
            for item in gpu {
                let gpu_info = format_data(" ", &item, _CYAN);
                info.push(gpu_info);
            }
        }
        Err(_) => {}
    }

    match get_disk() {
        Ok(disk) => {
            for item in disk {
                let disk_info = format_data(" ", &item, _CYAN);
                info.push(disk_info);
            }
        }
        Err(_) => {}
    }

    match get_memory() {
        Ok(mem) => {
            let mem_info = format_data("󰑭 ", &mem, _CYAN);
            info.push(mem_info);
        }
        Err(_) => {}
    }

    match get_swap() {
        Ok(mem) => {
            let swap_info = format_data("󰓡 ", &mem, _CYAN);
            info.push(swap_info);
        }
        Err(_) => {}
    }

    match get_resolution() {
        Ok(resolution) => {
            let resolution_tmp = resolution;
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

    match get_user() {
        Ok(user) => {
            let user_info = format_data(" ", &user, _CYAN);
            info.push(user_info);
        }
        Err(_) => {}
    }

    match get_distro() {
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

    match get_kernel() {
        Ok(kernel) => {
            let kernel_info = format_data(" ", &kernel, _CYAN);
            info.push(kernel_info);
        }
        Err(_) => {}
    }

    match get_wm() {
        Ok(wm) => {
            let wm_info = format_data(" ", &wm, _CYAN);
            info.push(wm_info);
        }
        Err(_) => {}
    }
    match get_shell() {
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

    match get_terminal() {
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

    match os_name {
        Os::Arch => match info::info::count_pacman() {
            Ok(package) => {
                let package_info = format_data("󰏖 ", &package, _CYAN);
                info.push(package_info);
            }
            Err(_) => {}
        },
        Os::Debian | Os::Ubuntu => match info::info::count_dpkg() {
            Ok(package) => {
                let package_info = format_data("󰏖 ", &package, _CYAN);
                info.push(package_info);
            }
            Err(_) => {}
        },
        Os::Other => {}
    }

    match get_uptime() {
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

    print!("{}", "\x1B[?7l");

    for i in 0..max_len {
        if i < left.len() {
            print!("{}", left[i]);
        }
        if i < right.len() {
            print!("{}", right[i]);
        }

        println!();
    }

    print!("{}", "\x1B[?7h");
}
