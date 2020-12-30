use std::str::FromStr;
use std::fmt;
use std::collections::{
    HashMap,
    HashSet,
};


use thiserror::Error;

use crate::utils;


#[derive(Error, Debug, Clone)]
pub enum DecodeError {
    #[error("Corrupt questions")]
    CorruptError(String),
    #[error("Invalid Opcode")]
    InvalidOpCode(String),
    #[error("unknown passport error")]
    Unknown,
}


fn find_invalid(numbers: &[u64], window: usize) -> Option<u64> {
    if window >= numbers.len() {
        return None;
    }
    'search: for pos in window..numbers.len() {
        let target = numbers[pos];
        println!("Testing: {} @{}", target, pos);
        // now check all the pairs of possible numbers between pos-window and pos-1 for a sum that
        // adds up to target:
        for lower in pos-window .. pos-1 {
            for upper in lower+1 .. pos {
                println!("for: {}, trying pair {}@{} and {}@{} == {}", target,
                    numbers[lower], lower, numbers[upper], upper, numbers[lower] + numbers[upper]);
                if numbers[lower] + numbers[upper] == target {
                    continue 'search;
                }
            }
        }
        // didn't find a sum, so let'd return it:
        return Some(target);
    }
    None
}

pub fn day9_1() {
    //let numbers_results = utils::read_file::<u64>("./input/day9-test-data.txt");
    let numbers_results = utils::read_file::<u64>("./input/day9.txt");
    let numbers = numbers_results.iter().map(|v| v.clone().unwrap()).collect::<Vec<_>>();
    //println!("{:?}", numbers);
    println!("Invalid num: {:?}", find_invalid(&numbers, 25));
}



// find a minimal (early) sequence of numbers that adds up to number, and return the sum of the
// first and last of tham.
fn find_sequence_num(numbers: &[u64], number: u64) -> Option<u64> {
    let size = numbers.len();
    'search: for lower in 0..size {
        let mut sum = numbers[lower];
        for upper in lower+1..=size {
            sum += numbers[upper];
            // found if sum matches the number
            if sum == number {
                // find smallest, and largest number in the range lower..=upper and add them.
                let mut largest: u64 = 0;
                let mut smallest: u64 = std::u64::MAX;
                for i in lower..=upper {
                    if numbers[i] > largest { largest = numbers[i];}
                    if numbers[i] < smallest { smallest = numbers[i];}
                }
                return Some(smallest + largest);
            }
            // gone too far so try next number in the list
            if sum > number {
                continue 'search;
            }
        }
    }
    None
}

pub fn day9_2() {
    //let numbers_results = utils::read_file::<u64>("./input/day9-test-data.txt");
    let numbers_results = utils::read_file::<u64>("./input/day9.txt");
    let numbers = numbers_results.iter().map(|v| v.clone().unwrap()).collect::<Vec<_>>();
    //println!("{:?}", numbers);
    if let Some(invalid_num) = find_invalid(&numbers, 25) {
        println!("Invalid num: {}", invalid_num); 
        println!("Sequence num: {:?}", find_sequence_num(&numbers, invalid_num));
    }
}

