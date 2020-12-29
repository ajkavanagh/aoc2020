use std::str::FromStr;
use std::fmt;
use std::collections::{
    HashMap,
    HashSet,
};


use thiserror::Error;

use crate::utils;


#[derive(Debug, PartialEq)]
enum OpCodeType {
    ACC,
    JMP,
    NOP,
    UNKNOWN,
}


#[derive(Debug)]
struct OpCode {
    code: OpCodeType,
    argument: i32,
}

#[derive(Error, Debug, Clone)]
pub enum DecodeError {
    #[error("Corrupt questions")]
    CorruptError(String),
    #[error("Invalid Opcode")]
    InvalidOpCode(String),
    #[error("unknown passport error")]
    Unknown,
}


fn code_to_opcodetype(s: &str) -> Result<OpCodeType, DecodeError> {
    let res = match s {
        "acc" => OpCodeType::ACC,
        "jmp" => OpCodeType::JMP,
        "nop" => OpCodeType::NOP,
        _     => OpCodeType::UNKNOWN,
    };
    if res == OpCodeType::UNKNOWN {
        return Err(DecodeError::InvalidOpCode(s.to_string()));
    }
    Ok(res)
}


impl FromStr for OpCode {
    type Err = DecodeError;

    // parses a '1-3 c' into a Rules of bounds (1,3) and element 'c'
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(" ").collect::<Vec<_>>();
        if parts.len() != 2 {
            return Err(DecodeError::CorruptError(format!("line is malformed: {}", s)));
        }
        let opcode = code_to_opcodetype(parts[0])?;
        let value = parts[1].parse::<i32>().map_err(|e| DecodeError::CorruptError(format!("{}", e)))?;
        Ok(OpCode {code: opcode, argument: value})
    }
}


// run the opcodes from 0 until we loop.  keep a record of what we have hit using a hashset.
// use the acc and pc (program counter) to run the machine until it loops.
fn run_til_loop(opcodes: &[&OpCode]) -> i32 {
    let mut acc = 0;
    let mut pc: usize = 0;
    let mut visited = HashSet::new();
    loop {
        if visited.contains(&pc) {
            return acc;
        }
        visited.insert(pc);
        let opcode = &opcodes[pc];
        match opcode.code {
            OpCodeType::ACC => {
                acc += opcode.argument;
                pc += 1;
            },
            OpCodeType::JMP => {
                pc = ((pc as i32) + opcode.argument) as usize;
            },
            OpCodeType::NOP => {
                pc += 1;
            },
            OpCodeType::UNKNOWN => {
                panic!("Shouldn't be able to get here!");
            },
        }
    }
}

pub fn day8_1() {
    // let's grab the test file
    //let opcode_results = utils::read_file::<OpCode>("./input/day8-test-data.txt");
    let opcode_results = utils::read_file::<OpCode>("./input/day8.txt");
    let opcodes = opcode_results.iter().map(|v| v.as_ref().unwrap()).collect::<Vec<_>>();
    for r in &opcodes {
        println!("{:?}", r);
    }
    let acc = run_til_loop(&opcodes);
    println!("Acc just before repeat is: {}", acc);
}

