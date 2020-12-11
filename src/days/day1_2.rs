// Day 1 Part 2 - Expense report
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

//Using the above example again, the three entries that sum to 2020 are 979, 366, and 675.
//Multiplying them together produces the answer, 241861950.

//In your expense report, what is the product of the three entries that sum to 2020?

use std::str::FromStr;
use std::num::ParseIntError;

const NUMBERS: [u32; 6] = [1721, 979, 366, 299, 675, 1456];


fn find_triple(numbers: &[u32]) -> Result<(u32, u32, u32), String> {
    for n1 in 0..(numbers.len() - 2) {
        for n2 in n1..(numbers.len() - 1)  {
            for n3 in n2..numbers.len() {
                let v1 = numbers[n1];
                let v2 = numbers[n2];
                let v3 = numbers[n3];
                if v1 + v2 + v3== 2020 {
                    return Ok((v1, v2, v3));
                }
            }
        }
    }
    Err("Couldn't find a triple that adds to 2020".to_string())
}


fn read_file<T: FromStr>(file_name: &str) -> Vec<Result<T, <T as FromStr>::Err>> {
    std::fs::read_to_string(file_name)
        .expect("file not found!")
        .lines()
        .map(|x| x.parse())
        .collect()
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

pub fn day1_2() {
    println!("First let's just do the test:");
    match find_triple(&NUMBERS) {
        Ok((v1,v2,v3)) => println!("The numbers are {0} * {1} * {2} = {3}", v1, v2, v3, v1 * v2 * v3),
        Err(s) => println!("{0}", s),
    }

    println!("Now let's read the expenses file and then find in tht file:");
    let parsed_numbers = read_file::<u32>("./input/day1-1.txt");
    let numbers = extract_numbers(&parsed_numbers);
    match find_triple(&numbers) {
        Ok((v1,v2,v3)) => println!("The numbers are {0} * {1} * {2} = {3}", v1, v2, v3, v1 * v2 * v3),
        Err(s) => println!("{0}", s),
    }
}
