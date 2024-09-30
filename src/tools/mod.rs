pub mod color;
use std::fs;
use std::io::BufRead;
use std::io::BufReader;
use std::path::PathBuf;

use color::_BOLD;
use color::_RESET;
pub mod logo;
pub mod pci;

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
pub fn get_parent(pid: i32) -> Option<i32> {
    let process_path = PathBuf::from("/proc").join(pid.to_string()).join("status");
    let file = fs::File::open(process_path).ok()?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        if let Ok(line) = line {
            if line.to_uppercase().starts_with("PPID") {
                return line.split_whitespace().nth(1).and_then(|s| s.parse().ok());
            }
        }
    }
    None
}


