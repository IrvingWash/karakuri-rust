const RESET: &str = "\x1b[0m";
const BLUE: &str = "\x1b[34m";
const RED: &str = "\x1b[31m";
const YELLOW: &str = "\x1b[33m";

#[allow(dead_code)]
pub fn log_info(message: &str) {
    log(message, BLUE);
}

#[allow(dead_code)]
pub fn log_warning(message: &str) {
    log(message, YELLOW);
}

pub fn log_error(message: &str) {
    log(message, RED);
}

#[allow(dead_code)]
pub fn log_fatal(message: &str) -> ! {
    log_error(message);

    std::process::exit(1);
}

fn log(message: &str, color: &str) {
    println!("karakuri: {color}{message}{RESET}");
}
