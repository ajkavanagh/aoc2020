use std::str::FromStr;
use std::fmt;

use thiserror::Error;

use crate::utils;


const MAP: &str =
   "..##.......\n\
    #...#...#..\n\
    .#....#..#.\n\
    ..#.#...#.#\n\
    .#...##..#.\n\
    ..#.##.....\n\
    .#.#.#....#\n\
    .#........#\n\
    #.##...#...\n\
    #...##....#\n\
    .#..#...#.#\n\
    ";


#[derive(Debug)]
struct Map {
    trees: Vec<Vec<bool>>,
    width: usize,
    height: usize,
}


#[derive(Debug)]
struct Delta {
    right: usize,
    down: usize,
}

impl Delta {
    fn new(right: usize, down: usize) -> Self {
        Self { right, down }
    }
}


#[derive(Debug)]
struct Coord {
    x: usize,
    y: usize,
}


impl Coord {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn go(&mut self, delta: &Delta) {
        self.x += delta.right;
        self.y += delta.down;
    }
}


impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        let header = format!("{}\n", "-".repeat(self.width + 2));
        let lines = self.trees
            .iter()
            .map(|line| {
                format!("|{}|\n", line.iter().map(|&b| {
                    if b { "#" } else { "." }}).collect::<String>())})
            .collect::<String>();

        write!(f, "{}{}{}", header, lines, header)
    }
}


impl Map {
    fn is_tree(&self, right: usize, down: usize) -> bool {
        let r = right % self.width;
        if down > self.height {
            false
        } else {
            self.trees.as_slice()[down].as_slice()[r]
        }
    }
}


#[derive(Error, Debug, Clone)]
pub enum MapError {
    #[error("corrupted map line")]
    DecodeError(String),
    #[error("not a square")]
    NotSquareError(String),
    #[error("unknown map error")]
    Unknown,
}


fn parse_line(line: &str) -> Result<Vec<bool>, MapError> {
    Ok(line.chars()
        .map(|c| c == '#')
        .collect::<Vec<_>>())
}


//fn parse_lines<I>(lines: I) -> Result<Vec<Vec<bool>>, MapError>
fn parse_lines<I>(lines: I) -> Result<Map, MapError>
    where I: IntoIterator,
          I::Item: AsRef<str>,
{
    let trees = lines
        .into_iter()
        .map(|s| parse_line(s.as_ref()).unwrap())  // we ought to do some error handling here!
        .collect::<Vec<_>>();

    let height = trees.len();
    let width = trees.first().unwrap().len();

    Ok(Map {trees, height, width})
}


fn readfile_to_map(file_name: &str) -> Result<Map, MapError>
{
    let trees = std::fs::read_to_string(file_name)
        .expect("file not found!")
        .lines()
        .map(|s| parse_line(s).unwrap())
        .collect::<Vec<_>>();

    let height = trees.len();
    let width = trees.first().unwrap().len();

    Ok(Map {trees, height, width})
}


fn count_trees(map: &Map, delta: &Delta) -> u32 {
    let mut at = Coord::new(0,0);
    let mut count: u32 = 0;
    loop {
        if map.is_tree(at.x, at.y) {
            count += 1;
        }
        at.go(&delta);
        if at.y >= map.height {
            break;
        }
    }
    count
}


pub fn day3_1() {
    println!("Day 3_1.");
    let map = parse_lines(MAP.lines()).unwrap();
    println!("{}", map);
    println!("Attempt the 3 right, 1 down thing...");

    let delta = Delta::new(3, 1);
    println!("found {} trees", count_trees(&map, &delta));

    // okay, now try the version on disk
    let rmap = readfile_to_map("./input/day3-1.txt").unwrap();
    println!("\n{}\n", rmap);
    println!("found {} trees for the on disk trees.", count_trees(&rmap, &delta));


}
