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
    Fedora,
    Ubuntu,
    Other,
}

fn main() {
    let mut info: Vec<String> = Vec::new();
    let mut os_name: Os = Os::Other;

    let hardware_info = "┌───────── Hardware Information ─────────┐ ".to_string();
    info.push(hardware_info);

    if let Ok(model) = get_model() {
        let model_info = format_data("󰌢 ", &model, _CYAN);
        info.push(model_info);
    }

    if let Ok(cpu) = get_cpu_info() {
        for item in cpu {
            let cpu_info = format_data(" ", &item, _CYAN);
            info.push(cpu_info);
        }
    }

    if let Ok(gpu) = get_gpu() {
        for item in gpu {
            let gpu_info = format_data(" ", &item, _CYAN);
            info.push(gpu_info);
        }
    }

    if let Ok(disk) = get_disk() {
        for item in disk {
            let disk_info = format_data(" ", &item, _CYAN);
            info.push(disk_info);
        }
    }

    if let Ok(mem) = get_memory() {
        let mem_info = format_data("󰑭 ", &mem, _CYAN);
        info.push(mem_info);
    }

    if let Ok(mem) = get_swap() {
        let swap_info = format_data("󰓡 ", &mem, _CYAN);
        info.push(swap_info);
    }

    if let Ok(resolution) = get_resolution() {
        let resolution_tmp = resolution;
        let res_info = format_data(" ", &resolution_tmp, _CYAN);
        info.push(res_info);
    }

    if let Ok(battery) = get_battery() {
        let battery_info = format_data(" ", &battery, _CYAN);
        info.push(battery_info);
    }

    let software_info = "├───────── Software Information ─────────┤".to_string();
    info.push(software_info);

    if let Ok(user) = get_user() {
        let user_info = format_data(" ", &user, _CYAN);
        info.push(user_info);
    }

    if let Ok(distro) = get_distro() {
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
        } else if os == "Fedora"{
            os_name = Os::Fedora;
            let distro_info = format_data(" ", &distro, _CYAN);
            info.push(distro_info)
        } else {
            os_name = Os::Other;
            let disto_info = format_data("󰌽 ", &distro, _CYAN);
            info.push(disto_info);
        }
    }

    if let Ok(kernel) = get_kernel() {
        let kernel_info = format_data(" ", &kernel, _CYAN);
        info.push(kernel_info);
    }

    if let Ok(wm) = get_wm() {
        let wm_info = format_data(" ", &wm, _CYAN);
        info.push(wm_info);
    }
    if let Ok(shell) = get_shell() {
        let mut shell_n = "";
        for _i in 0..1 {
            shell_n = shell.split('\n').next().unwrap();
        }
        let shell_info = format_data(" ", shell_n, _CYAN);
        info.push(shell_info);
    }

    if let Ok(terminal) = get_terminal() {
        let mut terminal_n = "";
        for _i in 0..1 {
            terminal_n = terminal.split('\n').next().unwrap();
        }
        let terminal_info = format_data(" ", terminal_n, _CYAN);
        info.push(terminal_info);
    }

    match os_name {
        Os::Arch => if let Ok(package) = info::info::count_pacman() {
            let package_info = format_data("󰏖 ", &package, _CYAN);
            info.push(package_info);
        },
        Os::Debian | Os::Ubuntu => if let Ok(package) = info::info::count_dpkg() {
            let package_info = format_data("󰏖 ", &package, _CYAN);
            info.push(package_info);
        },
        Os::Fedora => if let Ok(package) = info::info::count_rpm() {
            let package_info = format_data("󰏖 ", &package, _CYAN);
            info.push(package_info)
        },
        Os::Other => {}
    }

    if let Ok(time) = get_uptime() {
        let time_info = format_data("󰅐 ", &time, _CYAN);
        info.push(time_info);
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
        Os::Fedora => {
            let logo = Fedoralogo::new(_CYAN).getlogo();
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

fn print_left_to_right(left: &mut Vec<String>, right: &mut [String]) {
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

    print!("\x1B[?7l");

    for i in 0..max_len {
        if i < left.len() {
            print!("{}", left[i]);
        }
        if i < right.len() {
            print!("{}", right[i]);
        }

        println!();
    }

    print!("\x1B[?7h");
}
