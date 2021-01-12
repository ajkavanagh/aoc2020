use std::str::FromStr;
use std::fmt;
use std::cell::RefCell;
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
    // add in the final additional 3 count.
    *counts.entry(3).or_insert(0) += 1;
    counts
}


pub fn day10_1() {
    //let numbers_results = utils::read_file::<u32>("./input/day10-test-data.txt");
    let numbers_results = utils::read_file::<u32>("./input/day10.txt");
    let mut numbers = numbers_results.iter().map(|v| v.clone().unwrap()).collect::<Vec<_>>();
    // add in a 0 if it doesn't exist.
    if !numbers.contains(&0) {
        numbers.push(0);
    }
    println!("{:?}", numbers);
    numbers.sort();
    println!("sorted {:?}", numbers);
    let counts = count_intervals(numbers.as_slice());
    println!("counts {:?}", counts);
    let ones = counts.get(&1).unwrap();
    let threes = counts.get(&3).unwrap();
    println!("result is {}", ones * threes);
}


// From a user on reddit: https://www.reddit.com/r/adventofcode/comments/ka8z8x/2020_day_10_solutions/gf9pg9n/
//with open('day-10-input.txt', 'r') as f:
    //adapters = list(map(int, f.read().split('\n')))
//adapters.sort()
//adapters = adapters + [max(adapters) + 3]

//ans = {}
//ans[0] = 1
//for a in adapters:
    //ans[a] = ans.get(a - 1, 0) + ans.get(a - 2, 0) + ans.get(a - 3, 0)

//print(f'Answer: {ans[adapters[-1]]}'


// rust version of the above.
fn count_paths(adapters: &[u32]) -> i64 {
    let mut results = HashMap::with_capacity(adapters.len());
    results.insert(0, 1);
    for n in adapters.iter() {
        let v = *n as i64;
        results.insert(
            v,
            results.get(&(v-1)).unwrap_or(&0) +
            results.get(&(v-2)).unwrap_or(&0) +
            results.get(&(v-3)).unwrap_or(&0));
    }
    let last = adapters[adapters.len()-1] as i64;
    *results.get(&last).unwrap()
}


pub fn day10_2() {
    //let numbers_results = utils::read_file::<u32>("./input/day10-test-data.txt");
    //let numbers_results = utils::read_file::<u32>("./input/day10-small-test-data.txt");
    let numbers_results = utils::read_file::<u32>("./input/day10.txt");
    let mut numbers = numbers_results.iter().map(|v| v.clone().unwrap()).collect::<Vec<_>>();
    println!("{:?}", numbers);
    numbers.sort();
    println!("sorted {:?}", numbers);
    println!("counted paths: {}", count_paths(&numbers));
}
