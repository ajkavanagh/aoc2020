use std::env;
use std::cmp;
use std::process;
use std::io::prelude::*;
use std::fs::OpenOptions;
use std::fs;

mod days;
mod utils;

struct Config {
    day: u32,
    part: u32,
}


const MAX_DAY: u32 = 25;  // update when we add a day

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
            return Err(format!("day or part is not parsable as an int or not in range: input was '{}'", command));
        }
        return Ok(Config{day, part});
    }
}



fn usage() -> Result<(), String> {
    eprintln!("Usage: aoc2020 <day>-<part>");
    Ok(())
}


fn run_day_part(day: u32, part: u32) {
    match (day, part) {
        (1,1) => days::day1_1::day1_1(),
        (1,2) => days::day1_2::day1_2(),
        (2,1) => days::day2_1::day2_1(),
        (2,2) => days::day2_2::day2_2(),
        (3,1) => days::day3_1::day3_1(),
        (3,2) => days::day3_2::day3_2(),
        (4,1) => days::day4_1::day4_1(),
        (4,2) => days::day4_2::day4_2(),
        (5,1) => days::day5_1::day5_1(),
        (5,2) => days::day5_2::day5_2(),
        (6,1) => days::day6_1::day6_1(),
        (6,2) => days::day6_2::day6_2(),
        (7,1) => days::day7_1::day7_1(),
        (7,2) => days::day7_2::day7_2(),
        (8,1) => days::day8_1::day8_1(),
        (8,2) => days::day8_2::day8_2(),
        (9,1) => days::day9::day9_1(),
        (9,2) => days::day9::day9_2(),
        (10,1) => days::day10::day10_1(),
        (10,2) => days::day10::day10_2(),
        _ => println!("Day {0}-{1} not defined (yet?)", day, part),
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Couldn't parse arguments: {}", err);
        usage().unwrap();
        process::exit(1);
    });
    println!("the day is {}-{}", config.day, config.part);
    run_day_part(config.day, config.part);
}
