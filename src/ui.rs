use colored::Colorize;

use crate::{
    solves::{
        get_data, get_years,
        year::{AdventOfCodeDay, AdventOfCodeYear, DayProgress, SolveFunction},
    },
    ui::utils::{get_stdin_number, invalid_option, new_menu, warn},
};

pub mod banner;
pub mod menu;
pub mod utils;

fn print_year(idx: usize, year: &AdventOfCodeYear) {
    let mut full_solve_count = 0;
    let mut half_solve_count = 0;

    for day in &year.days {
        match day.progress() {
            DayProgress::FullySolved => full_solve_count += 1,
            DayProgress::PartlySolved => half_solve_count += 1,
            DayProgress::Unsolved => (),
        }
    }

    println!(
        "[{idx}]  - Year {} - {}{}",
        year.year,
        "*".repeat(full_solve_count).bright_yellow(),
        "*".repeat(half_solve_count).bright_magenta()
    );
}

pub fn start_menu() {
    loop {
        new_menu();
        println!("{}", "[-3] - Latest Year".green());
        println!("{}", "[-2] - Latest Day".green());
        println!("{}", "[-1] - Exit".red());

        let years = get_years();
        years
            .iter()
            .enumerate()
            .for_each(|(idx, year)| print_year(idx, year));

        println!("Select an option:");

        let answer = match get_stdin_number() {
            Some(n) => n,
            _ => continue,
        };

        match answer {
            -3 => year_menu(&years[years.len() - 1]),
            -2 => {
                let latest_year = years.len() - 1;
                let latest_day = &years[latest_year].days.len() - 1;

                day_menu(
                    latest_day,
                    &years[latest_year].days[latest_day],
                    years[latest_year].year,
                )
            }
            -1 => return,
            x if x < 0 || x as usize >= years.len() => invalid_option(),
            x => year_menu(&years[x as usize]),
        }
    }
}

fn print_day(idx: usize, day: &AdventOfCodeDay) {
    println!(
        "Day [{}] - {} - {}",
        idx + 1,
        day.name,
        match day.progress() {
            DayProgress::FullySolved => "**".bright_yellow(),
            DayProgress::PartlySolved => "*".bright_yellow(),
            DayProgress::Unsolved => "".bright_yellow(), // Can't actually see, but required to match types.
        }
    );
}

fn year_menu(year: &AdventOfCodeYear) {
    loop {
        new_menu();
        println!("--- {} ---", year.year);

        // Might be interesting to make an object that handles option menu behaviour
        // It's all very boilerplate. An object could also help by erroring on duplicate option numbers.
        println!("{}", "[-3] - Benchmark".yellow());
        println!("{}", "[-2] - Latest Day".green());
        println!("{}", "[-1] - Go back".red());

        year.days
            .iter()
            .enumerate()
            .for_each(|(idx, day)| print_day(idx, day));

        let answer = match get_stdin_number() {
            Some(n) => n,
            _ => continue,
        };

        let days_length = year.days.len();
        match answer {
            -3 => warn("Not implemented!"),
            -2 => day_menu(days_length - 1, &year.days[days_length - 1], year.year),
            -1 => return,
            x if x <= 0 || x as usize > days_length => invalid_option(),
            x => day_menu((x - 1) as usize, &year.days[(x - 1) as usize], year.year),
        }
    }
}

fn day_menu(idx: usize, day: &AdventOfCodeDay, year: u32) {
    loop {
        new_menu();
        println!("--- Day {} - {} ---", idx + 1, day.name);

        if matches!(day.progress(), DayProgress::Unsolved) {
            warn("Day is not solved!");
            return;
        }

        println!("{}", "[-1] - Go back".red());
        println!("{}", "[0]  - Solve".green());

        let part2_solved = matches!(day.progress(), DayProgress::FullySolved);
        let part1_solved = matches!(day.progress(), DayProgress::PartlySolved) || part2_solved;

        if part1_solved {
            println!("[1]  - Part 1");
        }

        if part2_solved {
            println!("[2]  - Part 2");
        }

        let answer = match get_stdin_number() {
            Some(n) => n,
            _ => continue,
        };

        match answer {
            -1 => return,
            0 => {
                new_menu();
                run_solve(day.part1, year, (idx + 1) as u32);

                if part2_solved {
                    run_solve(day.part2, year, (idx + 1) as u32);
                }

                println!();
                println!("{}", "[-1] - Go back".red());

                loop {
                    match get_stdin_number() {
                        Some(-1) => return,
                        _ => invalid_option(),
                    }
                }
            }
            1 if part1_solved => {
                new_menu();
                run_solve(day.part1, year, (idx + 1) as u32);

                println!();
                println!("{}", "[-1] - Go back".red());

                loop {
                    match get_stdin_number() {
                        Some(-1) => return,
                        _ => invalid_option(),
                    }
                }
            }
            2 if part2_solved => {
                new_menu();
                run_solve(day.part2, year, (idx + 1) as u32);

                println!();
                println!("{}", "[-1] - Go back".red());

                loop {
                    match get_stdin_number() {
                        Some(-1) => return,
                        _ => invalid_option(),
                    }
                }
            }
            _ => invalid_option(),
        }
    }
}

fn run_solve(solve_function: SolveFunction, year: u32, day: u32) {
    if let Ok(data) = get_data(year, day) {
        let result = solve_function
            .expect("run_solve is only called when we know solve_function is Ok()")(
            data
        );

        println!("Result: {result}");
    } else {
        warn("Couldn't load data for current day.");
    }
}
