use reqwest::{blocking::Client, header::COOKIE};

use self::{y2019::get_2019_solutions, year::AdventOfCodeYear};
use std::{
    error::Error,
    fs::{create_dir_all, File},
    io::{self, Read, Write},
};

pub mod y2019;
pub mod year;

pub fn get_years() -> Vec<AdventOfCodeYear> {
    vec![get_2019_solutions()]
}

pub fn get_data(year: u32, day: u32) -> Result<String, Box<dyn Error>> {
    read_cache(year, day).or_else(|_| {
        let session = get_session()
            .expect("Couldn't get session key to request data from. Add it to /data/session.txt");
        let data = request_data(year, day, session)?;

        let _ = write_cache(year, day, &data).or_else(|e| {
            println!("Couldn't write to cache!");
            Err(e)
        });

        Ok(data)
    })
}

fn get_session() -> Result<String, io::Error> {
    let mut file = File::open(format!("data/session.txt"))?;
    let mut data = String::new();

    file.read_to_string(&mut data)?;

    Ok(data.trim().to_string())
}

fn read_cache(year: u32, day: u32) -> Result<String, io::Error> {
    let mut file = File::open(format!("data/cache/{year}/day{day}.txt"))?;
    let mut data = String::new();

    file.read_to_string(&mut data)?;

    Ok(data)
}

fn write_cache(year: u32, day: u32, data: &String) -> Result<(), Box<dyn Error>> {
    create_dir_all(format!("data/cache/{year}"))?;

    let mut file = File::create(format!("data/cache/{year}/day{day}.txt"))?;
    file.write(data.as_bytes())?;

    Ok(())
}

fn request_data(year: u32, day: u32, session: String) -> Result<String, Box<dyn Error>> {
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
