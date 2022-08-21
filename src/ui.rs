use colored::Colorize;

use crate::{
    solves::{
        get_data, get_years,
        year::{AdventOfCodeDay, AdventOfCodeYear, DayProgress, SolveFunction},
    },
    ui::utils::warn,
};

use self::menu::Menu;

pub mod banner;
pub mod menu;
pub mod utils;

pub fn start_menu() {
    let years = get_years();

    let mut menu = Menu::new("");

    menu.add(-3, "Latest Year", || {
        year_menu(&years[years.len() - 1]);
    });

    menu.color(-3, colored::Color::Green);

    menu.add(-2, "Latest Day", || {
        let latest_year = years.len() - 1;
        let latest_day = &years[latest_year].days.len() - 1;

        day_menu(
            latest_day,
            &years[latest_year].days[latest_day],
            years[latest_year].year,
        )
    });

    menu.color(-2, colored::Color::Green);

    menu.add_back_option("Exit");

    years.iter().enumerate().for_each(|(idx, year)| {
        menu.add(
            (idx + 1) as i32,
            {
                let mut full_solve_count = 0;
                let mut half_solve_count = 0;

                for day in &year.days {
                    match day.progress() {
                        DayProgress::FullySolved => full_solve_count += 1,
                        DayProgress::PartlySolved => half_solve_count += 1,
                        DayProgress::Unsolved => (),
                    }
                }

                format!(
                    "Year {} - {}{}",
                    year.year,
                    "*".repeat(full_solve_count).bright_yellow(),
                    "*".repeat(half_solve_count).bright_magenta()
                )
            },
            || year_menu(year),
        )
    });

    menu.display();
}

fn year_menu(year: &AdventOfCodeYear) {
    let mut menu = Menu::new(format!("--- {} ---", year.year));

    menu.add(-3, "Benchmark", || warn("Not implemented!"));
    menu.color(-3, colored::Color::Yellow);

    menu.add(-2, "Latest Day", || warn(""));
    menu.color(-2, colored::Color::Green);

    menu.add_back_option("Go Back");

    year.days.iter().enumerate().for_each(|(idx, day)| {
        menu.add(
            (idx + 1) as i32,
            format!(
                "Day [{}] - {} - {}",
                idx + 1,
                day.name,
                match day.progress() {
                    DayProgress::FullySolved => "**".bright_yellow(),
                    DayProgress::PartlySolved => "*".bright_yellow(),
                    DayProgress::Unsolved => "".bright_yellow(),
                }
            ),
            move || day_menu(idx, day, year.year),
        )
    });

    menu.display();
}

fn day_menu(idx: usize, day: &AdventOfCodeDay, year: u32) {
    let progress = day.progress();

    let part2_solved = matches!(progress, DayProgress::FullySolved);
    let part1_solved = matches!(progress, DayProgress::PartlySolved) || part2_solved;

    let mut menu = Menu::new(format!("--- Day {} - {} ---", idx + 1, day.name));

    menu.add_conditional(
        -1,
        "Solve",
        || part1_solved,
        || {
            let mut results = run_solve(day.part1, year, idx as u32);

            if part2_solved {
                results = format!("{}\n{}", results, run_solve(day.part2, year, idx as u32));
            }

            let mut part_menu = Menu::new(results);

            part_menu.add_back_option("Go Back");
            part_menu.display();
        },
    );
    menu.color(-1, colored::Color::Green);

    menu.add_conditional(
        1,
        "Part 1",
        || part1_solved,
        || {
            let mut part_menu = Menu::new(run_solve(day.part1, year, idx as u32));

            part_menu.add_back_option("Go Back");
            part_menu.display();
        },
    );

    menu.add_conditional(
        2,
        "Part 2",
        || part2_solved,
        || {
            let mut part_menu = Menu::new(run_solve(day.part2, year, idx as u32));

            part_menu.add_back_option("Go Back");
            part_menu.display();
        },
    );

    menu.add_back_option("Go Back");

    menu.display();
}

fn run_solve(solve_function: SolveFunction, year: u32, day: u32) -> String {
    if let Ok(data) = get_data(year, day + 1) {
        let result = solve_function
            .expect("run_solve is only called when we know solve_function is Ok()")(
            data
        );

        format!("Result: {result}")
    } else {
        warn("Couldn't load data for current day.");
        String::from("")
    }
}
