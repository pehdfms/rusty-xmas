use std::{thread, time::Duration};

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
