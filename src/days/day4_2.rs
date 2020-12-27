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
    #[error("Not valid for part")]
    InvalidPart(String),
    #[error("unknown passport error")]
    Unknown,
}


#[derive(Debug)]
enum HeightUnit {
    UnitCM,
    UnitIN,
}


#[derive(Debug)]
struct Passport {
    ecl: String,
    pid: String,
    eyr: u32,
    hcl: u64,
    byr: u32,
    iyr: u32,
    cid: Option<u32>,
    //hgt: String,        // in cm (string because it includes a value)
    hgt: (HeightUnit, u32),   // enum and value
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
        // validate ecl is one of the hair colours allowed.
        let ecl = matches.get("ecl").unwrap().to_string();
        let valid_ecl = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
        let mut valid = false;
        for v in valid_ecl.iter() {
            if *v == ecl {
                valid = true;
                break;
            }
        }
        if !valid {
            return Err(PassportError::InvalidPart(format!("ecl - invalid hair colour: {}", ecl)));
        }

        // validate the passport ID - a nine-digit number, including leading zeroes.
        let pid = matches.get("pid").unwrap().to_string();
        let pid_chars = pid.chars().collect::<Vec<_>>();
        if pid_chars.len() != 9 {
            return Err(PassportError::InvalidPart(format!("pid - invalid length: {}", pid)));
        }
        let mut invalid_pid_chars: Vec<char> = Vec::new();
        for pid_char in pid_chars {
            if !pid_char.is_digit(10) {
                invalid_pid_chars.push(pid_char);
            }
        }
        if !invalid_pid_chars.is_empty() {
            return Err(PassportError::InvalidPart(format!("pid - invalid chars in passport number: {}", pid)));
        }

        // validate expiry year
        let eyr = matches.get("eyr").unwrap().parse::<u32>().map_err(
            |e| PassportError::ParseError(format!("eyr parse error on {}: {}",
                    matches.get("eyr").unwrap(), e)))?;
        if eyr < 2020 || eyr > 2030 {
            return Err(PassportError::InvalidPart(format!("eyr invalid year: {}", eyr)));
        }
        // validate haircolour - string in form '#abcdef (hex)
        let hcl = matches.get("hcl").unwrap().to_string();
        let hcls = hcl.chars().collect::<Vec<_>>();
        if hcls.len() != 7 {
            return Err(PassportError::InvalidPart(format!("hcl invalid - not 7 chars: {}", hcl)));
        }
        if hcls[0] != '#' {
            return Err(PassportError::InvalidPart(format!("hcl invalid - doesn't start with #: {}", hcl)));
        }
        let hex_str: String = hcls.iter().skip(1).collect();
        let raw_hex = hex::decode(hex_str).map_err(|e| PassportError::InvalidPart(
                format!("hcl: hex parse error on {}: {}", hcl, e)))?;
        let mut hcl_value: u64 = 0;
        for byte in raw_hex {
            hcl_value *= 16;
            hcl_value += byte as u64;
        }

        // validate birth year
        let byr = matches.get("byr").unwrap().parse::<u32>().map_err(
            |e| PassportError::ParseError(format!("byr parse error on {}: {}",
                    matches.get("byr").unwrap(), e)))?;
        if byr < 1919 || byr > 2002 {
            return Err(PassportError::InvalidPart(format!("byr invalid year: {}", byr)));
        }

        // validate issue year
        let iyr = matches.get("iyr").unwrap().parse::<u32>().map_err(
            |e| PassportError::ParseError(format!("iyr parse error on {}: {}",
                    matches.get("iyr").unwrap(), e)))?;
        if iyr < 2010 || iyr > 2020 {
            return Err(PassportError::InvalidPart(format!("iyr invalid year: {}", iyr)));
        }

        // validate optional cid
        let cid = if let Some(cid_) = matches.get("cid") {
            let parsed = cid_.parse::<u32>().map_err(|e| PassportError::ParseError(format!(
                "cid parse error on {}: {}", cid_, e)))?;
            Some(parsed)
        } else { None };

        // validate the height
        let hgt = matches.get("hgt").unwrap().to_string();
        if hgt.len() < 3 {
            return Err(PassportError::InvalidPart(format!("hgt invalid: {}", hgt)));
        }
        let last_two: String = hgt.chars().rev().take(2).collect();
        let height_str: String = hgt.chars().take(hgt.len() -2).collect();
        // parse the height str
        let height = height_str.parse::<u32>().map_err(|e| PassportError::ParseError(format!(
                "hgt parse error: can't get number from {} (tried {}): {}", hgt, height_str, e)))?;
        if last_two == "ni" {
            // deal with inches
            if height < 59 || height > 76 {
                return Err(PassportError::InvalidPart(format!("hgt: height is not in range {} in",
                            height)));
            }
        } else if last_two == "mc" {
            // deal with cm
            if height < 150 || height > 193 {
                return Err(PassportError::InvalidPart(format!("hgt: height is not in range {} in",
                            height)));
            }
        } else {
            return Err(PassportError::InvalidPart(format!("hgt is not cm or in: {}", hgt)));
        }
        let height_unit = if last_two == "ni" { HeightUnit::UnitIN } else { HeightUnit::UnitCM };

        Ok(Passport { ecl, pid, eyr, hcl: hcl_value, byr, iyr, cid, hgt: (height_unit, height) })
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


pub fn day4_2() {
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
