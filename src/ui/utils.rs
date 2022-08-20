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

pub fn get_stdin_number() -> Option<i32> {
    let mut answer = String::new();

    io::stdin()
        .read_line(&mut answer)
        .expect("Failed to read line");

    match answer.trim().parse() {
        Ok(res) => Some(res),
        _ => {
            warn("Please type a number!");
            None
        }
    }
}

pub fn invalid_option() {
    warn("Please select a valid option!");
}
