pub mod color;
use std::fs;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

use color::_BOLD;
use color::_RESET;
pub mod logo;


pub fn split_by_newline_new(logo: &String) -> Vec<String> {
    let mut split_vec: Vec<String> = Vec::new();
    for item in logo.split('\n') {
        split_vec.push(item.to_owned());
    }
    split_vec
}

pub fn format_data(icon: &str, value: &str, color: &str) -> String {
    format!(
        " {color} {key}  {_BOLD}{_RESET} {value}",
        key = icon,
        value = value,
    )
}

pub fn get_device_name_pci(vendor_id: u16, device_id: u16) -> io::Result<Option<String>> {
    let path_hwdata = Path::new("/usr/share/hwdata/pci.ids");
    let path_misc = Path::new("/usr/share/misc/pci.ids");
    let file;
    if path_hwdata.exists() {
        file = File::open(&path_hwdata)?;
    } else {
        file = File::open(&path_misc)?;
    }
    let reader = io::BufReader::new(file);

    let mut device_name = None;
    let mut current_vendor = None;

    for line in reader.lines() {
        let line = line?;
        if line.starts_with('#') || line.trim().is_empty() {
            continue;
        }

        if line.split_whitespace().count() >= 2 {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let id = parts[0];
            if id == vendor_id.to_string() {
                current_vendor = Some(parts[1..].join(" "));
                continue;
            }
        }

        if let Some(_) = &current_vendor {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 {
                let id = parts[0];
                let name = parts[1..].join(" ");
                if id == device_id.to_string() {
                    device_name = Some(format!("{}", name));
                    break;
                }
            }
        }
    }

    Ok(device_name)
}

pub fn read_pci_devices() -> io::Result<Vec<(u16, u16)>> {
    let pci_devices_path = "/sys/bus/pci/devices";
    let mut devices = Vec::new();

    for entry in fs::read_dir(pci_devices_path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            let modalias_file_path = path.join("modalias");

            if modalias_file_path.exists() {
                let modalias_content = fs::read_to_string(modalias_file_path)?;
                let modalias_trimmed = modalias_content.trim();

                let class_id = &modalias_trimmed[44..46];

                if class_id == "03" {
                    let vendor_id_str = &modalias_trimmed[9..13];
                    let device_id_str = &modalias_trimmed[18..22];
                    let vendor_id = vendor_id_str.parse::<u16>().unwrap();
                    let device_id = device_id_str.parse::<u16>().unwrap();
                    devices.push((vendor_id, device_id));
                }
            }
        }
    }

    Ok(devices)
}
