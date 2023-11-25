pub mod color;
use color::_BOLD;
use color::_RESET;
pub mod logo;


pub fn split_by_newline_new(logo: &String) -> Vec<String> {
    let mut split_vec: Vec<String> = Vec::new();
    for item in logo.split("\n") {
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


