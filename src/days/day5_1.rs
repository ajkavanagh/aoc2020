use std::str::FromStr;
use std::fmt;
use std::collections::HashMap;


use thiserror::Error;

use crate::utils;


#[derive(Debug)]
struct Seat {
    bp: String,
    row: u32,
    col: u32,
}


#[derive(Error, Debug, Clone)]
pub enum SeatError {
    #[error("Corrupt boarding pass")]
    CorruptError(String),
    #[error("unknown passport error")]
    Unknown,
}


fn consume_bp(code: &str) -> Result<Seat, SeatError> {
    let cs = code.chars().collect::<Vec<_>>();
    if cs.len() != 10 {
        return Err(SeatError::CorruptError(format!("{} is not 10 characters", code)));
    }
    // figure out row
    let mut row = 0;
    for fb in cs.iter().take(7) {
        row *= 2;
        if *fb == 'B' {
            row += 1;
        } else if *fb != 'F' {
            return Err(SeatError::CorruptError(format!("{} contains invalid FB", code)));
        }
    }
    // figure out column
    let mut col = 0;
    for lr in cs.iter().skip(7) {
        col *= 2;
        if *lr == 'R' {
            col += 1;
        } else if *lr != 'L' {
            return Err(SeatError::CorruptError(format!("{} contains invalid LR", code)));
        }
    }
    Ok(Seat {bp: code.to_string(), row, col})
}


impl FromStr for Seat {
    type Err = SeatError;

    // parses a '1-3 c' into a Rules of bounds (1,3) and element 'c'
    fn from_str(bp: &str) -> Result<Self, Self::Err> {
        consume_bp(bp)
    }
}


fn seat_id(seat: &Seat) -> u32 {
    seat.row * 8 + seat.col
}


pub fn day5_1() {
    println!("Day 5_1.");
    let seat1 = consume_bp("FBFBBFFRLR").unwrap();
    println!("{:?} code {}", seat1, seat_id(&seat1));
    let seat2 = consume_bp("BFFFBBFRRR").unwrap();
    println!("{:?} code {}", seat2, seat_id(&seat2));
    let seat3 = consume_bp("FFFBBBFRRR").unwrap();
    println!("{:?} code {}", seat3, seat_id(&seat3));
    let seat4 = consume_bp("BBFFBBFRLL").unwrap();
    println!("{:?} code {}", seat4, seat_id(&seat4));

    println!("let's grab the file and look at them");
    let seats = utils::read_file::<Seat>("./input/day5-1.txt");
    // find highest
    let mut max = 0;
    let mut seatid = 0;
    for seat in seats {
        seatid = seat_id(&(seat.unwrap()));
        if seatid > max {
            max = seatid;
        }
    }
    println!("highest seat id is {}", max);
}

