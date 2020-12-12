// Day 2 Part 2 - Password Philosophy with a change
//

//1-3 a: abcde
//1-3 b: cdefg
//2-9 c: ccccccccc



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


// validate the password using the following rule:
//
//    1-3 a: abcde is valid: position 1 contains a and position 3 does not.
//    1-3 b: cdefg is invalid: neither position 1 nor position 3 contains b.
//    2-9 c: ccccccccc is invalid: both position 2 and position 9 contain c.
fn validate_password(pr: &PasswordRules) -> bool {
    let f1 = pr.password.chars().nth((pr.rules.bounds.0 as usize)-1)
        .map(|c| c == pr.rules.element)
        .unwrap_or_else(|| false);
    let f2 = pr.password.chars().nth((pr.rules.bounds.1 as usize)-1)
        .map(|c| c == pr.rules.element)
        .unwrap_or_else(|| false);
    f1 ^ f2
}


pub fn day2_2() {
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
