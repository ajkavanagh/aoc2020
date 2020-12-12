// Day 1 Part 1 - Expense report
//
// Specifically, they need you to find the two entries that sum to 2020 and then multiply those two
// numbers together.

//For example, suppose your expense report contained the following:

//1721
//979
//366
//299
//675
//1456

//In this list, the two entries that sum to 2020 are 1721 and 299. Multiplying them together
//produces 1721 * 299 = 514579, so the correct answer is 514579.

use std::str::FromStr;
use std::num::ParseIntError;

use crate::utils;

const NUMBERS: [u32; 6] = [1721, 979, 366, 299, 675, 1456];


fn find_pair(numbers: &[u32]) -> Result<(u32, u32), String> {
    for n1 in 0..(numbers.len() - 1) {
        for n2 in n1..numbers.len() {
            let v1 = numbers[n1];
            let v2 = numbers[n2];
            if v1 + v2 == 2020 {
                return Ok((v1, v2));
            }
        }
    }
    Err("Couldn't find a pair that adds to 2020".to_string())
}


fn extract_numbers(parsed: &[Result<u32, ParseIntError>]) -> Vec<u32> {
    parsed
        .iter()
        .map(|x| {
            match *x {
                Ok(v) => v,
                Err(_) => 0 as u32,
            }
        })
        .collect()
}

pub fn day1_1() {
    println!("First let's just do the test:");
    match find_pair(&NUMBERS) {
        Ok((v1,v2)) => println!("The numbers are {0} * {1} = {2}", v1, v2, v1 * v2),
        Err(s) => println!("{0}", s),
    }

    println!("Now let's read the expenses file and then find in tht file:");
    let parsed_numbers = utils::read_file::<u32>("./input/day1-1.txt");
    let numbers = extract_numbers(&parsed_numbers);
    match find_pair(&numbers) {
        Ok((v1,v2)) => println!("The numbers are {0} * {1} = {2}", v1, v2, v1 * v2),
        Err(s) => println!("{0}", s),
    }
}
