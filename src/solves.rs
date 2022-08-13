use self::{y2019::get_2019_solutions, year::AdventOfCodeYear};
use std::{
    error::Error,
    fs::{create_dir_all, File},
    io,
};

pub mod y2019;
pub mod year;

pub fn get_years() -> Vec<AdventOfCodeYear> {
    vec![get_2019_solutions()]
}

pub fn get_data(year: u32, day: u32) -> Result<String, Box<dyn Error>> {
    write_cache(year, day);
    Ok(String::from("data"))
}

pub fn write_cache(year: u32, day: u32) -> Result<(), io::Error> {
    create_dir_all(format!("cache/{year}"))?;

    let mut f = File::create(format!("cache/{year}/day{day}.txt"))?;

    Ok(())
}
