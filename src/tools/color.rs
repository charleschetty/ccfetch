pub const _RED: &str = "\x1b[31m";
pub const _GREEN: &str = "\x1b[32m";
pub const _YELLOW: &str = "\x1b[33m";
pub const _BLUE: &str = "\x1b[34m";
pub const _CYAN: &str = "\x1b[36m";
pub const _GRAY: &str = "\x1b[38;5;8m";
pub const _MAGENTA: &str = "\x1b[35m";
pub const _BOLD: &str = "\x1b[1m";
pub const _RESET: &str = "\x1b[0m";

pub fn get_color() -> String {
    format!("   {item:4} {_RED}{item:4}{_RESET} {_GREEN}{item:4}{_RESET} {_YELLOW}{item:4}{_RESET} {_BLUE}{item:4}{_RESET} {_MAGENTA}{item:4}{_RESET} {_CYAN}{item:4}{_RESET} {item:3}{_RESET}", item="" )
}
