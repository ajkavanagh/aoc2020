use std::env;
use std::cmp;
use std::process;
use std::io::prelude::*;
use std::fs::OpenOptions;
use std::fs;

mod days;

struct Config {
    day: u32,
    part: u32,
}


const MAX_DAY: u32 = 1;  // update when we add a day

impl Config {

    fn new(args: &[String]) -> Result<Config, String> {
        let num_args = args.len();
        if num_args == 1 {
            return Ok(Config {day: 1, part: 1})
        }
        let command = args[1].to_lowercase();
        let parts = command.split("-").into_iter().collect::<Vec<&str>>();
        if parts.len() != 2 {
            return Err(format!("command '{}' isn't a valid day-part", command));
        }
        let day: u32 = parts[0].parse().unwrap_or(0);
        let part: u32 = parts[1].parse().unwrap_or(0);
        if day < 1 || day > MAX_DAY || part < 1 || part > 2 {
            return Err(format!("day or part is not parsable as an int or in range: input was '{}'", command));
        }
        return Ok(Config{day, part});
    }
}



fn usage() -> Result<(), String> {
    eprintln!("Usage: aoc2020 <day>-<part>");
    Ok(())
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Couldn't parse arguments: {}", err);
        usage().unwrap();
        process::exit(1);
    });
    println!("the day is {}-{}", config.day, config.part);
    days::day1_1::day1_1();
}
