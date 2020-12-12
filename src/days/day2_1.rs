// Day 2 Part 1 - Password Philosophy
//

//1-3 a: abcde
//1-3 b: cdefg
//2-9 c: ccccccccc

//Each line gives the password policy and then the password. The password policy indicates the
//lowest and highest number of times a given letter must appear for the password to be valid. For
//example, 1-3 a means that the password must contain a at least 1 time and at most 3 times.

//In the above example, 2 passwords are valid. The middle password, cdefg, is not; it contains no
//instances of b, but needs at least 1. The first and third passwords are valid: they contain one a
//or nine c, both within the limits of their respective policies.


use std::str::FromStr;
use std::fmt;

use thiserror::Error;

use crate::utils;


const PASSWORDS: &str = "1-3 a: abcde\n\
    1-3 b: cdefg\n\
    2-9 c: ccccccccc\n\
    ";


#[derive(Error, Debug, Clone)]
pub enum PasswordError {
    #[error("corrupted password line")]
    DecodeError(String),
    #[error("corrupted password rules")]
    RulesError(String),
    #[error("unknown password error")]
    Unknown,
}


#[derive(Debug, Clone)]
struct Rules {
    element: char,
    bounds: (u32, u32),
}


impl fmt::Display for Rules {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "char: {0}, range: {1}-{2}", self.element, self.bounds.0, self.bounds.1)
    }
}


impl FromStr for Rules {
    type Err = PasswordError;

    // parses a '1-3 c' into a Rules of bounds (1,3) and element 'c'
    fn from_str(rules: &str) -> Result<Self, Self::Err> {
        let parts = rules.split(" ").collect::<Vec<_>>();
        if parts.len() != 2 {
            return Err(PasswordError::DecodeError(format!("Missing space in passed string: {}", rules)));
        }
        let bounds = parts[0].split("-").collect::<Vec<_>>();
        if bounds.len() != 2 {
            return Err(PasswordError::RulesError(format!("Missing - in passed bounds: {}", parts[0])));
        }
        let (lower, upper) = (bounds[0].parse::<u32>(), bounds[1].parse::<u32>());
        let lb = lower.map_err(|e| PasswordError::DecodeError(format!("{0}", e)));
        let ub = upper.map_err(|e| PasswordError::DecodeError(format!("{0}", e)));
        let lv = lb?;
        let uv = ub?;
        if lv >= uv {
            return Err(PasswordError::RulesError(format!("Rules: lower bound can't be higher than upper bound!")));
        }
        // now extract the character.
        let chars = parts[1].chars().collect::<Vec<_>>();
        if chars.len() != 1 {
            return Err(PasswordError::DecodeError(format!("element rule must be a single character")));
        }
        Ok(Rules{element: chars[0], bounds: (lv, uv)})
    }

}


#[derive(Debug, Clone)]
struct PasswordRules {
    password: String,
    rules: Rules,
}


impl fmt::Display for PasswordRules {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "password: {0}, rules: {1}", self.password, self.rules)
    }
}


impl FromStr for PasswordRules {
    type Err = PasswordError;

    // parses a '1-3 c: password' into a Rules of bounds (1,3) and element 'c'
    // ignore the problem of a : in the password, by spliting on ": "
    fn from_str(pr: &str) -> Result<Self, Self::Err> {
        let parts = pr.split(": ").collect::<Vec<_>>();
        if parts.len() != 2 {
            return Err(PasswordError::DecodeError(format!("invalid rules and password: {}", pr)));
        }
        let rules = parts[0].parse::<Rules>()?;
        Ok(PasswordRules{password: parts[1].to_string(), rules})
    }
}


fn validate_password(pr: &PasswordRules) -> bool {
    let count = pr.password
        .chars()
        .filter(|c| *c == pr.rules.element)
        .count();
    let c32 = count as u32;
    c32 >= pr.rules.bounds.0 && c32 <= pr.rules.bounds.1
}


pub fn day2_1() {
    println!("First let's just do the test and see if we can parse the password rules:");
    let v = "1-3 c".parse::<Rules>().unwrap();
    println!("The rules are: {0}", v);
    println!("Now try to parse a full set of Password and rules.");
    let p = "1-3 c: abcdceec".parse::<PasswordRules>().unwrap();
    println!("The password + rules are {0}", p);
    // now let's see if that password is validate
    if validate_password(&p) {
        println!("{0} is valid", p);
    } else {
        println!("{0} is not valid", p);
    }
    // now do the block from above.
    let num_valid = PASSWORDS
        .split("\n")
        .map(|l| l.parse::<PasswordRules>())
        .filter(|prr| prr.is_ok())
        .map(|prr| prr.unwrap())
        .filter(|pr| validate_password(pr))
        .count();
    println!("valid passwords from PASSWORDS: {0:?}", num_valid);

    // finally let's process the input file
    println!("\nDoing the input file...");

    let parsed_passwords = utils::read_file::<PasswordRules>("./input/day2-1.txt");
    let num_valid_file = parsed_passwords
        .iter()
        .filter(|&prr| prr.is_ok())
        .cloned()
        .map(|prr| prr.unwrap())
        .filter(|pr| validate_password(pr))
        .count();
        //.collect::<Vec<_>>();
    println!("valid passwords in the file are: {0:?}", num_valid_file);

}
