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

type RulesMap = HashMap::<String, Vec<(u32, String)>>;


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


/// make a rulesmap
fn rules_map(rules: &[&Rule]) -> RulesMap {
    let mut map = HashMap::new();
    for rule in rules.iter() {
        if map.contains_key(&rule.bag) {
            panic!("map already contains {}", rule.bag);
        }
        map.insert(rule.bag.clone(), rule.contains.clone());
    }
    map
}

/// must contain (e.g. follow the rules and contain all the sub-bags)
fn num_contains(map: &RulesMap, bag: &str) -> u32 {
    let mut count = 0;
    if let Some(rules) = map.get(bag) {
        for rule in rules {
            count += rule.0;
            count += rule.0 * num_contains(map, &rule.1);
        }
    }
    count
}

pub fn day7_2() {
    // let's grab the test file
    //let test_rules = utils::read_file::<Rule>("./input/day7-test-data.txt");
    let test_rules = utils::read_file::<Rule>("./input/day7.txt");
    let rules = test_rules.iter().map(|v| v.as_ref().unwrap()).collect::<Vec<_>>();
    for r in &rules {
        println!("{:?}", r);
    }
    println!("rules map:");
    let map = rules_map(rules.as_slice());
    println!("{:?}", map);
    println!("see number of bags in a 'shiny gold'");
    let bags = num_contains(&map, "shiny gold");
    println!("{}", bags);

}
