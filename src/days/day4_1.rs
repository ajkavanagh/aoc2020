use std::str::FromStr;
use std::fmt;
use std::collections::HashMap;


use thiserror::Error;

use crate::utils;


const PASSPORTS: &str =
   "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\n\
    byr:1937 iyr:2017 cid:147 hgt:183cm\n\
    \n\
    iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884\n\
    hcl:#cfa07d byr:1929\n\
    \n\
    hcl:#ae17e1 iyr:2013\n\
    eyr:2024\n\
    ecl:brn pid:760753108 byr:1931\n\
    hgt:179cm\n\
    \n\
    hcl:#cfa07d eyr:2025 pid:166559648\n\
    iyr:2011 ecl:brn hgt:59in\n\
    ";


#[derive(Error, Debug, Clone)]
pub enum PassportError {
    #[error("Missing fields")]
    MissingError(String),
    #[error("corrupted passport line")]
    CorruptedError(String),
    #[error("Couldn't decode part")]
    ParseError(String),
    #[error("unknown passport error")]
    Unknown,
}


#[derive(Debug)]
struct Passport {
    ecl: String,
    pid: String,
    eyr: u32,
    hcl: String,
    byr: u32,
    iyr: u32,
    cid: Option<u32>,
    hgt: String,        // in cm (string because it includes a value)
}


impl FromStr for Passport {
    type Err = PassportError;

    // parses a complete passport string into a Passport struct
    // e.g. "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd byr:1937 iyr:2017 cid:147 hgt:183cm"
    // Note that cid is optional
    fn from_str(line: &str) -> Result<Self, Self::Err> {
        // split each part from the others so that they can be collected.
        let parts = line.split(" ").collect::<Vec<_>>();

        let mandatory = ["ecl", "pid", "eyr", "hcl", "byr", "iyr", "hgt"];

        // extract all the named bits.
        let mut matches = HashMap::new();
        for part in parts {
            let bits = part.split(":").collect::<Vec<_>>();
            if bits.len() != 2 {
                return Err(PassportError::CorruptedError(format!("part '{}' is corrupt?", part)));
            }
            let name = bits[0].to_lowercase();
            if matches.contains_key(&name) {
                return Err(PassportError::CorruptedError(format!("part '{} is duplicated", part)));
            }
            matches.insert(name, bits[1]);
        }
        // now make sure we have enough bits (cid is optional)
        let mut missing_keys = Vec::new();
        for key in mandatory.iter() {
            if !matches.contains_key(*key) {
                missing_keys.push(key.to_string());
            }
        }
        if missing_keys.len() != 0 {
            return Err(PassportError::MissingError(format!("missing keys in passport: {}",
                        missing_keys.join(", "))));
        }
        // finally, let's build the passport (with the optional cid) -- this may error if it can't
        // parse the relevant bits.
        let ecl = matches.get("ecl").unwrap().to_string();
        let pid = matches.get("pid").unwrap().to_string();
        let eyr = matches.get("eyr").unwrap().parse::<u32>().map_err(
            |e| PassportError::ParseError(format!("eyr parse error on {}: {}",
                    matches.get("eyr").unwrap(), e)))?;
        let hcl = matches.get("hcl").unwrap().to_string();
        let byr = matches.get("byr").unwrap().parse::<u32>().map_err(
            |e| PassportError::ParseError(format!("byr parse error on {}: {}",
                    matches.get("byr").unwrap(), e)))?;
        let iyr = matches.get("iyr").unwrap().parse::<u32>().map_err(
            |e| PassportError::ParseError(format!("iyr parse error on {}: {}",
                    matches.get("iyr").unwrap(), e)))?;
        // cid is optional
        let cid = if let Some(cid_) = matches.get("cid") {
            let parsed = cid_.parse::<u32>().map_err(|e| PassportError::ParseError(format!(
                "cid parse error on {}: {}", cid_, e)))?;
            Some(parsed)
        } else { None };
        let hgt = matches.get("hgt").unwrap().to_string();

        Ok(Passport { ecl, pid, eyr, hcl, byr, iyr, cid, hgt })
    }
}

// restructure in the input into a series of lines of passport
fn restructure_input(input: &str) -> Vec<String> {
    let mut line = String::new();
    let mut lines = Vec::new();
    for l in input.lines() {
        let stripped = l.trim();
        if l.is_empty() {
            lines.push(line.clone());
            line.clear();
        } else {
            if !line.is_empty() {
                line.push(' ');
            }
            line.push_str(stripped);
        }
    }
    if !line.is_empty() {
        lines.push(line.clone());
    }
    lines
}


pub fn day4_1() {
    println!("Day 4_1.");
    let passport_lines = restructure_input(PASSPORTS);
    println!("{}", passport_lines.join("\n"));
    // parse the passport_lines into passports.
    let passports = passport_lines
        .iter()
        .map(|l| l.parse::<Passport>())
        .collect::<Vec<_>>();
    for passport in &passports {
        println!("{:?}", passport);
    }
    let mut count: usize = 0;
    for passport in &passports {
        if passport.is_ok() {
            count += 1;
        }
    }
    println!("Valid passports in test: {}", count);

    // now lets count the ones in the file.
    let rpassport_data = std::fs::read_to_string("./input/day4-1.txt").unwrap();
    let rpassport_lines = restructure_input(&rpassport_data);
    let rpassports = rpassport_lines
        .iter()
        .map(|l| l.parse::<Passport>())
        .collect::<Vec<_>>();

    let mut count: usize = 0;
    for passport in &rpassports {
        if passport.is_ok() {
            count += 1;
        }
    }
    println!("Valid passports in file: {}", count);

}


