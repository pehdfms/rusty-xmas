use std::{collections::BTreeMap, fmt::Display};

use colored::{Color, Colorize};

use crate::ui::utils::{get_stdin_number, invalid_option};

use super::utils::new_menu;

pub struct MenuOption<'inner> {
    name: Box<dyn Display + 'inner>,
    color: Option<Color>,
    condition: Box<dyn Fn() -> bool + 'inner>,
    then: Box<dyn Fn() + 'inner>,
    is_back_option: bool,
}

pub struct Menu<'inner, 'outer> {
    content: Box<dyn Display + 'outer>,
    options: BTreeMap<i32, MenuOption<'inner>>,
}

impl<'inner, 'outer> Menu<'inner, 'outer> {
    pub fn new(title: impl Display + 'outer) -> Menu<'inner, 'outer> {
        Menu {
            content: Box::new(title),
            options: BTreeMap::new(),
        }
    }

    pub fn add(&mut self, option: i32, name: impl Display + 'inner, then: impl Fn() + 'inner) {
        self.add_conditional(option, name, Box::new(|| true), then);
    }

    pub fn add_conditional(
        &mut self,
        option: i32,
        name: impl Display + 'inner,
        condition: impl Fn() -> bool + 'inner,
        then: impl Fn() + 'inner,
    ) {
        if self.options.contains_key(&option) {
            panic!("Tried to insert option {option} in Menu but found duplicate!");
        }

        self.options.insert(
            option,
            MenuOption {
                name: Box::new(name),
                color: None,
                condition: Box::new(condition),
                then: Box::new(then),
                is_back_option: false,
            },
        );
    }

    pub fn insert(&mut self, option: i32, name: impl Display + 'inner, then: impl Fn() + 'inner) {
        self.insert_conditional(option, name, Box::new(|| true), then);
    }

    pub fn insert_conditional(
        &mut self,
        option: i32,
        name: impl Display + 'inner,
        condition: impl Fn() -> bool + 'inner,
        then: impl Fn() + 'inner,
    ) {
        self.options.insert(
            option,
            MenuOption {
                name: Box::new(name),
                color: None,
                condition: Box::new(condition),
                then: Box::new(then),
                is_back_option: false,
            },
        );
    }

    pub fn remove(&mut self, option: i32) {
        self.options.remove(&option);
    }

    pub fn add_back_option(&mut self, name: impl Display + 'inner) {
        self.add(0, name, || {});
        let option = self.options.get_mut(&0).unwrap();

        option.color = Some(Color::Red);
        option.is_back_option = true;
    }

    pub fn color(&mut self, option: i32, color: Color) {
        self.options
            .get_mut(&option)
            .expect("Option should exist when coloring!")
            .color = Some(color);
    }

    pub fn display(&self) {
        loop {
            self.new_menu();

            for (key, option) in &self.options {
                if !(option.condition)() {
                    continue;
                }

                let to_print = format!("[{key}] - {}", option.name);

                if let Some(color) = option.color {
                    println!("{}", to_print.color(color))
                } else {
                    println!("[{key}] - {}", option.name);
                }
            }

            println!("Select an option:");

            let choice = match get_stdin_number() {
                Some(n) => n,
                _ => continue,
            };

            match self.options.get(&choice) {
                Some(option) if (option.is_back_option) => return,
                Some(option) if (option.condition)() => (option.then)(),
                _ => invalid_option(),
            };
        }
    }

    fn new_menu(&self) {
        new_menu();

        if self.content.to_string().is_empty() {
            return;
        }

        println!("{}", self.content)
    }
}
