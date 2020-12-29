use std::str::FromStr;
use std::fmt;
use std::collections::{
    HashMap,
    HashSet,
};


use thiserror::Error;

use crate::utils;

#[derive(Debug)]
struct Rule {
    bag: String,
    contains: Vec<(u32, String)>,
}

type ContainsMap = HashMap::<String, HashSet<String>>;

#[derive(Error, Debug, Clone)]
pub enum DecodeError {
    #[error("Corrupt questions")]
    CorruptError(String),
    #[error("unknown passport error")]
    Unknown,
}


impl FromStr for Rule {
    type Err = DecodeError;

    // parses a '1-3 c' into a Rules of bounds (1,3) and element 'c'
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(" contain ").collect::<Vec<_>>();
        if parts.len() != 2 {
            return Err(DecodeError::CorruptError(format!("line is malformed: {}", s)));
        }
        let bag_bits = parts[0].trim().split(" ").collect::<Vec<_>>();
        let bag = format!("{} {}", bag_bits[0], bag_bits[1]);
        // split the remaining line into bags via the ','
        let chunks = parts[1].split(',').collect::<Vec<_>>();
        let mut contains = Vec::new();
        for chunk in chunks {
            if chunk.starts_with("no") {
                break;
            }
            // otherwise the line is  <n> bag type <ignored>
            let bits = chunk.trim().split(" ").collect::<Vec<_>>();
            if bits.len() != 4 {
                return Err(DecodeError::CorruptError(format!("line {} has wrong num chunks", s)));
            }
            let num = bits[0].parse::<u32>().unwrap();
            let rule = format!("{} {}", bits[1], bits[2]);
            contains.push((num, rule));
        }

        Ok(Rule {bag: bag.to_owned(), contains})
    }
}


fn containedby_map(rules: &[&Rule]) -> ContainsMap {
    let mut map = HashMap::new();
    for rule in rules.iter() {
        for bag in rule.contains.iter() {
            map.entry(bag.1.clone()).or_insert(HashSet::new()).insert(rule.bag.clone());
        }
    }
    map
}


/// return the bags that can contain (ultimately) a particular bag
fn held_by(map: &ContainsMap, bag: &str) -> HashSet<String> {
    let mut result = HashSet::new();
    if let Some(bags) = map.get(bag) {
        for cbag in bags {
            result.insert(cbag.clone());
            for rbag in held_by(map, cbag) {
                result.insert(rbag);
            }
        }
    }
    result
}


pub fn day7_1() {
    // let's grab the test file
    //let test_rules = utils::read_file::<Rule>("./input/day7-test-data.txt");
    let test_rules = utils::read_file::<Rule>("./input/day7.txt");
    let rules = test_rules.iter().map(|v| v.as_ref().unwrap()).collect::<Vec<_>>();
    for r in &rules {
        println!("{:?}", r);
    }
    println!("contained map:");
    let map = containedby_map(rules.as_slice());
    println!("{:?}", map);
    println!("see what is contained by 'shiny gold'");
    let bags = held_by(&map, "shiny gold");
    println!("{:?}", bags);
    println!("length: {}", bags.len());

}
