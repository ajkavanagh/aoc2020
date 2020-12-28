use std::str::FromStr;
use std::fmt;
use std::collections::BTreeMap;


use thiserror::Error;

use crate::utils;

#[derive(Debug)]
struct Questions {
    questions: BTreeMap<char, usize>,
    lines: Vec<String>,
}


#[derive(Error, Debug, Clone)]
pub enum DecodeError {
    #[error("Corrupt questions")]
    CorruptError(String),
    #[error("unknown passport error")]
    Unknown,
}


impl Questions {

    fn from_lines(lines: &[&str]) -> Result<Self, DecodeError> {
        let mut qs = BTreeMap::new();
        let mut ls = Vec::new();
        for line in lines {
            ls.push(line.to_string());
            for c in line.chars() {
                if c.is_ascii_lowercase() {
                    *qs.entry(c).or_insert(0) += 1;
                } else {
                    return Err(DecodeError::CorruptError(format!("line is invalid: {}", line)));
                }
            }
        }
        Ok(Questions{questions: qs, lines: ls})
    }

    fn num_q(&self) -> u32 {
        self.questions.len() as u32
    }
}


fn process_fn(file_name: &str) -> Vec<Result<Questions, DecodeError>> {
    let lines = std::fs::read_to_string(file_name)
        .expect("file not found!");
    let questions = utils::process_lines_to_batches(lines.lines().collect::<Vec<_>>().as_slice())
        .iter()
        .map(|batch| Questions::from_lines(batch))
        .collect::<Vec<_>>();
    questions
}


fn sum_num_questions(qs: &[&Questions]) -> u32 {
    qs.iter().map(|q| q.num_q()).sum()
}


pub fn day6_1() {
    println!("Day 6_1.");
    let files = ["./input/day6-test-data.txt", "./input/day6.txt"];
    for file in files.iter() {
        let qs = process_fn(file);
        let qsu = qs.iter().map(|q| q.as_ref().unwrap()).collect::<Vec<_>>();
        for q in &qsu {
            println!("{:?}", q);
        }
        println!("Sum of questions: {}", sum_num_questions(qsu.as_slice()));
    }
}
