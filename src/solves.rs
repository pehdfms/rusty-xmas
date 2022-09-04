use reqwest::{blocking::Client, header::COOKIE};

use self::{y2019::get_2019_solutions, year::AdventOfCodeYear};
use std::{
    error::Error,
    fs::{create_dir_all, File},
    io::{self, Read, Write},
};

pub mod y2019;
pub mod year;

#[must_use]
pub fn get_years<'a>() -> Vec<AdventOfCodeYear<'a>> {
    vec![get_2019_solutions()]
}

/// # Errors
/// This function errors if it can't request data from Advent of Code inputs.
pub fn get_data(year: u64, day: u64) -> Result<String, Box<dyn Error>> {
    read_cache(year, day).or_else(|_| {
        let session = get_session()
            .expect("Couldn't get session key to request data from. Add it to /data/session.txt");
        let data = request_data(year, day, &session)?;

        let _unused_result = write_cache(year, day, &data).map_err(|e| {
            println!("Couldn't write to cache!");
            e
        });

        Ok(data)
    })
}

fn get_session() -> Result<String, io::Error> {
    let mut file = File::open("data/session.txt")?;
    let mut data = String::new();

    file.read_to_string(&mut data)?;

    Ok(data.trim().to_string())
}

fn read_cache(year: u64, day: u64) -> Result<String, io::Error> {
    let mut file = File::open(format!("data/cache/{year}/day{day}.txt"))?;
    let mut data = String::new();

    file.read_to_string(&mut data)?;

    Ok(data)
}

fn write_cache(year: u64, day: u64, data: &String) -> Result<(), Box<dyn Error>> {
    create_dir_all(format!("data/cache/{year}"))?;

    let mut file = File::create(format!("data/cache/{year}/day{day}.txt"))?;
    file.write_all(data.as_bytes())?;

    Ok(())
}

fn request_data(year: u64, day: u64, session: &str) -> Result<String, Box<dyn Error>> {
    let client = Client::builder().cookie_store(true).build()?;
    let mut data = String::new();

    let url = format!("https://adventofcode.com/{year}/day/{day}/input");
    client
        .get(url)
        .header(COOKIE, format!("session={session}"))
        .send()?
        .read_to_string(&mut data)?;

    Ok(data)
}
