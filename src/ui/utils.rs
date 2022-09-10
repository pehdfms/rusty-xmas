use std::{io, thread, time::Duration};

use colored::Colorize;

use super::banner::BANNER;

pub fn clear() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

pub fn warn(message: &str) {
    println!("{}", message.red());
    thread::sleep(Duration::from_secs(1));
}

pub fn banner() {
    println!("{}", BANNER);
}

pub fn new_menu() {
    clear();
    banner();
}

#[must_use]
pub fn get_stdin_number() -> Option<i64> {
    let mut answer = String::new();

    io::stdin()
        .read_line(&mut answer)
        .expect("Failed to read line");

    if let Ok(res) = answer.trim().parse() {
        return Some(res);
    }

    warn("Please type a number!");

    None
}

pub fn invalid_option() {
    warn("Please select a valid option!");
}

#[must_use]
pub fn format_result_runtime(result: &str, duration: Duration) -> String {
    format!("{result} - Elapsed Time: {}ms", duration.as_millis())
}
