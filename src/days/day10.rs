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


fn count_intervals(numbers: &[u32]) -> HashMap<u32, u32> {
    let mut counts = HashMap::new();
    for i in 0..numbers.len()-1 {
        let diff = numbers[i+1] - numbers[i];
        *counts.entry(diff).or_insert(0) += 1;
    }
    // add in the 1 count and the a final additional 3 count.
    *counts.entry(1).or_insert(0) += 1;
    *counts.entry(3).or_insert(0) += 1;
    counts
}


pub fn day10_1() {
    //let numbers_results = utils::read_file::<u32>("./input/day10-test-data.txt");
    let numbers_results = utils::read_file::<u32>("./input/day10.txt");
    let mut numbers = numbers_results.iter().map(|v| v.clone().unwrap()).collect::<Vec<_>>();
    println!("{:?}", numbers);
    numbers.sort();
    println!("sorted {:?}", numbers);
    let counts = count_intervals(numbers.as_slice());
    println!("counts {:?}", counts);
    let ones = counts.get(&1).unwrap();
    let threes = counts.get(&3).unwrap();
    println!("result is {}", ones * threes);
}
